// ⚡ Robot RPC clients for the go2 model.
// Hand-verified against tmp_sdk2/include/unitree/robot/go2/* (unitree_sdk2).
// Service name / api_id constants come from the *_api.hpp headers; each method
// mirrors the C++ *_client.hpp signature (payload JSON keys = C++ param names).
//
//   let client = robot_api::go2::sport::SportClient::new()?;
//   let resp = client.move_to(0.3, 0.0, 0.0).await?;   // {"vx":..,"vy":..,"vyaw":..}

pub mod sport {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "sport";
    pub const API_VERSION: &str = "1.0.0.1";
    pub const ROBOT_SPORT_API_ID_DAMP: i32 = 1001;
    pub const ROBOT_SPORT_API_ID_BALANCESTAND: i32 = 1002;
    pub const ROBOT_SPORT_API_ID_STOPMOVE: i32 = 1003;
    pub const ROBOT_SPORT_API_ID_STANDUP: i32 = 1004;
    pub const ROBOT_SPORT_API_ID_STANDDOWN: i32 = 1005;
    pub const ROBOT_SPORT_API_ID_RECOVERYSTAND: i32 = 1006;
    pub const ROBOT_SPORT_API_ID_EULER: i32 = 1007;
    pub const ROBOT_SPORT_API_ID_MOVE: i32 = 1008;
    pub const ROBOT_SPORT_API_ID_SIT: i32 = 1009;
    pub const ROBOT_SPORT_API_ID_RISESIT: i32 = 1010;
    pub const ROBOT_SPORT_API_ID_SPEEDLEVEL: i32 = 1015;
    pub const ROBOT_SPORT_API_ID_HELLO: i32 = 1016;
    pub const ROBOT_SPORT_API_ID_STRETCH: i32 = 1017;
    pub const ROBOT_SPORT_API_ID_CONTENT: i32 = 1020;
    pub const ROBOT_SPORT_API_ID_DANCE1: i32 = 1022;
    pub const ROBOT_SPORT_API_ID_DANCE2: i32 = 1023;
    pub const ROBOT_SPORT_API_ID_SWITCHJOYSTICK: i32 = 1027;
    pub const ROBOT_SPORT_API_ID_POSE: i32 = 1028;
    pub const ROBOT_SPORT_API_ID_SCRAPE: i32 = 1029;
    pub const ROBOT_SPORT_API_ID_FRONTFLIP: i32 = 1030;
    pub const ROBOT_SPORT_API_ID_FRONTJUMP: i32 = 1031;
    pub const ROBOT_SPORT_API_ID_FRONTPOUNCE: i32 = 1032;
    pub const ROBOT_SPORT_API_ID_HEART: i32 = 1036;
    pub const ROBOT_SPORT_API_ID_STATICWALK: i32 = 1061;
    pub const ROBOT_SPORT_API_ID_TROTRUN: i32 = 1062;
    pub const ROBOT_SPORT_API_ID_ECONOMICGAIT: i32 = 1063;
    pub const ROBOT_SPORT_API_ID_LEFTFLIP: i32 = 2041;
    pub const ROBOT_SPORT_API_ID_BACKFLIP: i32 = 2043;
    pub const ROBOT_SPORT_API_ID_HANDSTAND: i32 = 2044;
    pub const ROBOT_SPORT_API_ID_FREEWALK: i32 = 2045;
    pub const ROBOT_SPORT_API_ID_FREEBOUND: i32 = 2046;
    pub const ROBOT_SPORT_API_ID_FREEJUMP: i32 = 2047;
    pub const ROBOT_SPORT_API_ID_FREEAVOID: i32 = 2048;
    pub const ROBOT_SPORT_API_ID_CLASSICWALK: i32 = 2049;
    pub const ROBOT_SPORT_API_ID_WALKUPRIGHT: i32 = 2050;
    pub const ROBOT_SPORT_API_ID_CROSSSTEP: i32 = 2051;
    pub const ROBOT_SPORT_API_ID_AUTORECOVERY_SET: i32 = 2054;
    pub const ROBOT_SPORT_API_ID_AUTORECOVERY_GET: i32 = 2055;
    pub const ROBOT_SPORT_API_ID_SWITCHAVOIDMODE: i32 = 2058;

