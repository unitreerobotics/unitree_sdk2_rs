use std::collections::BTreeMap;
use std::env;
use std::path::{Path, PathBuf};

// msg → Rust type generation

fn primitive_rust(ty: &str) -> Option<&'static str> {
    Some(match ty {
        "uint8" => "u8",
        "uint16" => "u16",
        "uint32" => "u32",
        "uint64" => "u64",
        "int8" => "i8",
        "int16" => "i16",
        "int32" => "i32",
        "int64" => "i64",
        "float32" => "f32",
        "float64" => "f64",
        "bool" => "bool",
        "char" => "u8",
        "byte" => "i8",
        // Fully qualified to avoid clashing with a struct named `String`
        "string" => "std::string::String",
        _ => return None,
    })
}

enum Dim {
    Scalar,
    Fixed(usize),
    Variable,
}

/// Parses one "type name" line, returning (rust type, field name, needs BigArray)
fn parse_field(line: &str) -> Option<(String, String, bool)> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }
    let line = line.split('#').next().unwrap_or("").trim();
    let mut it = line.split_whitespace();
    let ty = it.next()?;
    let name = it.next()?;
    let name = name.trim();

    // Parse Type[N] / Type[] / Type
    let (base, dim) = if let Some(stripped) = ty.strip_suffix("[]") {
        (stripped.to_string(), Dim::Variable)
    } else if let Some(open) = ty.find('[') {
        let base = ty[..open].to_string();
        let inner = ty[open..].trim_start_matches('[').trim_end_matches(']');
        let n: usize = inner.parse().ok()?;
        (base, Dim::Fixed(n))
    } else {
        (ty.to_string(), Dim::Scalar)
    };

    let big = matches!(dim, Dim::Fixed(n) if n > 32);

    let rust = if let Some(p) = primitive_rust(&base) {
        match dim {
            Dim::Scalar => p.to_string(),
            Dim::Fixed(n) => format!("[{}; {n}]", p),
            Dim::Variable => format!("Vec<{p}>"),
        }
    } else {
        match dim {
            Dim::Scalar => base,
            Dim::Fixed(n) => format!("[{base}; {n}]"),
            Dim::Variable => format!("Vec<{base}>"),
        }
    };

    Some((rust, name.to_string(), big))
}

/// Generates the Rust code for one struct (indent = prefix indentation)
fn gen_struct(name: &str, lines: &[String], indent: &str) -> String {
    let mut s = format!("{indent}#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]\n{indent}pub struct {name} {{\n");
    for line in lines {
        let Some((ty, field, big)) = parse_field(line) else {
            continue;
        };
        // Escape Rust keywords
        let field_ident = if is_keyword(&field) {
            format!("r#{}", field)
        } else {
            field
        };
        let big_attr = if big {
            &format!("{indent}    #[serde(with = \"serde_big_array::BigArray\")]\n")
        } else {
            ""
        };
        s.push_str(&format!("{big_attr}{indent}    pub {field_ident}: {ty},\n"));
    }
    s.push_str(&format!("{indent}}}\n\n"));
    s
}

fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "fn" | "type"
            | "loop"
            | "match"
            | "move"
            | "ref"
            | "as"
            | "in"
            | "if"
            | "else"
            | "for"
            | "while"
            | "return"
            | "use"
            | "mod"
            | "struct"
            | "enum"
            | "trait"
            | "impl"
            | "let"
            | "mut"
            | "const"
            | "static"
            | "unsafe"
    )
}

struct Package {
    name: String,    // e.g. unitree_hg
    cpp_ns: String,  // e.g. unitree_hg::msg::dds_
    idl_dir: String, // e.g. idl/hg (SDK include path relative to unitree/)
    tag: String,     // e.g. hg (symbol prefix, avoids name clashes across packages)
    types: BTreeMap<String, Vec<String>>,
}

fn pkg_tag(package: &str) -> String {
    match package {
        "unitree_hg" => "hg".into(),
        "unitree_go" => "go".into(),
        "std_msgs" => "std".into(),
        "unitree_api" => "api".into(),
        _ => package.to_string(),
    }
}

