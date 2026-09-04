// ⚡ Robot RPC clients for the h2 model.
// Hand-verified against tmp_sdk2/include/unitree/robot/h2/{arm,loco}
// (unitree_sdk2). Service name / api_id constants come from the *_api.hpp
// headers; each method mirrors the C++ *_client.hpp signature.
//
//   let client = robot_api::h2::loco::LocoClient::new()?;  // service "sport"
//   let resp = client.set_fsm_id(601).await?;
//   let arm = robot_api::h2::arm::H2ArmActionClient::new()?;
//   let resp = arm.execute_action(11).await?;
//
// Payload note: the C++ LocoClient serializes its scalar setters through the
// go2::JsonizeDataInt / JsonizeDataFloat / JsonizeDataBool /
// JsonizeVelocityCommand wrappers, so the on-wire request keys are
// `data` / `velocity` / `duration` (not the C++ parameter names). The payload
// shapes below reproduce exactly what the C++ client sends.

pub mod arm {
    use crate::rpc::{RpcClient, RpcResponse};

    pub const SERVICE_NAME: &str = "arm";
    pub const API_VERSION: &str = "1.0.0.14";
    pub const ROBOT_API_ID_ARM_ACTION_EXECUTE_ACTION: i32 = 7106;
    pub const ROBOT_API_ID_ARM_ACTION_GET_ACTION_LIST: i32 = 7107;
    pub const ROBOT_API_ID_ARM_ACTION_EXECUTE_CUSTOM_ACTION: i32 = 7108;
    pub const ROBOT_API_ID_ARM_ACTION_STOP_CUSTOM_ACTION: i32 = 7113;

    /// Rust mirror of the C++ `unitree::robot::h2::H2ArmActionClient`.
    pub struct H2ArmActionClient {
        client: RpcClient,
    }

