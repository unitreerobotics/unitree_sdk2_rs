// ⚡ Robot RPC clients for the b2 model.
// Hand-verified against tmp_sdk2/include/unitree/robot/b2/* (unitree_sdk2).
// Service name / api_id constants come from the *_api.hpp headers; each method
// mirrors the C++ *_client.hpp signature (payload JSON keys = C++ param names).
//
//   let client = robot_api::b2::sport::SportClient::new()?;
//   let resp = client.move_to(0.3, 0.0, 0.0).await?;   // {"vx":..,"vy":..,"vyaw":..}

pub mod back_video {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "back_videohub";
    pub const API_VERSION: &str = "1.0.0.0";
    pub const ROBOT_BACK_VIDEO_API_ID_GETIMAGESAMPLE: i32 = 1001;

    /// Rust mirror of the C++ `unitree::robot::b2::BackVideoClient`.
    pub struct BackVideoClient {
        client: RpcClient,
    }

    impl BackVideoClient {
        /// Connects to the `back_videohub` service and registers all its API ids.
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_BACK_VIDEO_API_ID_GETIMAGESAMPLE);
            Ok(Self { client })
        }
        /// api 1001 `GetImageSample` (the image bytes are returned in the response data).
        pub async fn get_image_sample(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_BACK_VIDEO_API_ID_GETIMAGESAMPLE, "{}".into()).await
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
    // The C++ client also declares SubscribeChangeStatus (a DDS subscription
    // helper, not an RPC call) and a second Meta(name, ConfigMeta&) overload —
    // both map to the same RPC as the methods below and are not exposed.

    /// Rust mirror of the C++ `unitree::robot::b2::ConfigClient`.
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

pub mod front_video {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "front_videohub";
    pub const API_VERSION: &str = "1.0.0.0";
    pub const ROBOT_FRONT_VIDEO_API_ID_GETIMAGESAMPLE: i32 = 1001;

    /// Rust mirror of the C++ `unitree::robot::b2::FrontVideoClient`.
    pub struct FrontVideoClient {
        client: RpcClient,
    }

    impl FrontVideoClient {
        /// Connects to the `front_videohub` service and registers all its API ids.
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_FRONT_VIDEO_API_ID_GETIMAGESAMPLE);
            Ok(Self { client })
        }
        /// api 1001 `GetImageSample` (the image bytes are returned in the response data).
        pub async fn get_image_sample(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_FRONT_VIDEO_API_ID_GETIMAGESAMPLE, "{}".into()).await
        }
    }
}

pub mod motion_switcher {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "motion_switcher";
    pub const API_VERSION: &str = "1.0.0.1";
    pub const MOTION_SWITCHER_API_ID_CHECK_MODE: i32 = 1001;
    pub const MOTION_SWITCHER_API_ID_SELECT_MODE: i32 = 1002;
    pub const MOTION_SWITCHER_API_ID_RELEASE_MODE: i32 = 1003;
    pub const MOTION_SWITCHER_API_ID_SET_SILENT: i32 = 1004;
    pub const MOTION_SWITCHER_API_ID_GET_SILENT: i32 = 1005;

    /// Rust mirror of the C++ `unitree::robot::b2::MotionSwitcherClient`.
    pub struct MotionSwitcherClient {
        client: RpcClient,
    }

