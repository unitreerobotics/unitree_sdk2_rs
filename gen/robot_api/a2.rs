// ⚡ Robot RPC clients for the a2 model.
// Hand-verified against tmp_sdk2/include/unitree/robot/a2/* (unitree_sdk2).
// Service name / api_id constants come from the *_api.hpp headers. The a2
// service clients are inlined in *_client.hpp: each method fills a Jsonize
// struct (or raw JsonMap keys) and serializes it right inside the body, so the
// payload JSON keys below follow those inline struct fields (e.g. `Euler` /
// `Move` fill go2::JsonizeVec3 → keys {"x","y","z"}), not the C++ parameter
// names.
//
//   let client = robot_api::a2::sport::SportClient::new()?;
//   let resp = client.move_to(0.3, 0.0, 0.0).await?;   // {"x":..,"y":..,"z":..}

pub mod audio {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "voice";
    pub const API_VERSION: &str = "1.0.0.0";
    pub const ROBOT_API_ID_AUDIO_TTS: i32 = 1001;
    pub const ROBOT_API_ID_AUDIO_ASR: i32 = 1002;
    pub const ROBOT_API_ID_AUDIO_START_PLAY: i32 = 1003;
    pub const ROBOT_API_ID_AUDIO_STOP_PLAY: i32 = 1004;
    pub const ROBOT_API_ID_AUDIO_GET_VOLUME: i32 = 1005;
    pub const ROBOT_API_ID_AUDIO_SET_VOLUME: i32 = 1006;
    pub const ROBOT_API_ID_AUDIO_SET_RGB_LED: i32 = 1010;

    /// Rust mirror of the C++ `unitree::robot::a2::AudioClient`.
    pub struct AudioClient {
        client: RpcClient,
        // Mirrors the C++ `uint32_t tts_index` member: post-incremented on every
        // TtsMaker call and serialized as the `index` payload key.
        tts_index: std::sync::atomic::AtomicU32,
    }