    impl H2ArmActionClient {
        /// Connects to the `arm` service and registers all its API ids.
        pub fn new() -> anyhow::Result<Self> {
            let client = RpcClient::new(SERVICE_NAME)?;
            client.register_api(ROBOT_API_ID_ARM_ACTION_EXECUTE_ACTION);
            client.register_api(ROBOT_API_ID_ARM_ACTION_GET_ACTION_LIST);
            client.register_api(ROBOT_API_ID_ARM_ACTION_EXECUTE_CUSTOM_ACTION);
            client.register_api(ROBOT_API_ID_ARM_ACTION_STOP_CUSTOM_ACTION);
            Ok(Self { client })
        }
        /// api 7106 `ExecuteAction(int32_t action_id)` -> {"action_id":..}
        pub async fn execute_action(&self, action_id: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"action_id": action_id});
            self.client
                .call(ROBOT_API_ID_ARM_ACTION_EXECUTE_ACTION, body.to_string())
                .await
        }
        /// api 7108 `ExecuteAction(const std::string& action_name)` -> {"action_name":..}
        /// C++ overload of `ExecuteAction`; renamed to mirror the API id it dispatches to.
        pub async fn execute_custom_action(
            &self,
            action_name: &str,
        ) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"action_name": action_name});
            self.client
                .call(
                    ROBOT_API_ID_ARM_ACTION_EXECUTE_CUSTOM_ACTION,
                    body.to_string(),
                )
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
        // Not exposed: the static `action_map` (name -> action_id) is client-local
        // bookkeeping, not an RPC.
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
    pub const ROBOT_API_ID_LOCO_GET_PHASE: i32 = 7006; // deprecated in the SDK
    pub const ROBOT_API_ID_LOCO_GET_ARM_SDK_STATUS: i32 = 7007;
    pub const ROBOT_API_ID_LOCO_GET_AVAILABLE_FSM_IDS: i32 = 7008;
    pub const ROBOT_API_ID_LOCO_SET_FSM_ID: i32 = 7101;
    pub const ROBOT_API_ID_LOCO_SET_BALANCE_MODE: i32 = 7102;
    pub const ROBOT_API_ID_LOCO_SET_SWING_HEIGHT: i32 = 7103;
    pub const ROBOT_API_ID_LOCO_SET_STAND_HEIGHT: i32 = 7104;
    pub const ROBOT_API_ID_LOCO_SET_VELOCITY: i32 = 7105;
    pub const ROBOT_API_ID_LOCO_SET_ARM_TASK: i32 = 7106;
    pub const ROBOT_API_ID_LOCO_SET_SPEED_MODE: i32 = 7107;
    pub const ROBOT_API_ID_LOCO_SET_PUNCH_API: i32 = 7108;
    pub const ROBOT_API_ID_LOCO_SET_ARM_SDK_STATUS: i32 = 7109;

    /// Rust mirror of the C++ `unitree::robot::h2::LocoClient`.
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
            client.register_api(ROBOT_API_ID_LOCO_GET_ARM_SDK_STATUS);
            client.register_api(ROBOT_API_ID_LOCO_GET_AVAILABLE_FSM_IDS);
            client.register_api(ROBOT_API_ID_LOCO_SET_FSM_ID);
            client.register_api(ROBOT_API_ID_LOCO_SET_BALANCE_MODE);
            client.register_api(ROBOT_API_ID_LOCO_SET_SWING_HEIGHT);
            client.register_api(ROBOT_API_ID_LOCO_SET_STAND_HEIGHT);
            client.register_api(ROBOT_API_ID_LOCO_SET_VELOCITY);
            client.register_api(ROBOT_API_ID_LOCO_SET_ARM_TASK);
            client.register_api(ROBOT_API_ID_LOCO_SET_SPEED_MODE);
            client.register_api(ROBOT_API_ID_LOCO_SET_PUNCH_API);
            client.register_api(ROBOT_API_ID_LOCO_SET_ARM_SDK_STATUS);
            Ok(Self { client })
        }
        /// api 7001 `GetFsmId` (C++ output param `fsm_id` is returned in the response data).
        pub async fn get_fsm_id(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_FSM_ID, "{}".into())
                .await
        }
        /// api 7002 `GetFsmMode` (C++ output param `fsm_mode` is returned in the response data).
        pub async fn get_fsm_mode(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_FSM_MODE, "{}".into())
                .await
        }
        /// api 7003 `GetBalanceMode` (C++ output param `balance_mode` is returned in the response data).
        pub async fn get_balance_mode(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_BALANCE_MODE, "{}".into())
                .await
        }
        /// api 7004 `GetSwingHeight` (C++ output param `swing_height` is returned in the response data).
        pub async fn get_swing_height(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_SWING_HEIGHT, "{}".into())
                .await
        }
        /// api 7005 `GetStandHeight` (C++ output param `stand_height` is returned in the response data).
        pub async fn get_stand_height(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_STAND_HEIGHT, "{}".into())
                .await
        }
        /// api 7006 `GetPhase` — deprecated in the SDK (the phase vector is returned in the response data).
        pub async fn get_phase(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_PHASE, "{}".into())
                .await
        }
        /// api 7007 `GetArmSdkStatus` (C++ output param `arm_sdk_status` is returned in the response data).
        pub async fn get_arm_sdk_status(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_ARM_SDK_STATUS, "{}".into())
                .await
        }
        /// api 7008 `GetAvailableFsmIds` (C++ output vectors `ids`/`names` are returned in the response data).
        pub async fn get_available_fsm_ids(&self) -> anyhow::Result<RpcResponse> {
            self.client
                .call(ROBOT_API_ID_LOCO_GET_AVAILABLE_FSM_IDS, "{}".into())
                .await
        }
        /// api 7101 `SetFsmId` -> {"data": fsm_id}
        pub async fn set_fsm_id(&self, fsm_id: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": fsm_id});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_FSM_ID, body.to_string())
                .await
        }
        /// api 7102 `SetBalanceMode` -> {"data": balance_mode}
        pub async fn set_balance_mode(&self, balance_mode: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": balance_mode});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_BALANCE_MODE, body.to_string())
                .await
        }
        /// api 7103 `SetSwingHeight` -> {"data": swing_height}
        pub async fn set_swing_height(&self, swing_height: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": swing_height});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_SWING_HEIGHT, body.to_string())
                .await
        }
        /// api 7104 `SetStandHeight` -> {"data": stand_height}
        pub async fn set_stand_height(&self, stand_height: f32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": stand_height});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_STAND_HEIGHT, body.to_string())
                .await
        }
        /// api 7105 `SetVelocity` -> {"velocity":[vx,vy,omega],"duration":duration}
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
        /// api 7108 `SetPunchApi` -> {"data": punch_api} (punch api as a float array)
        pub async fn set_punch_api(&self, punch_api: &[f32]) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": punch_api});
            self.client.call(ROBOT_API_ID_LOCO_SET_PUNCH_API, body.to_string()).await
        }
        /// api 7106 `SetTaskId` -> {"data": task_id}
        pub async fn set_task_id(&self, task_id: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": task_id});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_ARM_TASK, body.to_string())
                .await
        }
        /// api 7107 `SetSpeedMode` -> {"data": speed_mode}
        pub async fn set_speed_mode(&self, speed_mode: i32) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": speed_mode});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_SPEED_MODE, body.to_string())
                .await
        }
        /// api 7109 `SetArmSdkStatus` -> {"data": arm_sdk_status}
        pub async fn set_arm_sdk_status(
            &self,
            arm_sdk_status: bool,
        ) -> anyhow::Result<RpcResponse> {
            let body = serde_json::json!({"data": arm_sdk_status});
            self.client
                .call(ROBOT_API_ID_LOCO_SET_ARM_SDK_STATUS, body.to_string())
                .await
        }
        // Not exposed: the C++ "High Level API" wrappers below are client-local
        // conveniences with no API id of their own — they funnel into the RPCs
        // above with fixed values: Damp (set_fsm_id(1)), Start (601), Squat (2),
        // Sit (3), StandUp (4), ZeroTorque (0), StopMove (set_velocity(0,0,0)),
        // HighStand/LowStand (set_stand_height(±max)), Move (both overloads →
        // set_velocity), BalanceStand (set_balance_mode(0)), ContinuousGait
        // (set_balance_mode), SwitchMoveMode (client-local flag),
        // WaveHand/ShakeHand (set_task_id), EnableArmSDK/DisableArmSDK
        // (set_arm_sdk_status(true/false)).
    }
}
