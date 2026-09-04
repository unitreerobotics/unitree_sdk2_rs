// ⚡ Robot RPC clients for the g1 model.
// Hand-verified against tmp_sdk2/include/unitree/robot/g1/* (unitree_sdk2).
// Service name / api_id constants come from the *_api.hpp headers; each method
// mirrors the C++ *_client.hpp signature. The g1 clients are fully inline, so
// each request body reproduces the exact JSON the SDK sends: plain parameter
// objects are keyed by the C++ parameter names, while scalar payloads wrapped
// into go2::JsonizeData{Int,Float,Bool} use the "data" key (noted per method).
//
//   let client = robot_api::g1::loco::LocoClient::new()?;
//   let resp = client.set_velocity(0.3, 0.0, 0.0, 1.0).await?;  // {"velocity":[...],"duration":..}

pub mod agv {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "agv";
    pub const API_VERSION: &str = "1.0.0.1";
    pub const ROBOT_API_ID_AGV_MOVE: i32 = 1001;
    pub const ROBOT_API_ID_AGV_HEIGHT_ADJUST: i32 = 1002;

    /// Rust mirror of the C++ `unitree::robot::g1::AgvClient`.
    pub struct AgvClient {
        client: RpcClient,
    }

    impl AgvClient {
        /// Connects to the `agv` service and registers all its API ids.
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_API_ID_AGV_MOVE);
            client.register_api(ROBOT_API_ID_AGV_HEIGHT_ADJUST);
            Ok(Self { client })
        }
        /// api 1001 `Move` -> {"vx":..,"vy":..,"vyaw":..}
        /// Linear/angular velocity control: vx in [-1.5, 1.5] m/s, vyaw in
        /// [-0.6, 0.6] rad/s, vy is not supported by the AGV (pass 0.0).
        pub async fn move_to(&self, vx: f32, vy: f32, vyaw: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"vx": vx, "vy": vy, "vyaw": vyaw});
            self.client.call(ROBOT_API_ID_AGV_MOVE, body.to_string()).await
        }
        /// api 1002 `HeightAdjust` -> {"data": vz}
        /// Normalized column velocity in [-1.0, 1.0]. The C++ client wraps the
        /// `vz` parameter in `go2::JsonizeDataFloat`, hence the "data" key.
        pub async fn height_adjust(&self, vz: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": vz});
            self.client.call(ROBOT_API_ID_AGV_HEIGHT_ADJUST, body.to_string()).await
        }
    }
}

pub mod arm {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "arm";
    pub const API_VERSION: &str = "1.0.0.14";
    pub const ROBOT_API_ID_ARM_ACTION_EXECUTE_ACTION: i32 = 7106;
    pub const ROBOT_API_ID_ARM_ACTION_GET_ACTION_LIST: i32 = 7107;
    pub const ROBOT_API_ID_ARM_ACTION_EXECUTE_CUSTOM_ACTION: i32 = 7108;
    pub const ROBOT_API_ID_ARM_ACTION_STOP_CUSTOM_ACTION: i32 = 7113;

    /// Rust mirror of the C++ `unitree::robot::g1::G1ArmActionClient`.
    ///
    /// The C++ client has two `ExecuteAction` overloads bound to different API
    /// ids; Rust has no overloading, so they are split into `execute_action`
    /// (preset action by id) and `execute_custom_action` (teach action by name).
    /// The `action_map` member is a plain preset table (no RPC) and is not
    /// mirrored.
    pub struct G1ArmActionClient {
        client: RpcClient,
    }