/// msg package name → SDK header include path relative to `unitree/`
/// (unitree_hg→idl/hg, unitree_go→idl/go2, std_msgs→idl/ros2,
///  unitree_api→robot/internal/internal_idl_decl)
fn idl_dir(package: &str) -> Option<&str> {
    match package {
        "unitree_hg" => Some("idl/hg"),
        "unitree_go" => Some("idl/go2"),
        "std_msgs" => Some("idl/ros2"),
        "unitree_api" => Some("robot/internal/internal_idl_decl"),
        _ => None, // packages without an SDK IDL directory are skipped
    }
}

/// Scans msgs/ for all packages that have an SDK IDL directory
fn parse_packages(msg_root: &Path) -> Vec<Package> {
    let mut pkgs = Vec::new();
    if !msg_root.exists() {
        return pkgs;
    }
    let mut dirs: Vec<_> = std::fs::read_dir(msg_root)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("msg").is_dir())
        .collect();
    dirs.sort_by_key(|e| e.file_name());
    for e in dirs {
        let name = e.file_name().to_string_lossy().to_string();
        let Some(idl) = idl_dir(&name) else { continue };
        let msg_dir = e.path().join("msg");
        let mut types = BTreeMap::new();
        let mut entries: Vec<_> = std::fs::read_dir(&msg_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|x| x == "msg"))
            .collect();
        entries.sort_by_key(|e| e.file_name());
        for m in entries {
            let fname = m.file_name().to_string_lossy().to_string();
            let tname = fname.trim_end_matches(".msg").to_string();
            let lines = std::fs::read_to_string(m.path())
                .unwrap_or_default()
                .lines()
                .map(|s| s.to_string())
                .collect();
            types.insert(tname, lines);
        }
        pkgs.push(Package {
            name: name.clone(),
            cpp_ns: format!("{name}::msg::dds_"),
            idl_dir: idl.to_string(),
            tag: pkg_tag(&name),
            types,
        });
    }
    pkgs
}

/// Type name → snake_case (BmsState→bms_state, IMUState→imu_state)
fn snake_case(name: &str) -> String {
    let b = name.as_bytes();
    let mut out = String::new();
    for (i, &c) in b.iter().enumerate() {
        if c.is_ascii_uppercase() {
            let prev_lower = i > 0 && b[i - 1].is_ascii_lowercase();
            let next_lower = i + 1 < b.len() && b[i + 1].is_ascii_lowercase();
            if i > 0 && (prev_lower || next_lower) {
                out.push('_');
            }
            out.push((c as char).to_ascii_lowercase());
        } else {
            out.push(c as char);
        }
    }
    out
}

/// Generates gen/gen_types.rs (Rust structs + DdsType, nested modules mirroring C++ namespaces)
fn gen_rust_types(pkgs: &[Package]) -> String {
    let mut out = String::new();
    out.push_str("// ⚡ Auto-generated (msg → Rust). Regenerate by editing msgs/ or build.rs.\n\n");
    for pkg in pkgs {
        let parts: Vec<&str> = pkg.cpp_ns.split("::").collect();
        let depth = parts.len();
        for (i, part) in parts.iter().enumerate() {
            out.push_str(&format!("{}pub mod {part} {{\n", "  ".repeat(i)));
        }
        let body_indent = "  ".repeat(depth);
        for (name, lines) in &pkg.types {
            out.push_str(&gen_struct(name, lines, &body_indent));
            out.push_str(&format!(
                "{body_indent}impl crate::DdsType for {name} {{\n\
                 {body_indent}    fn dds_name() -> &'static str {{ \"{cpp_ns}::{name}_\" }}\n\
                 {body_indent}}}\n\n",
                cpp_ns = pkg.cpp_ns
            ));
        }
        for i in (0..depth).rev() {
            out.push_str(&format!("{}}}\n", "  ".repeat(i)));
        }
    }
    out
}