    /// Rust mirror of the C++ `unitree::robot::go2::SportClient`.
    pub struct SportClient {
        client: RpcClient,
    }

    impl SportClient {
        /// Connects to the `sport` service and registers all its API ids.
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_SPORT_API_ID_DAMP);
            client.register_api(ROBOT_SPORT_API_ID_BALANCESTAND);
            client.register_api(ROBOT_SPORT_API_ID_STOPMOVE);
            client.register_api(ROBOT_SPORT_API_ID_STANDUP);
            client.register_api(ROBOT_SPORT_API_ID_STANDDOWN);
            client.register_api(ROBOT_SPORT_API_ID_RECOVERYSTAND);
            client.register_api(ROBOT_SPORT_API_ID_EULER);
            client.register_api(ROBOT_SPORT_API_ID_MOVE);
            client.register_api(ROBOT_SPORT_API_ID_SIT);
            client.register_api(ROBOT_SPORT_API_ID_RISESIT);
            client.register_api(ROBOT_SPORT_API_ID_SPEEDLEVEL);
            client.register_api(ROBOT_SPORT_API_ID_HELLO);
            client.register_api(ROBOT_SPORT_API_ID_STRETCH);
            client.register_api(ROBOT_SPORT_API_ID_CONTENT);
            client.register_api(ROBOT_SPORT_API_ID_DANCE1);
            client.register_api(ROBOT_SPORT_API_ID_DANCE2);
            client.register_api(ROBOT_SPORT_API_ID_SWITCHJOYSTICK);
            client.register_api(ROBOT_SPORT_API_ID_POSE);
            client.register_api(ROBOT_SPORT_API_ID_SCRAPE);
            client.register_api(ROBOT_SPORT_API_ID_FRONTFLIP);
            client.register_api(ROBOT_SPORT_API_ID_FRONTJUMP);
            client.register_api(ROBOT_SPORT_API_ID_FRONTPOUNCE);
            client.register_api(ROBOT_SPORT_API_ID_HEART);
            client.register_api(ROBOT_SPORT_API_ID_STATICWALK);
            client.register_api(ROBOT_SPORT_API_ID_TROTRUN);
            client.register_api(ROBOT_SPORT_API_ID_ECONOMICGAIT);
            client.register_api(ROBOT_SPORT_API_ID_LEFTFLIP);
            client.register_api(ROBOT_SPORT_API_ID_BACKFLIP);
            client.register_api(ROBOT_SPORT_API_ID_HANDSTAND);
            client.register_api(ROBOT_SPORT_API_ID_FREEWALK);
            client.register_api(ROBOT_SPORT_API_ID_FREEBOUND);
            client.register_api(ROBOT_SPORT_API_ID_FREEJUMP);
            client.register_api(ROBOT_SPORT_API_ID_FREEAVOID);
            client.register_api(ROBOT_SPORT_API_ID_CLASSICWALK);
            client.register_api(ROBOT_SPORT_API_ID_WALKUPRIGHT);
            client.register_api(ROBOT_SPORT_API_ID_CROSSSTEP);
            client.register_api(ROBOT_SPORT_API_ID_AUTORECOVERY_SET);
            client.register_api(ROBOT_SPORT_API_ID_AUTORECOVERY_GET);
            client.register_api(ROBOT_SPORT_API_ID_SWITCHAVOIDMODE);
            Ok(Self { client })
        }

        /// api 1001 `Damp`.
        pub async fn damp(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_DAMP, "{}".into()).await
        }
        /// api 1002 `BalanceStand`.
        pub async fn balance_stand(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_BALANCESTAND, "{}".into()).await
        }
        /// api 1003 `StopMove`.
        pub async fn stop_move(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_STOPMOVE, "{}".into()).await
        }
        /// api 1004 `StandUp`.
        pub async fn stand_up(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_STANDUP, "{}".into()).await
        }
        /// api 1005 `StandDown`.
        pub async fn stand_down(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_STANDDOWN, "{}".into()).await
        }
        /// api 1006 `RecoveryStand`.
        pub async fn recovery_stand(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_RECOVERYSTAND, "{}".into()).await
        }
        /// api 1007 `Euler` -> {"roll":..,"pitch":..,"yaw":..}
        pub async fn euler(&self, roll: f32, pitch: f32, yaw: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"roll": roll, "pitch": pitch, "yaw": yaw});
            self.client.call(ROBOT_SPORT_API_ID_EULER, body.to_string()).await
        }
        /// api 1008 `Move` -> {"vx":..,"vy":..,"vyaw":..}
        pub async fn move_to(&self, vx: f32, vy: f32, vyaw: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"vx": vx, "vy": vy, "vyaw": vyaw});
            self.client.call(ROBOT_SPORT_API_ID_MOVE, body.to_string()).await
        }
        /// api 1009 `Sit`.
        pub async fn sit(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_SIT, "{}".into()).await
        }
        /// api 1010 `RiseSit`.
        pub async fn rise_sit(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_RISESIT, "{}".into()).await
        }
        /// api 1015 `SpeedLevel` -> {"level":..}
        pub async fn speed_level(&self, level: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"level": level});
            self.client.call(ROBOT_SPORT_API_ID_SPEEDLEVEL, body.to_string()).await
        }
        /// api 1016 `Hello`.
        pub async fn hello(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_HELLO, "{}".into()).await
        }
        /// api 1017 `Stretch`.
        pub async fn stretch(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_STRETCH, "{}".into()).await
        }
        /// api 1027 `SwitchJoystick` -> {"flag":..}
        pub async fn switch_joystick(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_SWITCHJOYSTICK, body.to_string()).await
        }
        /// api 1020 `Content`.
        pub async fn content(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_CONTENT, "{}".into()).await
        }
        /// api 1036 `Heart`.
        pub async fn heart(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_HEART, "{}".into()).await
        }
        /// api 1028 `Pose` -> {"flag":..}
        pub async fn pose(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_POSE, body.to_string()).await
        }
        /// api 1029 `Scrape`.
        pub async fn scrape(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_SCRAPE, "{}".into()).await
        }
        /// api 1030 `FrontFlip`.
        pub async fn front_flip(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_FRONTFLIP, "{}".into()).await
        }
        /// api 1031 `FrontJump`.
        pub async fn front_jump(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_FRONTJUMP, "{}".into()).await
        }
        /// api 1032 `FrontPounce`.
        pub async fn front_pounce(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_FRONTPOUNCE, "{}".into()).await
        }
        /// api 1022 `Dance1`.
        pub async fn dance1(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_DANCE1, "{}".into()).await
        }
        /// api 1023 `Dance2`.
        pub async fn dance2(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_DANCE2, "{}".into()).await
        }
        /// api 2041 `LeftFlip`.
        pub async fn left_flip(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_LEFTFLIP, "{}".into()).await
        }
        /// api 2043 `BackFlip`.
        pub async fn back_flip(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_BACKFLIP, "{}".into()).await
        }
        /// api 2044 `HandStand` -> {"flag":..}
        pub async fn hand_stand(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_HANDSTAND, body.to_string()).await
        }
        /// api 2045 `FreeWalk`.
        pub async fn free_walk(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_FREEWALK, "{}".into()).await
        }
        /// api 2046 `FreeBound` -> {"flag":..}
        pub async fn free_bound(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_FREEBOUND, body.to_string()).await
        }
        /// api 2047 `FreeJump` -> {"flag":..}
        pub async fn free_jump(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_FREEJUMP, body.to_string()).await
        }
        /// api 2048 `FreeAvoid` -> {"flag":..}
        pub async fn free_avoid(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_FREEAVOID, body.to_string()).await
        }
        /// api 2049 `ClassicWalk` -> {"flag":..}
        pub async fn classic_walk(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_CLASSICWALK, body.to_string()).await
        }
        /// api 2050 `WalkUpright` -> {"flag":..}
        pub async fn walk_upright(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_WALKUPRIGHT, body.to_string()).await
        }
        /// api 2051 `CrossStep` -> {"flag":..}
        pub async fn cross_step(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_CROSSSTEP, body.to_string()).await
        }
        /// api 2054 `AutoRecoverSet` -> {"flag":..}
        pub async fn auto_recover_set(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_AUTORECOVERY_SET, body.to_string()).await
        }
        /// api 2055 `AutoRecoverGet` (C++ output param `flag` is returned in the response data).
        pub async fn auto_recover_get(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_AUTORECOVERY_GET, "{}".into()).await
        }
        /// api 1061 `StaticWalk`.
        pub async fn static_walk(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_STATICWALK, "{}".into()).await
        }
        /// api 1062 `TrotRun`.
        pub async fn trot_run(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_TROTRUN, "{}".into()).await
        }
        /// api 1063 `EconomicGait`.
        pub async fn economic_gait(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_ECONOMICGAIT, "{}".into()).await
        }
        /// api 2058 `SwitchAvoidMode`.
        pub async fn switch_avoid_mode(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_SWITCHAVOIDMODE, "{}".into()).await
        }
    }
}

