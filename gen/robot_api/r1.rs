// ⚡ Robot RPC clients for the r1 model.
// Hand-verified against tmp_sdk2/include/unitree/robot/r1/* (unitree_sdk2).
// Service name / api_id constants come from the *_api.hpp headers; each method
// mirrors the C++ *_client.hpp signature. Payload JSON keys follow the request
// structs the C++ methods fill inline (audio_api.hpp / r1_loco_api.hpp and the
// shared go2::JsonizeDataInt), not the C++ parameter names where they differ.
//
//   let client = robot_api::r1::loco::LocoClient::new()?;
//   let resp = client.set_velocity(0.3, 0.0, 0.0, 1.0).await?;  // {"velocity":[..],"duration":..}

pub mod audio {
    use crate::rpc::{RpcClient, RpcResponse};
    use std::sync::atomic::{AtomicU32, Ordering};

    pub const SERVICE_NAME: &str = "voice";
    pub const API_VERSION: &str = "1.0.0.0";
    pub const ROBOT_API_ID_AUDIO_TTS: i32 = 1001;
    pub const ROBOT_API_ID_AUDIO_ASR: i32 = 1002;
    pub const ROBOT_API_ID_AUDIO_START_PLAY: i32 = 1003;
    pub const ROBOT_API_ID_AUDIO_STOP_PLAY: i32 = 1004;
    pub const ROBOT_API_ID_AUDIO_GET_VOLUME: i32 = 1005;
    pub const ROBOT_API_ID_AUDIO_SET_VOLUME: i32 = 1006;
    pub const ROBOT_API_ID_AUDIO_SET_RGB_LED: i32 = 1010;

    // The C++ AudioClient auto-increments a private `tts_index` member (uint32_t,
    // starting at 0) and sends it as the `index` key of every TTS request. A shared
    // process-wide counter keeps the Rust client stateless while preserving the
    // exact payload shape (the index is never surfaced to the caller in the SDK).
    static TTS_INDEX: AtomicU32 = AtomicU32::new(0);

    /// Rust mirror of the C++ `unitree::robot::r1::AudioClient`.
    pub struct AudioClient {
        client: RpcClient,
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
            Ok(Self { client })
        }
        // api 1002 `ASR` is registered but has no client method: the C++ client does
        // not call it over RPC — ASR results arrive on the DDS topic `rt/audio_msg`.

        /// api 1001 `TtsMaker` -> {"index":..,"speaker_id":..,"text":..}
        /// (`index` is auto-incremented like the C++ client's private `tts_index`).
        pub async fn tts_maker(&self, text: &str, speaker_id: i32) -> anyhow::Result<RpcResponse> {
            let index = TTS_INDEX.fetch_add(1, Ordering::Relaxed);
            let body = serde_json::json!({"index": index, "speaker_id": speaker_id, "text": text});
            self.client.call(ROBOT_API_ID_AUDIO_TTS, body.to_string()).await
        }
        /// api 1005 `GetVolume` (the volume is returned in the response data, key `volume`).
        pub async fn get_volume(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_AUDIO_GET_VOLUME, "{}".into()).await
        }
        /// api 1006 `SetVolume` -> {"volume":..}
        pub async fn set_volume(&self, volume: u8) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"volume": volume});
            self.client.call(ROBOT_API_ID_AUDIO_SET_VOLUME, body.to_string()).await
        }
        // api 1003 `PlayStream` is not exposed: C++ `PlayStream(app_name, stream_id,
        // pcm_data)` sends the PCM chunks as the *binary* Call-data argument, which
        // crate::rpc::RpcClient::call(api_id, request) cannot carry yet. Its JSON
        // parameter part would be {"app_name":..,"stream_id":..}.

        /// api 1004 `PlayStop` -> {"app_name":..}
        /// (the C++ example passes the stream id as the app-name parameter).
        pub async fn play_stop(&self, app_name: &str) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"app_name": app_name});
            self.client.call(ROBOT_API_ID_AUDIO_STOP_PLAY, body.to_string()).await
        }
        /// api 1010 `LedControl` -> {"R":..,"G":..,"B":..}
        pub async fn led_control(&self, r: u8, g: u8, b: u8) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"R": r, "G": g, "B": b});
            self.client.call(ROBOT_API_ID_AUDIO_SET_RGB_LED, body.to_string()).await
        }
    }
}

pub mod loco {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "sport";
    pub const API_VERSION: &str = "1.0.0.0";
    pub const ROBOT_API_ID_LOCO_GET_FSM_ID: i32 = 7001;
    pub const ROBOT_API_ID_LOCO_GET_FSM_MODE: i32 = 7002;
    pub const ROBOT_API_ID_LOCO_SET_FSM_ID: i32 = 7101;
    pub const ROBOT_API_ID_LOCO_SET_VELOCITY: i32 = 7105;
    pub const ROBOT_API_ID_LOCO_SET_SPEED_MODE: i32 = 7107;