    impl MotionSwitcherClient {
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(MOTION_SWITCHER_API_ID_CHECK_MODE);
            client.register_api(MOTION_SWITCHER_API_ID_SELECT_MODE);
            client.register_api(MOTION_SWITCHER_API_ID_RELEASE_MODE);
            client.register_api(MOTION_SWITCHER_API_ID_SET_SILENT);
            client.register_api(MOTION_SWITCHER_API_ID_GET_SILENT);
            Ok(Self { client })
        }
        /// api 1001 `CheckMode` (C++ output params `form` and `name` are returned in the response data).
        pub async fn check_mode(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(MOTION_SWITCHER_API_ID_CHECK_MODE, "{}".into()).await
        }
        /// api 1002 `SelectMode` -> {"nameOrAlias":..}
        pub async fn select_mode(&self, name_or_alias: &str) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"nameOrAlias": name_or_alias});
            self.client.call(MOTION_SWITCHER_API_ID_SELECT_MODE, body.to_string()).await
        }
        /// api 1003 `ReleaseMode`.
        pub async fn release_mode(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(MOTION_SWITCHER_API_ID_RELEASE_MODE, "{}".into()).await
        }
        /// api 1004 `SetSilent` -> {"silent":..}
        pub async fn set_silent(&self, silent: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"silent": silent});
            self.client.call(MOTION_SWITCHER_API_ID_SET_SILENT, body.to_string()).await
        }
        /// api 1005 `GetSilent` (C++ output param `silent` is returned in the response data).
        pub async fn get_silent(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(MOTION_SWITCHER_API_ID_GET_SILENT, "{}".into()).await
        }
    }
}

pub mod robot_state {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "robot_state";
    pub const API_VERSION: &str = "1.0.0.2";
    pub const ROBOT_STATE_API_ID_SERVICE_SWITCH: i32 = 1001;
    pub const ROBOT_STATE_API_ID_SET_REPORT_FREQ: i32 = 1002;
    pub const ROBOT_STATE_API_ID_SERVICE_LIST: i32 = 1003;
    pub const ROBOT_STATE_API_ID_LOWPOWER_SWITCH: i32 = 1004;
    pub const ROBOT_STATE_API_ID_LOWPOWER_STATUS: i32 = 1005;
    pub const ROBOT_STATE_API_ID_GET_PKG_VERSION: i32 = 1006;

    /// Rust mirror of the C++ `unitree::robot::b2::RobotStateClient`.
    pub struct RobotStateClient {
        client: RpcClient,
    }

