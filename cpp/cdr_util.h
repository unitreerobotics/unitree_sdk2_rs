#pragma once
#include <vector>
#include <cstdint>
#include <org/eclipse/cyclonedds/core/cdr/basic_cdr_ser.hpp>

namespace unitree {

// DDS struct ↔ CDR bytes (with the 4-byte CDR_LE encapsulation header,
// matching the encoding of the Rust `cdr` crate)

// Serialize DDS struct → CDR bytes (including the encapsulation header)
template<typename T>
std::vector<uint8_t> dds_to_bytes(const T& msg) {
    std::vector<uint8_t> buf(65536);  // large enough for all hg types
    org::eclipse::cyclonedds::core::cdr::basic_cdr_stream s;
    s.set_buffer(buf.data(), buf.size());
    if (!write(s, msg, false)) return {};
    size_t n = s.position();
    std::vector<uint8_t> out = {0x00, 0x01, 0x00, 0x00};  // CDR_LE encapsulation header
    out.insert(out.end(), buf.begin(), buf.begin() + n);
    return out;
}

// Deserialize CDR bytes → DDS struct (skipping the 4-byte encapsulation header)
template<typename T>
T bytes_to_dds(const std::vector<uint8_t>& bytes) {
    T msg{};
    if (bytes.size() <= 4) return msg;
    org::eclipse::cyclonedds::core::cdr::basic_cdr_stream s;
    s.set_buffer(const_cast<uint8_t*>(bytes.data() + 4), bytes.size() - 4);
    read(s, msg, false);
    return msg;
}

} // namespace unitree