    impl AudioClient {
        /// Connects to the `voice` service and registers all its API ids.
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_API_ID_AUDIO_TTS);
            client.register_api(ROBOT_API_ID_AUDIO_ASR);
            client.register_api(ROBOT_API_ID_AUDIO_START_PLAY);
            client.register_api(ROBOT_API_ID_AUDIO_STOP_PLAY);
            client.register_api(ROBOT_API_ID_AUDIO_GET_VOLUME);
            client.register_api(ROBOT_API_ID_AUDIO_SET_VOLUME);
            client.register_api(ROBOT_API_ID_AUDIO_SET_RGB_LED);
            Ok(Self {
                client,
                tts_index: std::sync::atomic::AtomicU32::new(0),
            })
        }
        // C++ `PlayStream` (ROBOT_API_ID_AUDIO_START_PLAY) also sends `pcm_data`
        // as a binary payload, which crate::rpc::call (JSON request string only)
        // cannot carry — not exposed. ROBOT_API_ID_AUDIO_ASR has no client method
        // in audio_client.hpp either; both ids stay registered to mirror Init().
        /// api 1001 `TtsMaker` -> {"index":..,"speaker_id":..,"text":..}
        pub async fn tts_maker(&self, text: &str, speaker_id: i32) -> anyhow::Result<RpcResponse> {
            // C++ keeps a per-client counter: index = tts_index++ (starts at 0).
            let index = self.tts_index.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let body = serde_json::json!({"index": index, "speaker_id": speaker_id, "text": text});
            self.client.call(ROBOT_API_ID_AUDIO_TTS, body.to_string()).await
        }
        /// api 1005 `GetVolume` (C++ output param `volume` is returned in the response data).
        pub async fn get_volume(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_AUDIO_GET_VOLUME, "{}".into()).await
        }
        /// api 1006 `SetVolume` -> {"volume":..}
        /// (C++ wraps it in JsonizeCommObjInt{name="volume", value=volume}, so the
        /// JSON key is the `name` member value, i.e. "volume".)
        pub async fn set_volume(&self, volume: u8) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"volume": volume});
            self.client.call(ROBOT_API_ID_AUDIO_SET_VOLUME, body.to_string()).await
        }
        /// api 1004 `PlayStop` -> {"app_name":..}
        pub async fn play_stop(&self, app_name: &str) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"app_name": app_name});
            self.client.call(ROBOT_API_ID_AUDIO_STOP_PLAY, body.to_string()).await
        }
        /// api 1010 `LedControl` -> {"R":..,"G":..,"B":..} (fields are uppercase in the wire payload)
        pub async fn led_control(&self, r: u8, g: u8, b: u8) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"R": r, "G": g, "B": b});
            self.client.call(ROBOT_API_ID_AUDIO_SET_RGB_LED, body.to_string()).await
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
    pub const ROBOT_SPORT_API_ID_EULER: i32 = 1007;
    pub const ROBOT_SPORT_API_ID_MOVE: i32 = 1008;
    pub const ROBOT_SPORT_API_ID_BODYPOSITION: i32 = 1009;
    pub const ROBOT_SPORT_API_ID_SWITCHGAIT: i32 = 1011;
    pub const ROBOT_SPORT_API_ID_BODYHEIGHT: i32 = 1013;
    pub const ROBOT_SPORT_API_ID_SPEEDLEVEL: i32 = 1015;
    pub const ROBOT_SPORT_API_ID_LEFTSIDEGAIT: i32 = 1016;
    pub const ROBOT_SPORT_API_ID_RIGHTSIDEGAIT: i32 = 1017;
    pub const ROBOT_SPORT_API_ID_HANDSTAND: i32 = 1018;
    pub const ROBOT_SPORT_API_ID_BIPEDSTAND: i32 = 1019;
    pub const ROBOT_SPORT_API_ID_FRONTFLIP: i32 = 1020;
    pub const ROBOT_SPORT_API_ID_BACKFLIP: i32 = 1021;
    pub const ROBOT_SPORT_API_ID_SETAUTORECOVERY: i32 = 1040;
    pub const ROBOT_SPORT_API_ID_RESET_ESTIMATOR: i32 = 1111;
    pub const ROBOT_SPORT_API_ID_GETSTATE: i32 = 1034;
    pub const ROBOT_SPORT_API_ID_TRAJECTORY: i32 = 1035;

    /// Mirror of the C++ `unitree::robot::a2::sport::PathPoint` used by `Trajectory`.
    #[derive(Debug, Clone, Copy, Default)]
    pub struct PathPoint {
        pub t_from_start: f32,
        pub x: f32,
        pub y: f32,
        pub yaw: f32,
    }

    /// Rust mirror of the C++ `unitree::robot::a2::SportClient`.
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
            client.register_api(ROBOT_SPORT_API_ID_BODYPOSITION);
            client.register_api(ROBOT_SPORT_API_ID_SWITCHGAIT);
            client.register_api(ROBOT_SPORT_API_ID_BODYHEIGHT);
            client.register_api(ROBOT_SPORT_API_ID_SPEEDLEVEL);
            client.register_api(ROBOT_SPORT_API_ID_LEFTSIDEGAIT);
            client.register_api(ROBOT_SPORT_API_ID_RIGHTSIDEGAIT);
            client.register_api(ROBOT_SPORT_API_ID_HANDSTAND);
            client.register_api(ROBOT_SPORT_API_ID_BIPEDSTAND);
            client.register_api(ROBOT_SPORT_API_ID_FRONTFLIP);
            client.register_api(ROBOT_SPORT_API_ID_BACKFLIP);
            client.register_api(ROBOT_SPORT_API_ID_SETAUTORECOVERY);
            client.register_api(ROBOT_SPORT_API_ID_RESET_ESTIMATOR);
            client.register_api(ROBOT_SPORT_API_ID_GETSTATE);
            client.register_api(ROBOT_SPORT_API_ID_TRAJECTORY);
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
        /// api 1007 `Euler` -> {"x":..,"y":..,"z":..} (roll→x, pitch→y, yaw→z; JsonizeVec3)
        pub async fn euler(&self, roll: f32, pitch: f32, yaw: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"x": roll, "y": pitch, "z": yaw});
            self.client.call(ROBOT_SPORT_API_ID_EULER, body.to_string()).await
        }
        /// api 1008 `Move` -> {"x":..,"y":..,"z":..} (vx→x, vy→y, vyaw→z; JsonizeVec3)
        pub async fn move_to(&self, vx: f32, vy: f32, vyaw: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"x": vx, "y": vy, "z": vyaw});
            self.client.call(ROBOT_SPORT_API_ID_MOVE, body.to_string()).await
        }
        /// api 1011 `SwitchGait` -> {"data":..} (gait_type → data; JsonizeDataInt)
        pub async fn switch_gait(&self, gait_type: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": gait_type});
            self.client.call(ROBOT_SPORT_API_ID_SWITCHGAIT, body.to_string()).await
        }
        /// api 1013 `BodyHeight` -> {"data":..} (height → data; JsonizeDataFloat)
        pub async fn body_height(&self, height: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": height});
            self.client.call(ROBOT_SPORT_API_ID_BODYHEIGHT, body.to_string()).await
        }
        /// api 1015 `SpeedLevel` -> {"data":..} (level → data; JsonizeDataInt)
        pub async fn speed_level(&self, level: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": level});
            self.client.call(ROBOT_SPORT_API_ID_SPEEDLEVEL, body.to_string()).await
        }
        /// api 1009 `BodyPosition` -> {"x":..,"y":..,"z":..,"yaw":..} (PoseVec4)
        pub async fn body_position(&self, x: f32, y: f32, z: f32, yaw: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"x": x, "y": y, "z": z, "yaw": yaw});
            self.client.call(ROBOT_SPORT_API_ID_BODYPOSITION, body.to_string()).await
        }
        /// api 1016 `LeftSideGait` -> {"data":..} (enter → data; JsonizeDataInt)
        pub async fn left_side_gait(&self, enter: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": enter});
            self.client.call(ROBOT_SPORT_API_ID_LEFTSIDEGAIT, body.to_string()).await
        }
        /// api 1017 `RightSideGait` -> {"data":..} (enter → data; JsonizeDataInt)
        pub async fn right_side_gait(&self, enter: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": enter});
            self.client.call(ROBOT_SPORT_API_ID_RIGHTSIDEGAIT, body.to_string()).await
        }
        /// api 1018 `HandStand` -> {"data":..} (enter → data; JsonizeDataInt)
        pub async fn hand_stand(&self, enter: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": enter});
            self.client.call(ROBOT_SPORT_API_ID_HANDSTAND, body.to_string()).await
        }
        /// api 1019 `BipedStand` -> {"data":..} (enter → data; JsonizeDataInt)
        pub async fn biped_stand(&self, enter: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": enter});
            self.client.call(ROBOT_SPORT_API_ID_BIPEDSTAND, body.to_string()).await
        }
        /// api 1020 `FrontFlip`.
        pub async fn front_flip(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_FRONTFLIP, "{}".into()).await
        }
        /// api 1021 `BackFlip`.
        pub async fn back_flip(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_BACKFLIP, "{}".into()).await
        }
        /// api 1111 `ResetEstimator`.
        pub async fn reset_estimator(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_RESET_ESTIMATOR, "{}".into()).await
        }
        /// api 1035 `Trajectory` -> {"path":[{t_from_start,x,y,yaw},..],"feedback_mode":..,"external_x":..,"external_y":..,"external_yaw":..}
        /// C++ defaults feedback_mode=0, external_x/y/yaw=0; pass them explicitly here.
        pub async fn trajectory(
            &self,
            path: &[PathPoint],
            feedback_mode: i32,
            external_x: f32,
            external_y: f32,
            external_yaw: f32,
        ) -> anyhow::Result<RpcResponse> {
            let path_points: Vec<_> = path
                .iter()
                .map(|pt| {
                    serde_json::json!({
                        "t_from_start": pt.t_from_start,
                        "x": pt.x,
                        "y": pt.y,
                        "yaw": pt.yaw
                    })
                })
                .collect();
            let body = serde_json::json!({
                "path": path_points,
                "feedback_mode": feedback_mode,
                "external_x": external_x,
                "external_y": external_y,
                "external_yaw": external_yaw
            });
            self.client.call(ROBOT_SPORT_API_ID_TRAJECTORY, body.to_string()).await
        }
        /// api 1040 `SetAutoRecovery` -> {"data":..} (switch_on → data; JsonizeDataInt)
        pub async fn set_auto_recovery(&self, switch_on: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": switch_on});
            self.client.call(ROBOT_SPORT_API_ID_SETAUTORECOVERY, body.to_string()).await
        }
        /// api 1034 `GetState` (the state map is returned in the response data).
        pub async fn get_state(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_GETSTATE, "{}".into()).await
        }
    }
}