    impl RobotStateClient {
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_STATE_API_ID_SERVICE_SWITCH);
            client.register_api(ROBOT_STATE_API_ID_SET_REPORT_FREQ);
            client.register_api(ROBOT_STATE_API_ID_SERVICE_LIST);
            client.register_api(ROBOT_STATE_API_ID_LOWPOWER_SWITCH);
            client.register_api(ROBOT_STATE_API_ID_LOWPOWER_STATUS);
            client.register_api(ROBOT_STATE_API_ID_GET_PKG_VERSION);
            Ok(Self { client })
        }
        /// api 1003 `ServiceList` (the list is returned in the response data).
        pub async fn service_list(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_STATE_API_ID_SERVICE_LIST, "{}".into()).await
        }
        /// api 1001 `ServiceSwitch` -> {"name":..,"swit":..} (C++ output param `status` is returned in the response data)
        pub async fn service_switch(&self, name: &str, swit: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"name": name, "swit": swit});
            self.client.call(ROBOT_STATE_API_ID_SERVICE_SWITCH, body.to_string()).await
        }
        /// api 1002 `SetReportFreq` -> {"interval":..,"duration":..}
        pub async fn set_report_freq(&self, interval: i32, duration: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"interval": interval, "duration": duration});
            self.client.call(ROBOT_STATE_API_ID_SET_REPORT_FREQ, body.to_string()).await
        }
        /// api 1004 `LowPowerSwitch` -> {"swit":..}
        pub async fn low_power_switch(&self, swit: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"swit": swit});
            self.client.call(ROBOT_STATE_API_ID_LOWPOWER_SWITCH, body.to_string()).await
        }
        /// api 1005 `LowPowerStatus` (C++ output param `status` is returned in the response data).
        pub async fn low_power_status(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_STATE_API_ID_LOWPOWER_STATUS, "{}".into()).await
        }
        /// api 1006 `GetPkgVersion` (C++ output params `packageVersion` and `moduleVersionMap` are returned in the response data).
        pub async fn get_pkg_version(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_STATE_API_ID_GET_PKG_VERSION, "{}".into()).await
        }
    }
}

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
    pub const ROBOT_SPORT_API_ID_MOVE: i32 = 1008;
    pub const ROBOT_SPORT_API_ID_SWITCHGAIT: i32 = 1011;
    pub const ROBOT_SPORT_API_ID_BODYHEIGHT: i32 = 1013;
    pub const ROBOT_SPORT_API_ID_SPEEDLEVEL: i32 = 1015;
    pub const ROBOT_SPORT_API_ID_TRAJECTORYFOLLOW: i32 = 1018;
    pub const ROBOT_SPORT_API_ID_CONTINUOUSGAIT: i32 = 1019;
    pub const ROBOT_SPORT_API_ID_MOVETOPOS: i32 = 1036;
    pub const ROBOT_SPORT_API_ID_SWITCHMOVEMODE: i32 = 1038;
    pub const ROBOT_SPORT_API_ID_VISIONWALK: i32 = 1101;
    pub const ROBOT_SPORT_API_ID_HANDSTAND: i32 = 1039;
    pub const ROBOT_SPORT_API_ID_AUTORECOVERY_SET: i32 = 1040;
    pub const ROBOT_SPORT_API_ID_FREEWALK: i32 = 1045;
    pub const ROBOT_SPORT_API_ID_CLASSICWALK: i32 = 1049;
    pub const ROBOT_SPORT_API_ID_FASTWALK: i32 = 1050;
    pub const ROBOT_SPORT_API_ID_FREEEULER: i32 = 1051;
    pub const ROBOT_SPORT_API_ID_FREE_HEIGHT: i32 = 1052;
    pub const ROBOT_SPORT_API_ID_GAIT_HEIGHT: i32 = 1053;
    // TrajectoryFollow (1018) is implemented below with a Rust `PathPoint` struct
    // mirroring the C++ `unitree::robot::b2::stPathPoint`.

    /// Mirror of the C++ `unitree::robot::b2::stPathPoint`, used by `TrajectoryFollow`.
    #[derive(Debug, Clone, Copy, Default)]
    pub struct PathPoint {
        pub time_from_start: f32,
        pub x: f32,
        pub y: f32,
        pub yaw: f32,
        pub vx: f32,
        pub vy: f32,
        pub vyaw: f32,
    }

    /// Rust mirror of the C++ `unitree::robot::b2::SportClient`.
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
            client.register_api(ROBOT_SPORT_API_ID_MOVE);
            client.register_api(ROBOT_SPORT_API_ID_SWITCHGAIT);
            client.register_api(ROBOT_SPORT_API_ID_BODYHEIGHT);
            client.register_api(ROBOT_SPORT_API_ID_SPEEDLEVEL);
            client.register_api(ROBOT_SPORT_API_ID_TRAJECTORYFOLLOW);
            client.register_api(ROBOT_SPORT_API_ID_CONTINUOUSGAIT);
            client.register_api(ROBOT_SPORT_API_ID_MOVETOPOS);
            client.register_api(ROBOT_SPORT_API_ID_SWITCHMOVEMODE);
            client.register_api(ROBOT_SPORT_API_ID_HANDSTAND);
            client.register_api(ROBOT_SPORT_API_ID_AUTORECOVERY_SET);
            client.register_api(ROBOT_SPORT_API_ID_FREEWALK);
            client.register_api(ROBOT_SPORT_API_ID_CLASSICWALK);
            client.register_api(ROBOT_SPORT_API_ID_FASTWALK);
            client.register_api(ROBOT_SPORT_API_ID_FREEEULER);
            client.register_api(ROBOT_SPORT_API_ID_FREE_HEIGHT);
            client.register_api(ROBOT_SPORT_API_ID_GAIT_HEIGHT);
            client.register_api(ROBOT_SPORT_API_ID_VISIONWALK);
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
        /// api 1008 `Move` -> {"vx":..,"vy":..,"vyaw":..}
        pub async fn move_to(&self, vx: f32, vy: f32, vyaw: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"vx": vx, "vy": vy, "vyaw": vyaw});
            self.client.call(ROBOT_SPORT_API_ID_MOVE, body.to_string()).await
        }
        /// api 1011 `SwitchGait` -> {"d":..}
        pub async fn switch_gait(&self, d: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"d": d});
            self.client.call(ROBOT_SPORT_API_ID_SWITCHGAIT, body.to_string()).await
        }
        /// api 1013 `BodyHeight` -> {"height":..}
        pub async fn body_height(&self, height: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"height": height});
            self.client.call(ROBOT_SPORT_API_ID_BODYHEIGHT, body.to_string()).await
        }
        /// api 1015 `SpeedLevel` -> {"level":..}
        pub async fn speed_level(&self, level: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"level": level});
            self.client.call(ROBOT_SPORT_API_ID_SPEEDLEVEL, body.to_string()).await
        }
        /// api 1019 `ContinuousGait` -> {"flag":..}
        pub async fn continuous_gait(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_CONTINUOUSGAIT, body.to_string()).await
        }
        /// api 1036 `MoveToPos` -> {"x":..,"y":..,"yaw":..}
        pub async fn move_to_pos(&self, x: f32, y: f32, yaw: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"x": x, "y": y, "yaw": yaw});
            self.client.call(ROBOT_SPORT_API_ID_MOVETOPOS, body.to_string()).await
        }
        /// api 1038 `SwitchMoveMode` -> {"flag":..}
        pub async fn switch_move_mode(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_SWITCHMOVEMODE, body.to_string()).await
        }
        /// api 1101 `VisionWalk` -> {"flag":..}
        pub async fn vision_walk(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_VISIONWALK, body.to_string()).await
        }
        /// api 1039 `HandStand` -> {"flag":..}
        pub async fn hand_stand(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_HANDSTAND, body.to_string()).await
        }
        /// api 1040 `AutoRecoverySet` -> {"flag":..}
        pub async fn auto_recovery_set(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_AUTORECOVERY_SET, body.to_string()).await
        }
        /// api 1045 `FreeWalk`.
        pub async fn free_walk(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_FREEWALK, "{}".into()).await
        }
        /// api 1049 `ClassicWalk` -> {"flag":..}
        pub async fn classic_walk(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_CLASSICWALK, body.to_string()).await
        }
        /// api 1050 `FastWalk` -> {"flag":..}
        pub async fn fast_walk(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_FASTWALK, body.to_string()).await
        }
        /// api 1051 `Euler` -> {"roll":..,"pitch":..,"yaw":..} (api const ROBOT_SPORT_API_ID_FREEEULER)
        pub async fn euler(&self, roll: f32, pitch: f32, yaw: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"roll": roll, "pitch": pitch, "yaw": yaw});
            self.client.call(ROBOT_SPORT_API_ID_FREEEULER, body.to_string()).await
        }
        /// api 1052 `FreeHeight` -> {"flag":..}
        pub async fn free_height(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_FREE_HEIGHT, body.to_string()).await
        }
        /// api 1053 `GaitHeight` -> {"flag":..}
        pub async fn gait_height(&self, flag: bool) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"flag": flag});
            self.client.call(ROBOT_SPORT_API_ID_GAIT_HEIGHT, body.to_string()).await
        }
        /// api 1018 `TrajectoryFollow` ->
        /// {"path":[{"timeFromStart":..,"x":..,"y":..,"yaw":..,"vx":..,"vy":..,"vyaw":..},...]}
        pub async fn trajectory_follow(&self, path: &[PathPoint]) -> anyhow::Result<RpcResponse> {
            let points: Vec<_> = path
                .iter()
                .map(|pt| {
                    serde_json::json!({
                        "timeFromStart": pt.time_from_start,
                        "x": pt.x,
                        "y": pt.y,
                        "yaw": pt.yaw,
                        "vx": pt.vx,
                        "vy": pt.vy,
                        "vyaw": pt.vyaw
                    })
                })
                .collect();
            let body = serde_json::json!({"path": points});
            self.client
                .call(ROBOT_SPORT_API_ID_TRAJECTORYFOLLOW, body.to_string())
                .await
        }
    }
}