/// Generates gen/gen_ffi.rs (full cxx::bridge module with per-topic FFI declarations)
fn gen_rust_ffi(pkgs: &[Package]) -> String {
    let mut out = String::new();
    out.push_str(
        "// ⚡ Auto-generated (msg → Rust FFI). Regenerate by editing msgs/ or build.rs.\n\n",
    );
    out.push_str("pub struct ByteHandler {\n");
    out.push_str("    cb: Box<dyn Fn(Vec<u8>) + Send>,\n");
    out.push_str("}\n");
    out.push_str("impl ByteHandler {\n");
    out.push_str("    pub fn new(cb: impl Fn(Vec<u8>) + Send + 'static) -> Self { Self { cb: Box::new(cb) } }\n");
    out.push_str("    pub(crate) fn on_bytes(&self, bytes: Vec<u8>) { (self.cb)(bytes); }\n");
    out.push_str("}\n\n");
    out.push_str("#[cxx::bridge(namespace = \"unitree\")]\n");
    out.push_str("pub mod ffi {\n");
    out.push_str("    extern \"Rust\" {\n");
    out.push_str("        type ByteHandler;\n");
    out.push_str("        fn on_bytes(self: &ByteHandler, bytes: Vec<u8>);\n");
    out.push_str("    }\n");
    out.push_str("    unsafe extern \"C++\" {\n");
    out.push_str("        include!(\"dds_bridge.h\");\n");
    out.push_str(
        "        fn boot_dds(domain_id: i32, network_interface: &str, config_file: &str);\n",
    );
    out.push_str("        fn subscribe_any(topic: &str, type_name: &str, handler: Box<ByteHandler>) -> i32;\n");
    out.push_str("        fn unsubscribe(id: i32);\n");
    for pkg in pkgs {
        for name in pkg.types.keys() {
            let low = snake_case(name);
            out.push_str(&format!(
                "        type {tag}{name}Publisher;\n\
                 \x20       fn new_{tag}_{low}_publisher(topic: &str) -> UniquePtr<{tag}{name}Publisher>;\n\
                 \x20       fn publish_{tag}_{low}_bytes(p: &{tag}{name}Publisher, bytes: Vec<u8>);\n",
                tag = pkg.tag
            ));
        }
    }
    out.push_str("    }\n");
    out.push_str("}\n");
    out
}

/// Generates gen/gen_topic_impl.rs (topic_publish! invocations)
fn gen_rust_topic_impl(pkgs: &[Package]) -> String {
    let mut out = String::new();
    out.push_str("// ⚡ Auto-generated (msg → Rust topic_publish). Regenerate by editing msgs/ or build.rs.\n\n");
    for pkg in pkgs {
        for name in pkg.types.keys() {
            let low = snake_case(name);
            out.push_str(&format!(
                "topic_publish!(crate::gen_types::{cpp_ns}::{name}, {tag}{name}Publisher, new_{tag}_{low}_publisher, publish_{tag}_{low}_bytes);\n",
                cpp_ns = pkg.cpp_ns, tag = pkg.tag
            ));
        }
    }
    out
}

/// Generates gen/gen_topics.h (C++ per-topic declarations + make_subscriber)
fn gen_topics_h(pkgs: &[Package]) -> String {
    let mut out = String::new();
    out.push_str("// ⚡ Auto-generated (msg → C++). Regenerate by editing msgs/ or build.rs.\n#pragma once\n\n");
    for pkg in pkgs {
        for name in pkg.types.keys() {
            out.push_str(&format!(
                "#include <unitree/{idl}/{name}_.hpp>\n",
                idl = pkg.idl_dir
            ));
        }
    }
    out.push_str("\nnamespace unitree {\n\n");
    for pkg in pkgs {
        for name in pkg.types.keys() {
            let low = snake_case(name);
            out.push_str(&format!(
                "using {tag}{name}Publisher = Publisher<{cpp_ns}::{name}_>;\n\
                 std::unique_ptr<{tag}{name}Publisher> new_{tag}_{low}_publisher(rust::Str topic);\n\
                 void publish_{tag}_{low}_bytes(const {tag}{name}Publisher& pub, rust::Vec<uint8_t> bytes);\n\n",
                tag = pkg.tag, cpp_ns = pkg.cpp_ns
            ));
        }
    }
    out.push_str("std::unique_ptr<SubscriberBase> make_subscriber(\n");
    out.push_str("    const std::string& topic, const std::string& type_name,\n");
    out.push_str("    std::function<void(rust::Vec<uint8_t>)> cb);\n\n");
    out.push_str("} // namespace unitree\n");
    out
}

