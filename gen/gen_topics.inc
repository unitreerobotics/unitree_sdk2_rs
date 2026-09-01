// ⚡ Auto-generated (msg → C++). Regenerate by editing msgs/ or build.rs.

template class Subscriber<std_msgs::msg::dds_::String_>;
template class Publisher<std_msgs::msg::dds_::String_>;
std::unique_ptr<stdStringPublisher> new_std_string_publisher(rust::Str t) {
  return std::make_unique<stdStringPublisher>(std::string(t));
}
void publish_std_string_bytes(const stdStringPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<std_msgs::msg::dds_::String_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::AudioData_>;
template class Publisher<unitree_go::msg::dds_::AudioData_>;
std::unique_ptr<goAudioDataPublisher> new_go_audio_data_publisher(rust::Str t) {
  return std::make_unique<goAudioDataPublisher>(std::string(t));
}
void publish_go_audio_data_bytes(const goAudioDataPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::AudioData_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::BmsCmd_>;
template class Publisher<unitree_go::msg::dds_::BmsCmd_>;
std::unique_ptr<goBmsCmdPublisher> new_go_bms_cmd_publisher(rust::Str t) {
  return std::make_unique<goBmsCmdPublisher>(std::string(t));
}
void publish_go_bms_cmd_bytes(const goBmsCmdPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::BmsCmd_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::BmsState_>;
template class Publisher<unitree_go::msg::dds_::BmsState_>;
std::unique_ptr<goBmsStatePublisher> new_go_bms_state_publisher(rust::Str t) {
  return std::make_unique<goBmsStatePublisher>(std::string(t));
}
void publish_go_bms_state_bytes(const goBmsStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::BmsState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::Error_>;
template class Publisher<unitree_go::msg::dds_::Error_>;
std::unique_ptr<goErrorPublisher> new_go_error_publisher(rust::Str t) {
  return std::make_unique<goErrorPublisher>(std::string(t));
}
void publish_go_error_bytes(const goErrorPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::Error_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::Go2FrontVideoData_>;
template class Publisher<unitree_go::msg::dds_::Go2FrontVideoData_>;
std::unique_ptr<goGo2FrontVideoDataPublisher> new_go_go2_front_video_data_publisher(rust::Str t) {
  return std::make_unique<goGo2FrontVideoDataPublisher>(std::string(t));
}
void publish_go_go2_front_video_data_bytes(const goGo2FrontVideoDataPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::Go2FrontVideoData_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::HeightMap_>;
template class Publisher<unitree_go::msg::dds_::HeightMap_>;
std::unique_ptr<goHeightMapPublisher> new_go_height_map_publisher(rust::Str t) {
  return std::make_unique<goHeightMapPublisher>(std::string(t));
}
void publish_go_height_map_bytes(const goHeightMapPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::HeightMap_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::IMUState_>;
template class Publisher<unitree_go::msg::dds_::IMUState_>;
std::unique_ptr<goIMUStatePublisher> new_go_imu_state_publisher(rust::Str t) {
  return std::make_unique<goIMUStatePublisher>(std::string(t));
}
void publish_go_imu_state_bytes(const goIMUStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::IMUState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::InterfaceConfig_>;
template class Publisher<unitree_go::msg::dds_::InterfaceConfig_>;
std::unique_ptr<goInterfaceConfigPublisher> new_go_interface_config_publisher(rust::Str t) {
  return std::make_unique<goInterfaceConfigPublisher>(std::string(t));
}
void publish_go_interface_config_bytes(const goInterfaceConfigPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::InterfaceConfig_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::LidarState_>;
template class Publisher<unitree_go::msg::dds_::LidarState_>;
std::unique_ptr<goLidarStatePublisher> new_go_lidar_state_publisher(rust::Str t) {
  return std::make_unique<goLidarStatePublisher>(std::string(t));
}
void publish_go_lidar_state_bytes(const goLidarStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::LidarState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::LowCmd_>;
template class Publisher<unitree_go::msg::dds_::LowCmd_>;
std::unique_ptr<goLowCmdPublisher> new_go_low_cmd_publisher(rust::Str t) {
  return std::make_unique<goLowCmdPublisher>(std::string(t));
}
void publish_go_low_cmd_bytes(const goLowCmdPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::LowCmd_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::LowState_>;
template class Publisher<unitree_go::msg::dds_::LowState_>;
std::unique_ptr<goLowStatePublisher> new_go_low_state_publisher(rust::Str t) {
  return std::make_unique<goLowStatePublisher>(std::string(t));
}
void publish_go_low_state_bytes(const goLowStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::LowState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::MotorCmd_>;
template class Publisher<unitree_go::msg::dds_::MotorCmd_>;
std::unique_ptr<goMotorCmdPublisher> new_go_motor_cmd_publisher(rust::Str t) {
  return std::make_unique<goMotorCmdPublisher>(std::string(t));
}
void publish_go_motor_cmd_bytes(const goMotorCmdPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::MotorCmd_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::MotorCmds_>;
template class Publisher<unitree_go::msg::dds_::MotorCmds_>;
std::unique_ptr<goMotorCmdsPublisher> new_go_motor_cmds_publisher(rust::Str t) {
  return std::make_unique<goMotorCmdsPublisher>(std::string(t));
}
void publish_go_motor_cmds_bytes(const goMotorCmdsPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::MotorCmds_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::MotorState_>;
template class Publisher<unitree_go::msg::dds_::MotorState_>;
std::unique_ptr<goMotorStatePublisher> new_go_motor_state_publisher(rust::Str t) {
  return std::make_unique<goMotorStatePublisher>(std::string(t));
}
void publish_go_motor_state_bytes(const goMotorStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::MotorState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::MotorStates_>;
template class Publisher<unitree_go::msg::dds_::MotorStates_>;
std::unique_ptr<goMotorStatesPublisher> new_go_motor_states_publisher(rust::Str t) {
  return std::make_unique<goMotorStatesPublisher>(std::string(t));
}
void publish_go_motor_states_bytes(const goMotorStatesPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::MotorStates_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::PathPoint_>;
template class Publisher<unitree_go::msg::dds_::PathPoint_>;
std::unique_ptr<goPathPointPublisher> new_go_path_point_publisher(rust::Str t) {
  return std::make_unique<goPathPointPublisher>(std::string(t));
}
void publish_go_path_point_bytes(const goPathPointPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::PathPoint_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::Req_>;
template class Publisher<unitree_go::msg::dds_::Req_>;
std::unique_ptr<goReqPublisher> new_go_req_publisher(rust::Str t) {
  return std::make_unique<goReqPublisher>(std::string(t));
}
void publish_go_req_bytes(const goReqPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::Req_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::Res_>;
template class Publisher<unitree_go::msg::dds_::Res_>;
std::unique_ptr<goResPublisher> new_go_res_publisher(rust::Str t) {
  return std::make_unique<goResPublisher>(std::string(t));
}
void publish_go_res_bytes(const goResPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::Res_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::SportModeCmd_>;
template class Publisher<unitree_go::msg::dds_::SportModeCmd_>;
std::unique_ptr<goSportModeCmdPublisher> new_go_sport_mode_cmd_publisher(rust::Str t) {
  return std::make_unique<goSportModeCmdPublisher>(std::string(t));
}
void publish_go_sport_mode_cmd_bytes(const goSportModeCmdPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::SportModeCmd_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::SportModeState_>;
template class Publisher<unitree_go::msg::dds_::SportModeState_>;
std::unique_ptr<goSportModeStatePublisher> new_go_sport_mode_state_publisher(rust::Str t) {
  return std::make_unique<goSportModeStatePublisher>(std::string(t));
}
void publish_go_sport_mode_state_bytes(const goSportModeStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::SportModeState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::TimeSpec_>;
template class Publisher<unitree_go::msg::dds_::TimeSpec_>;
std::unique_ptr<goTimeSpecPublisher> new_go_time_spec_publisher(rust::Str t) {
  return std::make_unique<goTimeSpecPublisher>(std::string(t));
}
void publish_go_time_spec_bytes(const goTimeSpecPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::TimeSpec_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::UwbState_>;
template class Publisher<unitree_go::msg::dds_::UwbState_>;
std::unique_ptr<goUwbStatePublisher> new_go_uwb_state_publisher(rust::Str t) {
  return std::make_unique<goUwbStatePublisher>(std::string(t));
}
void publish_go_uwb_state_bytes(const goUwbStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::UwbState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::UwbSwitch_>;
template class Publisher<unitree_go::msg::dds_::UwbSwitch_>;
std::unique_ptr<goUwbSwitchPublisher> new_go_uwb_switch_publisher(rust::Str t) {
  return std::make_unique<goUwbSwitchPublisher>(std::string(t));
}
void publish_go_uwb_switch_bytes(const goUwbSwitchPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::UwbSwitch_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_go::msg::dds_::WirelessController_>;
template class Publisher<unitree_go::msg::dds_::WirelessController_>;
std::unique_ptr<goWirelessControllerPublisher> new_go_wireless_controller_publisher(rust::Str t) {
  return std::make_unique<goWirelessControllerPublisher>(std::string(t));
}
void publish_go_wireless_controller_bytes(const goWirelessControllerPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_go::msg::dds_::WirelessController_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_hg::msg::dds_::BmsCmd_>;
template class Publisher<unitree_hg::msg::dds_::BmsCmd_>;
std::unique_ptr<hgBmsCmdPublisher> new_hg_bms_cmd_publisher(rust::Str t) {
  return std::make_unique<hgBmsCmdPublisher>(std::string(t));
}
void publish_hg_bms_cmd_bytes(const hgBmsCmdPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_hg::msg::dds_::BmsCmd_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_hg::msg::dds_::BmsState_>;
template class Publisher<unitree_hg::msg::dds_::BmsState_>;
std::unique_ptr<hgBmsStatePublisher> new_hg_bms_state_publisher(rust::Str t) {
  return std::make_unique<hgBmsStatePublisher>(std::string(t));
}
void publish_hg_bms_state_bytes(const hgBmsStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_hg::msg::dds_::BmsState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_hg::msg::dds_::HandCmd_>;
template class Publisher<unitree_hg::msg::dds_::HandCmd_>;
std::unique_ptr<hgHandCmdPublisher> new_hg_hand_cmd_publisher(rust::Str t) {
  return std::make_unique<hgHandCmdPublisher>(std::string(t));
}
void publish_hg_hand_cmd_bytes(const hgHandCmdPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_hg::msg::dds_::HandCmd_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_hg::msg::dds_::HandState_>;
template class Publisher<unitree_hg::msg::dds_::HandState_>;
std::unique_ptr<hgHandStatePublisher> new_hg_hand_state_publisher(rust::Str t) {
  return std::make_unique<hgHandStatePublisher>(std::string(t));
}
void publish_hg_hand_state_bytes(const hgHandStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_hg::msg::dds_::HandState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_hg::msg::dds_::IMUState_>;
template class Publisher<unitree_hg::msg::dds_::IMUState_>;
std::unique_ptr<hgIMUStatePublisher> new_hg_imu_state_publisher(rust::Str t) {
  return std::make_unique<hgIMUStatePublisher>(std::string(t));
}
void publish_hg_imu_state_bytes(const hgIMUStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_hg::msg::dds_::IMUState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_hg::msg::dds_::LowCmd_>;
template class Publisher<unitree_hg::msg::dds_::LowCmd_>;
std::unique_ptr<hgLowCmdPublisher> new_hg_low_cmd_publisher(rust::Str t) {
  return std::make_unique<hgLowCmdPublisher>(std::string(t));
}
void publish_hg_low_cmd_bytes(const hgLowCmdPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_hg::msg::dds_::LowCmd_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_hg::msg::dds_::LowState_>;
template class Publisher<unitree_hg::msg::dds_::LowState_>;
std::unique_ptr<hgLowStatePublisher> new_hg_low_state_publisher(rust::Str t) {
  return std::make_unique<hgLowStatePublisher>(std::string(t));
}
void publish_hg_low_state_bytes(const hgLowStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_hg::msg::dds_::LowState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_hg::msg::dds_::MainBoardState_>;
template class Publisher<unitree_hg::msg::dds_::MainBoardState_>;
std::unique_ptr<hgMainBoardStatePublisher> new_hg_main_board_state_publisher(rust::Str t) {
  return std::make_unique<hgMainBoardStatePublisher>(std::string(t));
}
void publish_hg_main_board_state_bytes(const hgMainBoardStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_hg::msg::dds_::MainBoardState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_hg::msg::dds_::MotorCmd_>;
template class Publisher<unitree_hg::msg::dds_::MotorCmd_>;
std::unique_ptr<hgMotorCmdPublisher> new_hg_motor_cmd_publisher(rust::Str t) {
  return std::make_unique<hgMotorCmdPublisher>(std::string(t));
}
void publish_hg_motor_cmd_bytes(const hgMotorCmdPublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_hg::msg::dds_::MotorCmd_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_hg::msg::dds_::MotorState_>;
template class Publisher<unitree_hg::msg::dds_::MotorState_>;
std::unique_ptr<hgMotorStatePublisher> new_hg_motor_state_publisher(rust::Str t) {
  return std::make_unique<hgMotorStatePublisher>(std::string(t));
}
void publish_hg_motor_state_bytes(const hgMotorStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_hg::msg::dds_::MotorState_>(to_std_vec(bytes));
  pub.write(msg);
}

template class Subscriber<unitree_hg::msg::dds_::PressSensorState_>;
template class Publisher<unitree_hg::msg::dds_::PressSensorState_>;
std::unique_ptr<hgPressSensorStatePublisher> new_hg_press_sensor_state_publisher(rust::Str t) {
  return std::make_unique<hgPressSensorStatePublisher>(std::string(t));
}
void publish_hg_press_sensor_state_bytes(const hgPressSensorStatePublisher& pub, rust::Vec<uint8_t> bytes) {
  auto msg = bytes_to_dds<unitree_hg::msg::dds_::PressSensorState_>(to_std_vec(bytes));
  pub.write(msg);
}

std::unique_ptr<SubscriberBase> make_subscriber(
    const std::string& topic, const std::string& type_name,
    std::function<void(rust::Vec<uint8_t>)> cb) {
  if (type_name == "std_msgs::msg::dds_::String_")
   return std::make_unique<Subscriber<std_msgs::msg::dds_::String_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::AudioData_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::AudioData_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::BmsCmd_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::BmsCmd_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::BmsState_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::BmsState_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::Error_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::Error_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::Go2FrontVideoData_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::Go2FrontVideoData_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::HeightMap_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::HeightMap_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::IMUState_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::IMUState_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::InterfaceConfig_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::InterfaceConfig_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::LidarState_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::LidarState_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::LowCmd_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::LowCmd_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::LowState_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::LowState_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::MotorCmd_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::MotorCmd_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::MotorCmds_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::MotorCmds_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::MotorState_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::MotorState_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::MotorStates_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::MotorStates_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::PathPoint_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::PathPoint_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::Req_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::Req_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::Res_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::Res_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::SportModeCmd_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::SportModeCmd_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::SportModeState_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::SportModeState_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::TimeSpec_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::TimeSpec_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::UwbState_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::UwbState_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::UwbSwitch_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::UwbSwitch_>>(topic, cb);
  if (type_name == "unitree_go::msg::dds_::WirelessController_")
   return std::make_unique<Subscriber<unitree_go::msg::dds_::WirelessController_>>(topic, cb);
  if (type_name == "unitree_hg::msg::dds_::BmsCmd_")
   return std::make_unique<Subscriber<unitree_hg::msg::dds_::BmsCmd_>>(topic, cb);
  if (type_name == "unitree_hg::msg::dds_::BmsState_")
   return std::make_unique<Subscriber<unitree_hg::msg::dds_::BmsState_>>(topic, cb);
  if (type_name == "unitree_hg::msg::dds_::HandCmd_")
   return std::make_unique<Subscriber<unitree_hg::msg::dds_::HandCmd_>>(topic, cb);
  if (type_name == "unitree_hg::msg::dds_::HandState_")
   return std::make_unique<Subscriber<unitree_hg::msg::dds_::HandState_>>(topic, cb);
  if (type_name == "unitree_hg::msg::dds_::IMUState_")
   return std::make_unique<Subscriber<unitree_hg::msg::dds_::IMUState_>>(topic, cb);
  if (type_name == "unitree_hg::msg::dds_::LowCmd_")
   return std::make_unique<Subscriber<unitree_hg::msg::dds_::LowCmd_>>(topic, cb);
  if (type_name == "unitree_hg::msg::dds_::LowState_")
   return std::make_unique<Subscriber<unitree_hg::msg::dds_::LowState_>>(topic, cb);
  if (type_name == "unitree_hg::msg::dds_::MainBoardState_")
   return std::make_unique<Subscriber<unitree_hg::msg::dds_::MainBoardState_>>(topic, cb);
  if (type_name == "unitree_hg::msg::dds_::MotorCmd_")
   return std::make_unique<Subscriber<unitree_hg::msg::dds_::MotorCmd_>>(topic, cb);
  if (type_name == "unitree_hg::msg::dds_::MotorState_")
   return std::make_unique<Subscriber<unitree_hg::msg::dds_::MotorState_>>(topic, cb);
  if (type_name == "unitree_hg::msg::dds_::PressSensorState_")
   return std::make_unique<Subscriber<unitree_hg::msg::dds_::PressSensorState_>>(topic, cb);
  return nullptr;
}
