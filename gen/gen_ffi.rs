// ⚡ Auto-generated (msg → Rust FFI). Regenerate by editing msgs/ or build.rs.

pub struct ByteHandler {
    cb: Box<dyn Fn(Vec<u8>) + Send>,
}
impl ByteHandler {
    pub fn new(cb: impl Fn(Vec<u8>) + Send + 'static) -> Self { Self { cb: Box::new(cb) } }
    pub(crate) fn on_bytes(&self, bytes: Vec<u8>) { (self.cb)(bytes); }
}

#[cxx::bridge(namespace = "unitree")]
pub mod ffi {
    extern "Rust" {
        type ByteHandler;
        fn on_bytes(self: &ByteHandler, bytes: Vec<u8>);
    }
    unsafe extern "C++" {
        include!("dds_bridge.h");
        fn boot_dds(domain_id: i32, network_interface: &str, config_file: &str);
        fn subscribe_any(topic: &str, type_name: &str, handler: Box<ByteHandler>) -> i32;
        fn unsubscribe(id: i32);
        type stdStringPublisher;
        fn new_std_string_publisher(topic: &str) -> UniquePtr<stdStringPublisher>;
        fn publish_std_string_bytes(p: &stdStringPublisher, bytes: Vec<u8>);
        type goAudioDataPublisher;
        fn new_go_audio_data_publisher(topic: &str) -> UniquePtr<goAudioDataPublisher>;
        fn publish_go_audio_data_bytes(p: &goAudioDataPublisher, bytes: Vec<u8>);
        type goBmsCmdPublisher;
        fn new_go_bms_cmd_publisher(topic: &str) -> UniquePtr<goBmsCmdPublisher>;
        fn publish_go_bms_cmd_bytes(p: &goBmsCmdPublisher, bytes: Vec<u8>);
        type goBmsStatePublisher;
        fn new_go_bms_state_publisher(topic: &str) -> UniquePtr<goBmsStatePublisher>;
        fn publish_go_bms_state_bytes(p: &goBmsStatePublisher, bytes: Vec<u8>);
        type goErrorPublisher;
        fn new_go_error_publisher(topic: &str) -> UniquePtr<goErrorPublisher>;
        fn publish_go_error_bytes(p: &goErrorPublisher, bytes: Vec<u8>);
        type goGo2FrontVideoDataPublisher;
        fn new_go_go2_front_video_data_publisher(topic: &str) -> UniquePtr<goGo2FrontVideoDataPublisher>;
        fn publish_go_go2_front_video_data_bytes(p: &goGo2FrontVideoDataPublisher, bytes: Vec<u8>);
        type goHeightMapPublisher;
        fn new_go_height_map_publisher(topic: &str) -> UniquePtr<goHeightMapPublisher>;
        fn publish_go_height_map_bytes(p: &goHeightMapPublisher, bytes: Vec<u8>);
        type goIMUStatePublisher;
        fn new_go_imu_state_publisher(topic: &str) -> UniquePtr<goIMUStatePublisher>;
        fn publish_go_imu_state_bytes(p: &goIMUStatePublisher, bytes: Vec<u8>);
        type goInterfaceConfigPublisher;
        fn new_go_interface_config_publisher(topic: &str) -> UniquePtr<goInterfaceConfigPublisher>;
        fn publish_go_interface_config_bytes(p: &goInterfaceConfigPublisher, bytes: Vec<u8>);
        type goLidarStatePublisher;
        fn new_go_lidar_state_publisher(topic: &str) -> UniquePtr<goLidarStatePublisher>;
        fn publish_go_lidar_state_bytes(p: &goLidarStatePublisher, bytes: Vec<u8>);
        type goLowCmdPublisher;
        fn new_go_low_cmd_publisher(topic: &str) -> UniquePtr<goLowCmdPublisher>;
        fn publish_go_low_cmd_bytes(p: &goLowCmdPublisher, bytes: Vec<u8>);
        type goLowStatePublisher;
        fn new_go_low_state_publisher(topic: &str) -> UniquePtr<goLowStatePublisher>;
        fn publish_go_low_state_bytes(p: &goLowStatePublisher, bytes: Vec<u8>);
        type goMotorCmdPublisher;
        fn new_go_motor_cmd_publisher(topic: &str) -> UniquePtr<goMotorCmdPublisher>;
        fn publish_go_motor_cmd_bytes(p: &goMotorCmdPublisher, bytes: Vec<u8>);
        type goMotorCmdsPublisher;
        fn new_go_motor_cmds_publisher(topic: &str) -> UniquePtr<goMotorCmdsPublisher>;
        fn publish_go_motor_cmds_bytes(p: &goMotorCmdsPublisher, bytes: Vec<u8>);
        type goMotorStatePublisher;
        fn new_go_motor_state_publisher(topic: &str) -> UniquePtr<goMotorStatePublisher>;
        fn publish_go_motor_state_bytes(p: &goMotorStatePublisher, bytes: Vec<u8>);
        type goMotorStatesPublisher;
        fn new_go_motor_states_publisher(topic: &str) -> UniquePtr<goMotorStatesPublisher>;
        fn publish_go_motor_states_bytes(p: &goMotorStatesPublisher, bytes: Vec<u8>);
        type goPathPointPublisher;
        fn new_go_path_point_publisher(topic: &str) -> UniquePtr<goPathPointPublisher>;
        fn publish_go_path_point_bytes(p: &goPathPointPublisher, bytes: Vec<u8>);
        type goReqPublisher;
        fn new_go_req_publisher(topic: &str) -> UniquePtr<goReqPublisher>;
        fn publish_go_req_bytes(p: &goReqPublisher, bytes: Vec<u8>);
        type goResPublisher;
        fn new_go_res_publisher(topic: &str) -> UniquePtr<goResPublisher>;
        fn publish_go_res_bytes(p: &goResPublisher, bytes: Vec<u8>);
        type goSportModeCmdPublisher;
        fn new_go_sport_mode_cmd_publisher(topic: &str) -> UniquePtr<goSportModeCmdPublisher>;
        fn publish_go_sport_mode_cmd_bytes(p: &goSportModeCmdPublisher, bytes: Vec<u8>);
        type goSportModeStatePublisher;
        fn new_go_sport_mode_state_publisher(topic: &str) -> UniquePtr<goSportModeStatePublisher>;
        fn publish_go_sport_mode_state_bytes(p: &goSportModeStatePublisher, bytes: Vec<u8>);
        type goTimeSpecPublisher;
        fn new_go_time_spec_publisher(topic: &str) -> UniquePtr<goTimeSpecPublisher>;
        fn publish_go_time_spec_bytes(p: &goTimeSpecPublisher, bytes: Vec<u8>);
        type goUwbStatePublisher;
        fn new_go_uwb_state_publisher(topic: &str) -> UniquePtr<goUwbStatePublisher>;
        fn publish_go_uwb_state_bytes(p: &goUwbStatePublisher, bytes: Vec<u8>);
        type goUwbSwitchPublisher;
        fn new_go_uwb_switch_publisher(topic: &str) -> UniquePtr<goUwbSwitchPublisher>;
        fn publish_go_uwb_switch_bytes(p: &goUwbSwitchPublisher, bytes: Vec<u8>);
        type goWirelessControllerPublisher;
        fn new_go_wireless_controller_publisher(topic: &str) -> UniquePtr<goWirelessControllerPublisher>;
        fn publish_go_wireless_controller_bytes(p: &goWirelessControllerPublisher, bytes: Vec<u8>);
        type hgBmsCmdPublisher;
        fn new_hg_bms_cmd_publisher(topic: &str) -> UniquePtr<hgBmsCmdPublisher>;
        fn publish_hg_bms_cmd_bytes(p: &hgBmsCmdPublisher, bytes: Vec<u8>);
        type hgBmsStatePublisher;
        fn new_hg_bms_state_publisher(topic: &str) -> UniquePtr<hgBmsStatePublisher>;
        fn publish_hg_bms_state_bytes(p: &hgBmsStatePublisher, bytes: Vec<u8>);
        type hgHandCmdPublisher;
        fn new_hg_hand_cmd_publisher(topic: &str) -> UniquePtr<hgHandCmdPublisher>;
        fn publish_hg_hand_cmd_bytes(p: &hgHandCmdPublisher, bytes: Vec<u8>);
        type hgHandStatePublisher;
        fn new_hg_hand_state_publisher(topic: &str) -> UniquePtr<hgHandStatePublisher>;
        fn publish_hg_hand_state_bytes(p: &hgHandStatePublisher, bytes: Vec<u8>);
        type hgIMUStatePublisher;
        fn new_hg_imu_state_publisher(topic: &str) -> UniquePtr<hgIMUStatePublisher>;
        fn publish_hg_imu_state_bytes(p: &hgIMUStatePublisher, bytes: Vec<u8>);
        type hgLowCmdPublisher;
        fn new_hg_low_cmd_publisher(topic: &str) -> UniquePtr<hgLowCmdPublisher>;
        fn publish_hg_low_cmd_bytes(p: &hgLowCmdPublisher, bytes: Vec<u8>);
        type hgLowStatePublisher;
        fn new_hg_low_state_publisher(topic: &str) -> UniquePtr<hgLowStatePublisher>;
        fn publish_hg_low_state_bytes(p: &hgLowStatePublisher, bytes: Vec<u8>);
        type hgMainBoardStatePublisher;
        fn new_hg_main_board_state_publisher(topic: &str) -> UniquePtr<hgMainBoardStatePublisher>;
        fn publish_hg_main_board_state_bytes(p: &hgMainBoardStatePublisher, bytes: Vec<u8>);
        type hgMotorCmdPublisher;
        fn new_hg_motor_cmd_publisher(topic: &str) -> UniquePtr<hgMotorCmdPublisher>;
        fn publish_hg_motor_cmd_bytes(p: &hgMotorCmdPublisher, bytes: Vec<u8>);
        type hgMotorStatePublisher;
        fn new_hg_motor_state_publisher(topic: &str) -> UniquePtr<hgMotorStatePublisher>;
        fn publish_hg_motor_state_bytes(p: &hgMotorStatePublisher, bytes: Vec<u8>);
        type hgPressSensorStatePublisher;
        fn new_hg_press_sensor_state_publisher(topic: &str) -> UniquePtr<hgPressSensorStatePublisher>;
        fn publish_hg_press_sensor_state_bytes(p: &hgPressSensorStatePublisher, bytes: Vec<u8>);
    }
}