    impl G1ArmActionClient {
        /// Connects to the `arm` service and registers all its API ids.
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_API_ID_ARM_ACTION_EXECUTE_ACTION);
            client.register_api(ROBOT_API_ID_ARM_ACTION_GET_ACTION_LIST);
            client.register_api(ROBOT_API_ID_ARM_ACTION_EXECUTE_CUSTOM_ACTION);
            client.register_api(ROBOT_API_ID_ARM_ACTION_STOP_CUSTOM_ACTION);
            Ok(Self { client })
        }
        /// api 7106 `ExecuteAction` -> {"action_id":..}
        /// Executes a Unitree preset action by id (id 99 releases the action).
        pub async fn execute_action(&self, action_id: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"action_id": action_id});
            self.client
                .call(ROBOT_API_ID_ARM_ACTION_EXECUTE_ACTION, body.to_string())
                .await
        }
        /// api 7108 `ExecuteAction` (custom) -> {"action_name":..}
        /// Executes a custom teach action by name. This mirrors the C++
        /// `ExecuteAction(const std::string&)` overload, which is bound to
        /// `ROBOT_API_ID_ARM_ACTION_EXECUTE_CUSTOM_ACTION`.
        pub async fn execute_custom_action(&self, action_name: &str) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"action_name": action_name});
            self.client
                .call(ROBOT_API_ID_ARM_ACTION_EXECUTE_CUSTOM_ACTION, body.to_string())
                .await
        }
        /// api 7113 `StopCustomAction`.
        pub async fn stop_custom_action(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_ARM_ACTION_STOP_CUSTOM_ACTION, "{}".into())
                .await
        }
        /// api 7107 `GetActionList` (the action list is returned in the response data).
        pub async fn get_action_list(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_ARM_ACTION_GET_ACTION_LIST, "{}".into())
                .await
        }
    }
}

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

    /// Rust mirror of the C++ `unitree::robot::g1::AudioClient`.
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
        /// api 1001 `TtsMaker` -> {"text":..,"speaker_id":..}
        /// (the C++ client also stamps an internal auto-incremented "index"
        /// field into the payload; it is not an input parameter and is omitted)
        pub async fn tts_maker(&self, text: &str, speaker_id: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"text": text, "speaker_id": speaker_id});
            self.client.call(ROBOT_API_ID_AUDIO_TTS, body.to_string()).await
        }
        /// api 1005 `GetVolume` (C++ output param `volume` is returned in the
        /// response data).
        pub async fn get_volume(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_AUDIO_GET_VOLUME, "{}".into()).await
        }
        /// api 1006 `SetVolume` -> {"volume":..}
        pub async fn set_volume(&self, volume: u8) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"volume": volume});
            self.client.call(ROBOT_API_ID_AUDIO_SET_VOLUME, body.to_string()).await
        }
        /// api 1003 `PlayStream` -> {"app_name":..,"stream_id":..}
        /// Input param `pcm_data` (raw PCM) is skipped: the C++ client ships it
        /// over the binary RPC channel, which crate::rpc::RpcClient does not
        /// support.
        pub async fn play_stream(&self, app_name: &str, stream_id: &str) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"app_name": app_name, "stream_id": stream_id});
            self.client.call(ROBOT_API_ID_AUDIO_START_PLAY, body.to_string()).await
        }
        /// api 1004 `PlayStop` -> {"app_name":..}
        pub async fn play_stop(&self, app_name: &str) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"app_name": app_name});
            self.client.call(ROBOT_API_ID_AUDIO_STOP_PLAY, body.to_string()).await
        }
        /// api 1010 `LedControl` -> {"R":..,"G":..,"B":..}
        /// JSON keys keep the C++ names (uppercase R/G/B fields).
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
    pub const ROBOT_API_ID_LOCO_GET_BALANCE_MODE: i32 = 7003;
    pub const ROBOT_API_ID_LOCO_GET_SWING_HEIGHT: i32 = 7004;
    pub const ROBOT_API_ID_LOCO_GET_STAND_HEIGHT: i32 = 7005;
    pub const ROBOT_API_ID_LOCO_GET_PHASE: i32 = 7006; // deprecated
    pub const ROBOT_API_ID_LOCO_FSM_API: i32 = 7100;
    pub const ROBOT_API_ID_LOCO_SET_FSM_ID: i32 = 7101;
    pub const ROBOT_API_ID_LOCO_SET_BALANCE_MODE: i32 = 7102;
    pub const ROBOT_API_ID_LOCO_SET_SWING_HEIGHT: i32 = 7103;
    pub const ROBOT_API_ID_LOCO_SET_STAND_HEIGHT: i32 = 7104;
    pub const ROBOT_API_ID_LOCO_SET_VELOCITY: i32 = 7105;
    pub const ROBOT_API_ID_LOCO_SET_ARM_TASK: i32 = 7106;
    pub const ROBOT_API_ID_LOCO_SET_SPEED_MODE: i32 = 7107;
    pub const ROBOT_API_ID_LOCO_SWITCH_TO_USER_CTRL: i32 = 7110;
    pub const ROBOT_API_ID_LOCO_SWITCH_TO_INTERNAL_CTRL: i32 = 7111;

    // The C++ client also declares high-level convenience wrappers that do not
    // map to their own API id (they forward to the low-level calls below with
    // fixed values), so they are not mirrored as RPC methods. Equivalent calls:
    //   Damp()->set_fsm_id(1)          Start()->set_fsm_id(500)
    //   Squat()->set_fsm_id(2)         Sit()->set_fsm_id(3)
    //   StandUp()->set_fsm_id(4)       ZeroTorque()->set_fsm_id(0)
    //   StopMove()->set_velocity(0,0,0,1)  BalanceStand()->set_balance_mode(0)
    //   ContinuousGait(f)->set_balance_mode(f?1:0)  WaveHand(f)->set_task_id(f?1:0)
    //   HighStand()->set_stand_height(4294967295.0)  LowStand()->set_stand_height(0.0)
    //   Move(vx,vy,vyaw,cont)->set_velocity(vx,vy,vyaw, cont?864000.0:1.0)
    // Move(vx,vy,vyaw) / SwitchMoveMode / ShakeHand keep per-client state
    // (continous_move_, first_shake_hand_stage_) and are not stateless RPCs.
    // GetMimicMotion / _fsm_api are raw-parameter wrappers of the FSM_API id
    // (GetMimicMotion sends {"fsm_id":550,"api_id":1} to api 7100); no method
    // is generated for them because their request body is not built from input
    // parameters.

    /// Rust mirror of the C++ `unitree::robot::g1::LocoClient`.
    pub struct LocoClient {
        client: RpcClient,
    }

    impl LocoClient {
        /// Connects to the `sport` service and registers all its API ids.
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_API_ID_LOCO_GET_FSM_ID);
            client.register_api(ROBOT_API_ID_LOCO_GET_FSM_MODE);
            client.register_api(ROBOT_API_ID_LOCO_GET_BALANCE_MODE);
            client.register_api(ROBOT_API_ID_LOCO_GET_SWING_HEIGHT);
            client.register_api(ROBOT_API_ID_LOCO_GET_STAND_HEIGHT);
            client.register_api(ROBOT_API_ID_LOCO_GET_PHASE);
            client.register_api(ROBOT_API_ID_LOCO_FSM_API);
            client.register_api(ROBOT_API_ID_LOCO_SET_FSM_ID);
            client.register_api(ROBOT_API_ID_LOCO_SET_BALANCE_MODE);
            client.register_api(ROBOT_API_ID_LOCO_SET_SWING_HEIGHT);
            client.register_api(ROBOT_API_ID_LOCO_SET_STAND_HEIGHT);
            client.register_api(ROBOT_API_ID_LOCO_SET_VELOCITY);
            client.register_api(ROBOT_API_ID_LOCO_SET_ARM_TASK);
            client.register_api(ROBOT_API_ID_LOCO_SET_SPEED_MODE);
            client.register_api(ROBOT_API_ID_LOCO_SWITCH_TO_USER_CTRL);
            client.register_api(ROBOT_API_ID_LOCO_SWITCH_TO_INTERNAL_CTRL);
            Ok(Self { client })
        }
        /// api 7001 `GetFsmId` (C++ output param `fsm_id` is returned in the
        /// response data).
        pub async fn get_fsm_id(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_LOCO_GET_FSM_ID, "{}".into()).await
        }
        /// api 7002 `GetFsmMode` (C++ output param `fsm_mode` is returned in
        /// the response data).
        pub async fn get_fsm_mode(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_LOCO_GET_FSM_MODE, "{}".into()).await
        }
        /// api 7003 `GetBalanceMode` (C++ output param `balance_mode` is
        /// returned in the response data).
        pub async fn get_balance_mode(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_LOCO_GET_BALANCE_MODE, "{}".into()).await
        }
        /// api 7004 `GetSwingHeight` (C++ output param `swing_height` is
        /// returned in the response data).
        pub async fn get_swing_height(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_LOCO_GET_SWING_HEIGHT, "{}".into()).await
        }
        /// api 7005 `GetStandHeight` (C++ output param `stand_height` is
        /// returned in the response data).
        pub async fn get_stand_height(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_LOCO_GET_STAND_HEIGHT, "{}".into()).await
        }
        /// api 7006 `GetPhase` (deprecated in the SDK; C++ output param `phase`
        /// is returned in the response data).
        pub async fn get_phase(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_API_ID_LOCO_GET_PHASE, "{}".into()).await
        }
        /// api 7101 `SetFsmId` -> {"data": fsm_id}
        /// Switches the locomotion FSM state (e.g. 0 = zero torque, 1 = damp,
        /// 2 = squat, 3 = sit, 4 = stand up, 500 = start).
        pub async fn set_fsm_id(&self, fsm_id: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": fsm_id});
            self.client.call(ROBOT_API_ID_LOCO_SET_FSM_ID, body.to_string()).await
        }
        /// api 7102 `SetBalanceMode` -> {"data": balance_mode}
        pub async fn set_balance_mode(&self, balance_mode: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": balance_mode});
            self.client.call(ROBOT_API_ID_LOCO_SET_BALANCE_MODE, body.to_string()).await
        }
        /// api 7103 `SetSwingHeight` -> {"data": swing_height}
        pub async fn set_swing_height(&self, swing_height: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": swing_height});
            self.client.call(ROBOT_API_ID_LOCO_SET_SWING_HEIGHT, body.to_string()).await
        }
        /// api 7104 `SetStandHeight` -> {"data": stand_height}
        pub async fn set_stand_height(&self, stand_height: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": stand_height});
            self.client.call(ROBOT_API_ID_LOCO_SET_STAND_HEIGHT, body.to_string()).await
        }
        /// api 7105 `SetVelocity` -> {"velocity":[vx,vy,omega],"duration":..}
        /// Body velocity control (vx forward, vy lateral, omega yaw rate) held
        /// for `duration` seconds. The C++ default `duration` is 1.0 s; the
        /// continuous-move helper (864000.0 s) is not reproduced here.
        pub async fn set_velocity(&self, vx: f32, vy: f32, omega: f32, duration: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"velocity": [vx, vy, omega], "duration": duration});
            self.client.call(ROBOT_API_ID_LOCO_SET_VELOCITY, body.to_string()).await
        }
        /// api 7106 `SetTaskId` -> {"data": task_id}
        /// Sets the upper-body (arm) task id used by high-level motions
        /// (C++ method `SetTaskId`, API id constant `SET_ARM_TASK`).
        pub async fn set_task_id(&self, task_id: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": task_id});
            self.client.call(ROBOT_API_ID_LOCO_SET_ARM_TASK, body.to_string()).await
        }
        /// api 7110 `SwitchToUserCtrl` -> {"data": false}
        /// (the C++ client sends a default `JsonizeDataBool`, i.e. {"data":false})
        pub async fn switch_to_user_ctrl(&self) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": false});
            self.client.call(ROBOT_API_ID_LOCO_SWITCH_TO_USER_CTRL, body.to_string()).await
        }
        /// api 7111 `SwitchToInternalCtrl` -> {"data": mode}
        /// C++ enum `InternalFsmMode` maps to int: 0 = LAST, 1 = PASSIVE,
        /// 2 = WALKRUN (the enum type itself is not representable, so `mode`
        /// takes the wire value directly).
        pub async fn switch_to_internal_ctrl(&self, mode: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": mode});
            self.client.call(ROBOT_API_ID_LOCO_SWITCH_TO_INTERNAL_CTRL, body.to_string()).await
        }
        /// api 7107 `SetSpeedMode` -> {"data": speed_mode}
        pub async fn set_speed_mode(&self, speed_mode: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": speed_mode});
            self.client.call(ROBOT_API_ID_LOCO_SET_SPEED_MODE, body.to_string()).await
        }
    }
}