pub mod config {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "config";
    pub const API_VERSION: &str = "1.0.0.1";
    pub const CONFIG_API_ID_SET: i32 = 1001;
    pub const CONFIG_API_ID_GET: i32 = 1002;
    pub const CONFIG_API_ID_DEL: i32 = 1003;
    pub const CONFIG_API_ID_META: i32 = 1004;

    /// Rust mirror of the C++ `unitree::robot::go2::ConfigClient`.
    pub struct ConfigClient {
        client: RpcClient,
    }

    impl ConfigClient {
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(CONFIG_API_ID_SET);
            client.register_api(CONFIG_API_ID_GET);
            client.register_api(CONFIG_API_ID_DEL);
            client.register_api(CONFIG_API_ID_META);
            Ok(Self { client })
        }
        /// api 1001 `Set` -> {"name":..,"content":..}
        pub async fn set(&self, name: &str, content: &str) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"name": name, "content": content});
            self.client.call(CONFIG_API_ID_SET, body.to_string()).await
        }
        /// api 1002 `Get` -> {"name":..} (the value is returned in the response data)
        pub async fn get(&self, name: &str) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"name": name});
            self.client.call(CONFIG_API_ID_GET, body.to_string()).await
        }
        /// api 1003 `Del` -> {"name":..}
        pub async fn del(&self, name: &str) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"name": name});
            self.client.call(CONFIG_API_ID_DEL, body.to_string()).await
        }
        /// api 1004 `Meta` -> {"name":..} (the metadata is returned in the response data)
        pub async fn meta(&self, name: &str) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"name": name});
            self.client.call(CONFIG_API_ID_META, body.to_string()).await
        }
    }
}