    /// Rust mirror of the C++ `unitree::robot::r1::LocoClient`.
    pub struct LocoClient {
        client: RpcClient,
    }

    impl LocoClient {
        /// Connects to the `sport` service (r1 locomotion) and registers all its API ids.
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_API_ID_LOCO_GET_FSM_ID);
            client.register_api(ROBOT_API_ID_LOCO_GET_FSM_MODE);
            client.register_api(ROBOT_API_ID_LOCO_SET_FSM_ID);
            client.register_api(ROBOT_API_ID_LOCO_SET_VELOCITY);
            client.register_api(ROBOT_API_ID_LOCO_SET_SPEED_MODE);
            Ok(Self { client })
        }
        /// api 7001 `GetFsmId` (the fsm id is returned in the response data, key `data`).
        pub async fn get_fsm_id(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_LOCO_GET_FSM_ID, "{}".into()).await
        }
        /// api 7002 `GetFsmMode` (the fsm mode is returned in the response data, key `data`).
        pub async fn get_fsm_mode(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_LOCO_GET_FSM_MODE, "{}".into()).await
        }
        /// api 7101 `SetFsmId` -> {"data":..}
        pub async fn set_fsm_id(&self, fsm_id: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": fsm_id});
            self.client.call(ROBOT_API_ID_LOCO_SET_FSM_ID, body.to_string()).await
        }
        /// api 7105 `SetVelocity` -> {"velocity":[...],"duration":..}
        /// (C++ declares `duration = 1.f` as a default value when omitted).
        pub async fn set_velocity(
            &self,
            vx: f32,
            vy: f32,
            omega: f32,
            duration: f32,
        ) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"velocity": [vx, vy, omega], "duration": duration});
            self.client.call(ROBOT_API_ID_LOCO_SET_VELOCITY, body.to_string()).await
        }
        /// api 7101 `Damp` (SetFsmId(1)) -> {"data":1}
        pub async fn damp(&self) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": 1});
            self.client.call(ROBOT_API_ID_LOCO_SET_FSM_ID, body.to_string()).await
        }
        /// api 7101 `Start` (SetFsmId(811)) -> {"data":811}
        pub async fn start(&self) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": 811});
            self.client.call(ROBOT_API_ID_LOCO_SET_FSM_ID, body.to_string()).await
        }
        /// api 7101 `StandUp` (SetFsmId(4)) -> {"data":4}
        pub async fn stand_up(&self) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": 4});
            self.client.call(ROBOT_API_ID_LOCO_SET_FSM_ID, body.to_string()).await
        }
        /// api 7101 `ZeroTorque` (SetFsmId(0)) -> {"data":0}
        pub async fn zero_torque(&self) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": 0});
            self.client.call(ROBOT_API_ID_LOCO_SET_FSM_ID, body.to_string()).await
        }
        /// api 7105 `StopMove` (SetVelocity(0,0,0)) -> {"velocity":[0,0,0],"duration":1.0}
        pub async fn stop_move(&self) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"velocity": [0.0, 0.0, 0.0], "duration": 1.0});
            self.client.call(ROBOT_API_ID_LOCO_SET_VELOCITY, body.to_string()).await
        }
        /// api 7105 `Move` -> {"velocity":[...],"duration":..}
        /// (duration = 864000 s when `continous_move` is true, otherwise 1 s).
        pub async fn move_to(
            &self,
            vx: f32,
            vy: f32,
            vyaw: f32,
            continous_move: bool,
        ) -> anyhow::Result<RpcResponse> {
            let duration = if continous_move { 864000.0 } else { 1.0 };
            let body = serde_json::json!({"velocity": [vx, vy, vyaw], "duration": duration});
            self.client.call(ROBOT_API_ID_LOCO_SET_VELOCITY, body.to_string()).await
        }
        /// api 7107 `SetSpeedMode` -> {"data":..}
        pub async fn set_speed_mode(&self, speed_mode: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": speed_mode});
            self.client.call(ROBOT_API_ID_LOCO_SET_SPEED_MODE, body.to_string()).await
        }
    }
    // Not exposed (C++ LocoClient members that do not map onto an RPC call):
    // - `Move(vx, vy, vyaw)` 3-arg overload: its duration comes from the client's
    //   private `continous_move_` member; use `move_to(vx, vy, vyaw, continous_move)`
    //   with an explicit flag instead.
    // - `SwitchMoveMode(bool)`: only toggles that private member, never calls RPC.
    // - `Squat()` / `Sit()`: commented out in the C++ header.
}