/// Generates gen/gen_topics.inc (C++ per-topic implementations + template instantiations)
fn gen_topics_inc(pkgs: &[Package]) -> String {
    let mut out = String::new();
    out.push_str("// ⚡ Auto-generated (msg → C++). Regenerate by editing msgs/ or build.rs.\n\n");
    for pkg in pkgs {
        for name in pkg.types.keys() {
            let low = snake_case(name);
            out.push_str(&format!(
                "template class Subscriber<{cpp_ns}::{name}_>;\n\
                 template class Publisher<{cpp_ns}::{name}_>;\n\
                 std::unique_ptr<{tag}{name}Publisher> new_{tag}_{low}_publisher(rust::Str t) {{\n\
                 \x20 return std::make_unique<{tag}{name}Publisher>(std::string(t));\n\
                 }}\n\
                 void publish_{tag}_{low}_bytes(const {tag}{name}Publisher& pub, rust::Vec<uint8_t> bytes) {{\n\
                 \x20 auto msg = bytes_to_dds<{cpp_ns}::{name}_>(to_std_vec(bytes));\n\
                 \x20 pub.write(msg);\n\
                 }}\n\n",
                cpp_ns = pkg.cpp_ns, tag = pkg.tag
            ));
        }
    }
    out.push_str("std::unique_ptr<SubscriberBase> make_subscriber(\n");
    out.push_str("    const std::string& topic, const std::string& type_name,\n");
    out.push_str("    std::function<void(rust::Vec<uint8_t>)> cb) {\n");
    for pkg in pkgs {
        for name in pkg.types.keys() {
            out.push_str(&format!(
                "  if (type_name == \"{cpp_ns}::{name}_\")\n\
                 \x20\x20 return std::make_unique<Subscriber<{cpp_ns}::{name}_>>(topic, cb);\n",
                cpp_ns = pkg.cpp_ns
            ));
        }
    }
    out.push_str("  return nullptr;\n}\n");
    out
}

fn write_if_changed(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if std::fs::read_to_string(path)
        .map(|s| s != content)
        .unwrap_or(true)
    {
        std::fs::write(path, content).unwrap();
        println!("cargo:warning=regenerated {}", path.display());
    }
}

const SDK2_DEFAULT_URL: &str = "https://github.com/unitreerobotics/unitree_sdk2.git";

/// SDK source: a local directory or a git clone
enum SdkSource {
    /// Use the local SDK directly (no clone, no build)
    Path(PathBuf),
    /// Shallow-clone to UNITREE_SDK2_DIR (default tmp_sdk2)
    Url(String),
}

/// Resolves the SDK source and destination directory. Priority: PATH > URL > default GitHub
fn sdk_cfg() -> (SdkSource, PathBuf) {
    // An empty PATH is treated as unset (allows switching back to URL mode temporarily)
    if let Ok(p) = env::var("UNITREE_SDK2_PATH") {
        let p = p.trim();
        if !p.is_empty() {
            return (SdkSource::Path(PathBuf::from(p)), PathBuf::new());
        }
    }

    // URL (explicit env or the default GitHub)
    let url = if let Ok(u) = env::var("UNITREE_SDK2_URL") {
        let u = u.trim();
        if u.is_empty() {
            panic!("UNITREE_SDK2_URL is empty; set a valid git URL, or use UNITREE_SDK2_PATH to point to a local SDK");
        }
        u.to_string()
    } else {
        SDK2_DEFAULT_URL.to_string()
    };

    let dir = env::var("UNITREE_SDK2_DIR").unwrap_or_else(|_| "tmp_sdk2".to_string());
    let dir = PathBuf::from(dir);
    let dir = if dir.is_absolute() {
        dir
    } else {
        // Land in OUT_DIR: same behavior for normal builds and cargo publish verify,
        // and zero writes to the source directory.
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR")).join(dir)
    };
    (SdkSource::Url(url), dir)
}