pub mod robot_state {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "robot_state";
    pub const API_VERSION: &str = "1.0.0.1";
    pub const ROBOT_STATE_API_ID_SERVICE_SWITCH: i32 = 1001;
    pub const ROBOT_STATE_API_ID_SET_REPORT_FREQ: i32 = 1002;
    pub const ROBOT_STATE_API_ID_SERVICE_LIST: i32 = 1003;

    /// Rust mirror of the C++ `unitree::robot::go2::RobotStateClient`.
    pub struct RobotStateClient {
        client: RpcClient,
    }

    impl RobotStateClient {
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_STATE_API_ID_SERVICE_SWITCH);
            client.register_api(ROBOT_STATE_API_ID_SET_REPORT_FREQ);
            client.register_api(ROBOT_STATE_API_ID_SERVICE_LIST);
            Ok(Self { client })
        }
        /// api 1003 `ServiceList` (the list is returned in the response data).
        pub async fn service_list(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_STATE_API_ID_SERVICE_LIST, "{}".into()).await
        }
        /// api 1001 `ServiceSwitch` -> {"name":..,"swit":..}
        pub async fn service_switch(&self, name: &str, swit: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"name": name, "swit": swit});
            self.client.call(ROBOT_STATE_API_ID_SERVICE_SWITCH, body.to_string()).await
        }
        /// api 1002 `SetReportFreq` -> {"interval":..,"duration":..}
        pub async fn set_report_freq(&self, interval: i32, duration: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"interval": interval, "duration": duration});
            self.client.call(ROBOT_STATE_API_ID_SET_REPORT_FREQ, body.to_string()).await
        }
    }
}

