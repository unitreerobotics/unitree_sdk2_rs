// ⚡ Robot RPC clients for the as2 model.
// Hand-verified against tmp_sdk2/include/unitree/robot/as2/* (unitree_sdk2).
// Service name / api_id constants come from the *_api.hpp headers. The as2
// service clients are inlined in *_client.hpp: each method fills a Jsonize
// struct (or raw JsonMap keys) and serializes it right inside the body, so the
// payload JSON keys below follow those inline struct fields (e.g. `Euler` /
// `Move` fill go2::JsonizeVec3 → keys {"x","y","z"}), not the C++ parameter
// names.
//
//   let client = robot_api::as2::sport::SportClient::new()?;
//   let resp = client.move_to(0.3, 0.0, 0.0).await?;   // {"x":..,"y":..,"z":..}

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
    pub const ROBOT_SPORT_API_ID_GREETING: i32 = 1022;
    pub const ROBOT_SPORT_API_ID_HEART: i32 = 1023;
    pub const ROBOT_SPORT_API_ID_CONTENT: i32 = 1024;
    pub const ROBOT_SPORT_API_ID_DANCE1: i32 = 1025;
    pub const ROBOT_SPORT_API_ID_DANCE2: i32 = 1026;
    pub const ROBOT_SPORT_API_ID_HANDSHAKE: i32 = 1027;
    pub const ROBOT_SPORT_API_ID_STRETCH: i32 = 1028;
    pub const ROBOT_SPORT_API_ID_SIT: i32 = 1029;
    pub const ROBOT_SPORT_API_ID_FRONTJUMP: i32 = 1030;
    pub const ROBOT_SPORT_API_ID_PUSHUP: i32 = 1031;
    pub const ROBOT_SPORT_API_ID_UPJUMP: i32 = 1032;
    pub const ROBOT_SPORT_API_ID_SETAUTORECOVERY: i32 = 1040;
    pub const ROBOT_SPORT_API_ID_SWITCHJOYSTICK: i32 = 1041;
    pub const ROBOT_SPORT_API_ID_GETSTATE: i32 = 1034;

    /// Rust mirror of the C++ `unitree::robot::as2::SportClient`.
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
            client.register_api(ROBOT_SPORT_API_ID_GREETING);
            client.register_api(ROBOT_SPORT_API_ID_HEART);
            client.register_api(ROBOT_SPORT_API_ID_CONTENT);
            client.register_api(ROBOT_SPORT_API_ID_DANCE1);
            client.register_api(ROBOT_SPORT_API_ID_DANCE2);
            client.register_api(ROBOT_SPORT_API_ID_HANDSHAKE);
            client.register_api(ROBOT_SPORT_API_ID_STRETCH);
            client.register_api(ROBOT_SPORT_API_ID_SIT);
            client.register_api(ROBOT_SPORT_API_ID_FRONTJUMP);
            client.register_api(ROBOT_SPORT_API_ID_PUSHUP);
            client.register_api(ROBOT_SPORT_API_ID_UPJUMP);
            client.register_api(ROBOT_SPORT_API_ID_SETAUTORECOVERY);
            client.register_api(ROBOT_SPORT_API_ID_SWITCHJOYSTICK);
            client.register_api(ROBOT_SPORT_API_ID_GETSTATE);
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
        /// api 1022 `Greeting`.
        pub async fn greeting(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_GREETING, "{}".into()).await
        }
        /// api 1023 `Heart`.
        pub async fn heart(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_HEART, "{}".into()).await
        }
        /// api 1024 `Content`.
        pub async fn content(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_CONTENT, "{}".into()).await
        }
        /// api 1025 `Dance1`.
        pub async fn dance1(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_DANCE1, "{}".into()).await
        }
        /// api 1026 `Dance2`.
        pub async fn dance2(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_DANCE2, "{}".into()).await
        }
        /// api 1027 `Handshake`.
        pub async fn handshake(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_HANDSHAKE, "{}".into()).await
        }
        /// api 1028 `Stretch`.
        pub async fn stretch(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_STRETCH, "{}".into()).await
        }
        /// api 1029 `Sit` -> {"data":..} (enter → data; JsonizeDataInt)
        pub async fn sit(&self, enter: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": enter});
            self.client.call(ROBOT_SPORT_API_ID_SIT, body.to_string()).await
        }
        /// api 1030 `FrontJump`.
        pub async fn front_jump(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_FRONTJUMP, "{}".into()).await
        }
        /// api 1031 `PushUp`.
        pub async fn push_up(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_PUSHUP, "{}".into()).await
        }
        /// api 1032 `UpJump`.
        pub async fn up_jump(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_UPJUMP, "{}".into()).await
        }
        /// api 1040 `SetAutoRecovery` -> {"data":..} (switch_on → data; JsonizeDataInt)
        pub async fn set_auto_recovery(&self, switch_on: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": switch_on});
            self.client.call(ROBOT_SPORT_API_ID_SETAUTORECOVERY, body.to_string()).await
        }
        /// api 1041 `SwitchJoystick` -> {"data":..} (switch_on → data; JsonizeDataInt)
        pub async fn switch_joystick(&self, switch_on: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": switch_on});
            self.client.call(ROBOT_SPORT_API_ID_SWITCHJOYSTICK, body.to_string()).await
        }
        /// api 1034 `GetState` (the state map is returned in the response data).
        pub async fn get_state(&self) -> anyhow::Result<RpcResponse> {
            self.client.call(ROBOT_SPORT_API_ID_GETSTATE, "{}".into()).await
        }
    }
}