/// Whether the SDK directory has the prebuilt library for the target arch
/// (the repo ships prebuilt binaries; no cmake build is done)
fn sdk_ready(sdk_dir: &Path, target_arch: &str) -> bool {
    sdk_dir.join("include").is_dir()
        && sdk_dir
            .join("lib")
            .join(target_arch)
            .join("libunitree_sdk2.a")
            .is_file()
}

/// PATH mode: use the local SDK directory as-is; fail if the prebuilt library is missing
fn ensure_sdk_path(sdk_dir: &Path, target_arch: &str) {
    if !sdk_ready(sdk_dir, target_arch) {
        panic!(
            "UNITREE_SDK2_PATH does not point to a complete SDK: {}\n\
             It must contain include/ and lib/{}/libunitree_sdk2.a (the repo ships prebuilt binaries)",
            sdk_dir.display(), target_arch
        );
    }
}

/// URL mode: use the existing prebuilt SDK if present, otherwise shallow-clone
fn ensure_sdk(sdk_dir: &Path, url: &str, target_arch: &str) {
    if sdk_ready(sdk_dir, target_arch) {
        return; // complete prebuilt SDK already present
    }
    // Remove any leftover directory to avoid "destination path already exists"
    if sdk_dir.exists() {
        std::fs::remove_dir_all(sdk_dir).expect("failed to clean SDK directory");
    }
    println!("cargo:warning=cloning unitree_sdk2: {url}");
    let status = std::process::Command::new("git")
        .args(["clone", "--depth", "1", url])
        .arg(sdk_dir)
        .status()
        .unwrap_or_else(|e| panic!("failed to run git clone (git and network required): {e}"));
    if !status.success() {
        panic!("git clone failed; could not fetch unitree_sdk2. Check the network, or set UNITREE_SDK2_PATH to a local SDK.");
    }
    if !sdk_ready(sdk_dir, target_arch) {
        panic!(
            "cloned unitree_sdk2 is missing the {target_arch} prebuilt library (lib/{target_arch}/libunitree_sdk2.a).\n\
             Make sure the repo version ships binaries, or set UNITREE_SDK2_PATH to a complete SDK directory."
        );
    }
}