pub mod video {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "videohub";
    pub const API_VERSION: &str = "1.0.0.0";
    pub const ROBOT_VIDEO_API_ID_GETIMAGESAMPLE: i32 = 1001;

    /// Rust mirror of the C++ `unitree::robot::go2::VideoClient`.
    pub struct VideoClient {
        client: RpcClient,
    }

    impl VideoClient {
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_VIDEO_API_ID_GETIMAGESAMPLE);
            Ok(Self { client })
        }
        /// api 1001 `GetImageSample` (the image bytes are returned in the response data).
        pub async fn get_image_sample(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_VIDEO_API_ID_GETIMAGESAMPLE, "{}".into()).await
        }
    }
}

pub mod vui {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "vui";
    pub const API_VERSION: &str = "1.0.0.0";
    pub const ROBOT_VUI_API_ID_SETSWITCH: i32 = 1001;
    pub const ROBOT_VUI_API_ID_GETSWITCH: i32 = 1002;
    pub const ROBOT_VUI_API_ID_SETVOLUME: i32 = 1003;
    pub const ROBOT_VUI_API_ID_GETVOLUME: i32 = 1004;
    pub const ROBOT_VUI_API_ID_SETBRIGHTNESS: i32 = 1005;
    pub const ROBOT_VUI_API_ID_GETBRIGHTNESS: i32 = 1006;

    /// Rust mirror of the C++ `unitree::robot::go2::VuiClient`.
    pub struct VuiClient {
        client: RpcClient,
    }

    impl VuiClient {
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_VUI_API_ID_SETSWITCH);
            client.register_api(ROBOT_VUI_API_ID_GETSWITCH);
            client.register_api(ROBOT_VUI_API_ID_SETVOLUME);
            client.register_api(ROBOT_VUI_API_ID_GETVOLUME);
            client.register_api(ROBOT_VUI_API_ID_SETBRIGHTNESS);
            client.register_api(ROBOT_VUI_API_ID_GETBRIGHTNESS);
            Ok(Self { client })
        }
        /// api 1001 `SetSwitch` -> {"enable":..}
        pub async fn set_switch(&self, enable: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"enable": enable});
            self.client.call(ROBOT_VUI_API_ID_SETSWITCH, body.to_string()).await
        }
        /// api 1002 `GetSwitch` (the value is returned in the response data).
        pub async fn get_switch(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_VUI_API_ID_GETSWITCH, "{}".into()).await
        }
        /// api 1003 `SetVolume` -> {"level":..}
        pub async fn set_volume(&self, level: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"level": level});
            self.client.call(ROBOT_VUI_API_ID_SETVOLUME, body.to_string()).await
        }
        /// api 1004 `GetVolume` (the value is returned in the response data).
        pub async fn get_volume(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_VUI_API_ID_GETVOLUME, "{}".into()).await
        }
        /// api 1005 `SetBrightness` -> {"level":..}
        pub async fn set_brightness(&self, level: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"level": level});
            self.client.call(ROBOT_VUI_API_ID_SETBRIGHTNESS, body.to_string()).await
        }
        /// api 1006 `GetBrightness` (the value is returned in the response data).
        pub async fn get_brightness(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_VUI_API_ID_GETBRIGHTNESS, "{}".into()).await
        }
    }
}

