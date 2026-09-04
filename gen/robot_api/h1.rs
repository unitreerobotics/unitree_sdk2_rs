// ⚡ Robot RPC clients for the h1 model.
// Hand-verified against tmp_sdk2/include/unitree/robot/h1/* (unitree_sdk2).
// Service name / api_id constants come from the *_api.hpp headers; each method
// mirrors the C++ *_client.hpp signature.
//
//   let client = robot_api::h1::loco::LocoClient::new()?;
//   let resp = client.set_velocity(0.3, 0.0, 0.0, 1.0).await?;
//
// Payload note: the C++ LocoClient serializes its scalar setters through the
// go2::JsonizeDataInt / JsonizeDataFloat / JsonizeVelocityCommand wrappers, so
// the on-wire request keys are `data` / `velocity` / `duration` (not the C++
// parameter names). The payload shapes below reproduce exactly what the C++
// client sends.

pub mod loco {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "loco";
    pub const API_VERSION: &str = "2.0.0.0";
    pub const ROBOT_API_ID_LOCO_GET_FSM_ID: i32 = 8001;
    pub const ROBOT_API_ID_LOCO_GET_FSM_MODE: i32 = 8002;
    pub const ROBOT_API_ID_LOCO_GET_BALANCE_MODE: i32 = 8003;
    pub const ROBOT_API_ID_LOCO_GET_SWING_HEIGHT: i32 = 8004;
    pub const ROBOT_API_ID_LOCO_GET_STAND_HEIGHT: i32 = 8005;
    pub const ROBOT_API_ID_LOCO_GET_PHASE: i32 = 8006; // deprecated in the SDK
    pub const ROBOT_API_ID_LOCO_SET_FSM_ID: i32 = 8101;
    pub const ROBOT_API_ID_LOCO_SET_BALANCE_MODE: i32 = 8102;
    pub const ROBOT_API_ID_LOCO_SET_SWING_HEIGHT: i32 = 8103;
    pub const ROBOT_API_ID_LOCO_SET_STAND_HEIGHT: i32 = 8104;
    pub const ROBOT_API_ID_LOCO_SET_VELOCITY: i32 = 8105;
    pub const ROBOT_API_ID_LOCO_SET_PHASE: i32 = 8106;
    pub const ROBOT_API_ID_LOCO_SET_ARM_TASK: i32 = 8107;
    pub const ROBOT_API_ID_LOCO_ENABLE_ODOM: i32 = 8201;
    pub const ROBOT_API_ID_LOCO_DISABLE_ODOM: i32 = 8202;
    pub const ROBOT_API_ID_LOCO_GET_ODOM: i32 = 8203;
    pub const ROBOT_API_ID_LOCO_SET_TARGET_POSITION: i32 = 8204;

    /// Rust mirror of the C++ `unitree::robot::h1::LocoClient`.
    pub struct LocoClient {
        client: RpcClient,
    }