// Main flow

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // ── msg → Rust type generation (scan msgs/ for packages with SDK IDL) ──
    let msg_root = PathBuf::from(
        env::var("UNITREE_MSG_ROOT")
            .unwrap_or_else(|_| manifest_dir.join("msgs").to_string_lossy().to_string()),
    );
    let pkgs = parse_packages(&msg_root);
    if pkgs.is_empty() {
        panic!("no packages under msgs/ to generate (need unitree_hg / unitree_go / std_msgs with SDK IDL): {}", msg_root.display());
    }
    println!(
        "cargo:warning=generating {} packages: {:?}",
        pkgs.len(),
        pkgs.iter().map(|p| p.name.as_str()).collect::<Vec<_>>()
    );

    // Generate Rust types
    write_if_changed(
        &manifest_dir.join("gen/gen_types.rs"),
        &gen_rust_types(&pkgs),
    );
    // Generate Rust FFI + topic_publish invocations
    write_if_changed(&manifest_dir.join("gen/gen_ffi.rs"), &gen_rust_ffi(&pkgs));
    write_if_changed(
        &manifest_dir.join("gen/gen_topic_impl.rs"),
        &gen_rust_topic_impl(&pkgs),
    );
    // Generate C++ per-topic glue
    write_if_changed(&manifest_dir.join("gen/gen_topics.h"), &gen_topics_h(&pkgs));
    write_if_changed(
        &manifest_dir.join("gen/gen_topics.inc"),
        &gen_topics_inc(&pkgs),
    );

    println!("cargo:rerun-if-changed={}", msg_root.display());

    // ── SDK build (skip C++ on macOS / docs.rs for type-check only) ──
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // docs.rs sandbox has no network/SDK (always sets DOCS_RS=1); skip C++ compilation
    let is_docs_rs = env::var("DOCS_RS").map(|v| v == "1").unwrap_or(false);
    if target_os == "macos" || is_docs_rs {
        let reason = if is_docs_rs {
            "docs.rs environment (no network/SDK), type-check only"
        } else {
            "macOS: type-check only; build in Docker/Linux for real devices"
        };
        println!("cargo:warning=skipping C++ compilation ({reason})");
    } else {
        let (source, sdk_dir) = sdk_cfg();
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
        let sdk_arch = match target_arch.as_str() {
            "aarch64" | "x86_64" => target_arch,
            o => panic!("unsupported target architecture: {o}"),
        };
        // PATH → use the local directory; URL → shallow-clone when the library is missing
        let sdk_dir = match &source {
            SdkSource::Path(p) => {
                ensure_sdk_path(p, &sdk_arch);
                println!(
                    "cargo:warning=using unitree_sdk2 (UNITREE_SDK2_PATH): {}",
                    p.display()
                );
                p.clone()
            }
            SdkSource::Url(url) => {
                ensure_sdk(&sdk_dir, url, &sdk_arch);
                println!(
                    "cargo:warning=using unitree_sdk2 (UNITREE_SDK2_URL, cloned or reused): {}",
                    sdk_dir.display()
                );
                sdk_dir
            }
        };
        // Record the actual SDK directory to target/<profile>/sdk_dir.txt for build.sh
        if let Some(profile_dir) = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"))
            .ancestors()
            .nth(3)
        {
            let _ = std::fs::write(
                profile_dir.join("sdk_dir.txt"),
                format!("{}\n", sdk_dir.display()),
            );
        }
        let thirdparty = sdk_dir.join("thirdparty");
        let sdk_lib_dir = sdk_dir.join("lib").join(&sdk_arch);
        let thirdparty_lib_dir = thirdparty.join("lib").join(&sdk_arch);
        assert!(
            sdk_lib_dir.join("libunitree_sdk2.a").exists(),
            "libunitree_sdk2.a not found"
        );

        cxx_build::bridges(["src/lib.rs", "gen/gen_ffi.rs"])
            .file("cpp/dds_bridge.cpp")
            .file("cpp/rpc_bridge.cpp")
            .include("cpp")
            .include(sdk_dir.join("include"))
            .include(thirdparty.join("include"))
            .include(thirdparty.join("include/ddscxx"))
            .include(thirdparty.join("include/iceoryx/v2.0.2"))
            .std("c++17")
            .compile("unitree_sdk2_rs_bridge");

        println!("cargo:rustc-link-search=native={}", sdk_lib_dir.display());
        println!("cargo:rustc-link-lib=static=unitree_sdk2");
        println!(
            "cargo:rustc-link-search=native={}",
            thirdparty_lib_dir.display()
        );
        println!("cargo:rustc-link-lib=dylib=ddsc");
        println!("cargo:rustc-link-lib=dylib=ddscxx");
        println!("cargo:rustc-link-arg=-Wl,--disable-new-dtags");
        println!(
            "cargo:rustc-link-arg=-Wl,-rpath,{}",
            thirdparty_lib_dir.display()
        );
        println!("cargo:rustc-link-lib=pthread");
    }

    println!("cargo:rerun-if-env-changed=UNITREE_SDK2_PATH");
    println!("cargo:rerun-if-env-changed=UNITREE_SDK2_URL");
    println!("cargo:rerun-if-env-changed=UNITREE_SDK2_DIR");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=gen/gen_types.rs");
    println!("cargo:rerun-if-changed=gen/gen_ffi.rs");
    println!("cargo:rerun-if-changed=gen/gen_topic_impl.rs");
    println!("cargo:rerun-if-changed=gen/gen_topics.h");
    println!("cargo:rerun-if-changed=gen/gen_topics.inc");
    println!("cargo:rerun-if-changed=cpp/dds_bridge.h");
    println!("cargo:rerun-if-changed=cpp/dds_bridge.cpp");
    println!("cargo:rerun-if-changed=cpp/cdr_util.h");
    println!("cargo:rerun-if-changed=cpp/rpc_bridge.h");
    println!("cargo:rerun-if-changed=cpp/rpc_bridge.cpp");
}