pub mod utrack {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "uwbswitch";
    pub const API_VERSION: &str = "1.0.0.1";
    pub const ROBOT_API_ID_UTRACK_SWITCH_SET: i32 = 1001;
    pub const ROBOT_API_ID_UTRACK_SWITCH_GET: i32 = 1002;
    pub const ROBOT_API_ID_UTRACK_IS_TRACKING: i32 = 1003;

    /// Rust mirror of the C++ `unitree::robot::go2::UtrackClient`.
    pub struct UtrackClient {
        client: RpcClient,
    }

    impl UtrackClient {
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_API_ID_UTRACK_SWITCH_SET);
            client.register_api(ROBOT_API_ID_UTRACK_SWITCH_GET);
            client.register_api(ROBOT_API_ID_UTRACK_IS_TRACKING);
            Ok(Self { client })
        }
        /// api 1001 `SwitchSet` -> {"enable":..}
        pub async fn switch_set(&self, enable: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"enable": enable});
            self.client.call(ROBOT_API_ID_UTRACK_SWITCH_SET, body.to_string()).await
        }
        /// api 1002 `SwitchGet` (the value is returned in the response data).
        pub async fn switch_get(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_UTRACK_SWITCH_GET, "{}".into()).await
        }
        /// api 1003 `IsTracking` (the value is returned in the response data).
        pub async fn is_tracking(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_UTRACK_IS_TRACKING, "{}".into()).await
        }
    }
}

pub mod obstacles_avoid {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "obstacles_avoid";
    pub const API_VERSION: &str = "1.0.0.2";
    pub const ROBOT_API_ID_OBSTACLES_AVOID_SWITCH_SET: i32 = 1001;
    pub const ROBOT_API_ID_OBSTACLES_AVOID_SWITCH_GET: i32 = 1002;
    pub const ROBOT_API_ID_OBSTACLES_AVOID_MOVE: i32 = 1003;
    pub const ROBOT_API_ID_OBSTACLES_AVOID_USE_REMOTE_COMMAND_FROM_API: i32 = 1004;
    // The C++ client also declares MoveToAbsolutePosition / MoveToIncrementPosition,
    // but no API_ID constant exists in obstacles_avoid_api.hpp — not exposed.

    /// Rust mirror of the C++ `unitree::robot::go2::ObstaclesAvoidClient`.
    pub struct ObstaclesAvoidClient {
        client: RpcClient,
    }

    impl ObstaclesAvoidClient {
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_API_ID_OBSTACLES_AVOID_SWITCH_SET);
            client.register_api(ROBOT_API_ID_OBSTACLES_AVOID_SWITCH_GET);
            client.register_api(ROBOT_API_ID_OBSTACLES_AVOID_MOVE);
            client.register_api(ROBOT_API_ID_OBSTACLES_AVOID_USE_REMOTE_COMMAND_FROM_API);
            Ok(Self { client })
        }
        /// api 1001 `SwitchSet` -> {"enable":..}
        pub async fn switch_set(&self, enable: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"enable": enable});
            self.client.call(ROBOT_API_ID_OBSTACLES_AVOID_SWITCH_SET, body.to_string()).await
        }
        /// api 1002 `SwitchGet` (the value is returned in the response data).
        pub async fn switch_get(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_OBSTACLES_AVOID_SWITCH_GET, "{}".into()).await
        }
        /// api 1003 `Move` -> {"x":..,"y":..,"yaw":..}
        pub async fn move_to(&self, x: f32, y: f32, yaw: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"x": x, "y": y, "yaw": yaw});
            self.client.call(ROBOT_API_ID_OBSTACLES_AVOID_MOVE, body.to_string()).await
        }
        /// api 1004 `UseRemoteCommandFromApi` -> {"isRemoteCommandsFromApi":..}
        pub async fn use_remote_command_from_api(
            &self,
            is_remote_commands_from_api: bool,
        ) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"isRemoteCommandsFromApi": is_remote_commands_from_api});
            self.client
                .call(ROBOT_API_ID_OBSTACLES_AVOID_USE_REMOTE_COMMAND_FROM_API, body.to_string())
                .await
        }
    }
}