    impl LocoClient {
        /// Connects to the `loco` service and registers all its API ids.
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_API_ID_LOCO_GET_FSM_ID);
            client.register_api(ROBOT_API_ID_LOCO_GET_FSM_MODE);
            client.register_api(ROBOT_API_ID_LOCO_GET_BALANCE_MODE);
            client.register_api(ROBOT_API_ID_LOCO_GET_SWING_HEIGHT);
            client.register_api(ROBOT_API_ID_LOCO_GET_STAND_HEIGHT);
            client.register_api(ROBOT_API_ID_LOCO_GET_PHASE);
            client.register_api(ROBOT_API_ID_LOCO_SET_FSM_ID);
            client.register_api(ROBOT_API_ID_LOCO_SET_BALANCE_MODE);
            client.register_api(ROBOT_API_ID_LOCO_SET_SWING_HEIGHT);
            client.register_api(ROBOT_API_ID_LOCO_SET_STAND_HEIGHT);
            client.register_api(ROBOT_API_ID_LOCO_SET_VELOCITY);
            client.register_api(ROBOT_API_ID_LOCO_SET_PHASE);
            client.register_api(ROBOT_API_ID_LOCO_SET_ARM_TASK);
            client.register_api(ROBOT_API_ID_LOCO_ENABLE_ODOM);
            client.register_api(ROBOT_API_ID_LOCO_DISABLE_ODOM);
            client.register_api(ROBOT_API_ID_LOCO_GET_ODOM);
            client.register_api(ROBOT_API_ID_LOCO_SET_TARGET_POSITION);
            Ok(Self { client })
        }
        /// api 8001 `GetFsmId` (C++ output param `fsm_id` is returned in the response data).
        pub async fn get_fsm_id(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_FSM_ID, "{}".into())
                .await
        }
        /// api 8002 `GetFsmMode` (C++ output param `fsm_mode` is returned in the response data).
        pub async fn get_fsm_mode(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_FSM_MODE, "{}".into())
                .await
        }
        /// api 8003 `GetBalanceMode` (C++ output param `balance_mode` is returned in the response data).
        pub async fn get_balance_mode(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_BALANCE_MODE, "{}".into())
                .await
        }
        /// api 8004 `GetSwingHeight` (C++ output param `swing_height` is returned in the response data).
        pub async fn get_swing_height(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_SWING_HEIGHT, "{}".into())
                .await
        }
        /// api 8005 `GetStandHeight` (C++ output param `stand_height` is returned in the response data).
        pub async fn get_stand_height(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_STAND_HEIGHT, "{}".into())
                .await
        }
        /// api 8006 `GetPhase` — deprecated in the SDK (the phase vector is returned in the response data).
        pub async fn get_phase(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_PHASE, "{}".into())
                .await
        }
        /// api 8101 `SetFsmId` -> {"data": fsm_id}
        pub async fn set_fsm_id(&self, fsm_id: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": fsm_id});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_FSM_ID, body.to_string())
                .await
        }
        /// api 8102 `SetBalanceMode` -> {"data": balance_mode}
        pub async fn set_balance_mode(&self, balance_mode: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": balance_mode});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_BALANCE_MODE, body.to_string())
                .await
        }
        /// api 8103 `SetSwingHeight` -> {"data": swing_height}
        pub async fn set_swing_height(&self, swing_height: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": swing_height});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_SWING_HEIGHT, body.to_string())
                .await
        }
        /// api 8104 `SetStandHeight` -> {"data": stand_height}
        pub async fn set_stand_height(&self, stand_height: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": stand_height});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_STAND_HEIGHT, body.to_string())
                .await
        }
        /// api 8106 `SetPhase` -> {"data": phase} (phase as a float array)
        pub async fn set_phase(&self, phase: &[f32]) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": phase});
            self.client.call(ROBOT_API_ID_LOCO_SET_PHASE, body.to_string()).await
        }
        /// api 8105 `SetVelocity` -> {"velocity":[vx,vy,omega],"duration":duration}
        pub async fn set_velocity(
            &self,
            vx: f32,
            vy: f32,
            omega: f32,
            duration: f32,
        ) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"velocity": [vx, vy, omega], "duration": duration});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_VELOCITY, body.to_string())
                .await
        }
        /// api 8107 `SetTaskId` -> {"data": task_id}
        pub async fn set_task_id(&self, task_id: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": task_id});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_ARM_TASK, body.to_string())
                .await
        }
        /// api 8201 `EnableOdom`.
        pub async fn enable_odom(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_ENABLE_ODOM, "{}".into())
                .await
        }
        /// api 8202 `DisableOdom`.
        pub async fn disable_odom(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_DISABLE_ODOM, "{}".into())
                .await
        }
        /// api 8203 `GetOdom` (C++ output params `x`/`y`/`yaw` are returned in the response data).
        pub async fn get_odom(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_ODOM, "{}".into())
                .await
        }
        /// api 8204 `SetTargetPos` -> {"x":..,"y":..,"yaw":..,"relative":..}
        pub async fn set_target_pos(
            &self,
            x: f32,
            y: f32,
            yaw: f32,
            relative: bool,
        ) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"x": x, "y": y, "yaw": yaw, "relative": relative});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_TARGET_POSITION, body.to_string())
                .await
        }
        // Not exposed: the C++ "High Level API" wrappers below are client-local
        // conveniences with no API id of their own — they funnel into the RPCs
        // above with fixed values: Damp (set_fsm_id(1)), Start (204), StandUp (2),
        // ZeroTorque (0), StopMove (set_velocity(0,0,0)), HighStand/LowStand
        // (set_stand_height(±max)), Move (both overloads → set_velocity),
        // BalanceStand (set_balance_mode(0)), ContinuousGait (set_balance_mode),
        // SwitchMoveMode (client-local flag), SetNextFoot (set_phase),
        // WaveHand/ShakeHand (set_task_id(0..2)).
    }
}
