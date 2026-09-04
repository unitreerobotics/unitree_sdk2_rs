// ⚡ Auto-generated (msg → C++). Regenerate by editing msgs/ or build.rs.
#pragma once

#include <unitree/idl/ros2/String_.hpp>
#include <unitree/robot/internal/internal_idl_decl/Request_.hpp>
#include <unitree/robot/internal/internal_idl_decl/RequestHeader_.hpp>
#include <unitree/robot/internal/internal_idl_decl/RequestIdentity_.hpp>
#include <unitree/robot/internal/internal_idl_decl/RequestLease_.hpp>
#include <unitree/robot/internal/internal_idl_decl/RequestPolicy_.hpp>
#include <unitree/robot/internal/internal_idl_decl/Response_.hpp>
#include <unitree/robot/internal/internal_idl_decl/ResponseHeader_.hpp>
#include <unitree/robot/internal/internal_idl_decl/ResponseStatus_.hpp>
#include <unitree/idl/go2/AudioData_.hpp>
#include <unitree/idl/go2/BmsCmd_.hpp>
#include <unitree/idl/go2/BmsState_.hpp>
#include <unitree/idl/go2/Error_.hpp>
#include <unitree/idl/go2/Go2FrontVideoData_.hpp>
#include <unitree/idl/go2/HeightMap_.hpp>
#include <unitree/idl/go2/IMUState_.hpp>
#include <unitree/idl/go2/InterfaceConfig_.hpp>
#include <unitree/idl/go2/LidarState_.hpp>
#include <unitree/idl/go2/LowCmd_.hpp>
#include <unitree/idl/go2/LowState_.hpp>
#include <unitree/idl/go2/MotorCmd_.hpp>
#include <unitree/idl/go2/MotorCmds_.hpp>
#include <unitree/idl/go2/MotorState_.hpp>
#include <unitree/idl/go2/MotorStates_.hpp>
#include <unitree/idl/go2/PathPoint_.hpp>
#include <unitree/idl/go2/Req_.hpp>
#include <unitree/idl/go2/Res_.hpp>
#include <unitree/idl/go2/SportModeCmd_.hpp>
#include <unitree/idl/go2/SportModeState_.hpp>
#include <unitree/idl/go2/TimeSpec_.hpp>
#include <unitree/idl/go2/UwbState_.hpp>
#include <unitree/idl/go2/UwbSwitch_.hpp>
#include <unitree/idl/go2/WirelessController_.hpp>
#include <unitree/idl/hg/BmsCmd_.hpp>
#include <unitree/idl/hg/BmsState_.hpp>
#include <unitree/idl/hg/HandCmd_.hpp>
#include <unitree/idl/hg/HandState_.hpp>
#include <unitree/idl/hg/IMUState_.hpp>
#include <unitree/idl/hg/LowCmd_.hpp>
#include <unitree/idl/hg/LowState_.hpp>
#include <unitree/idl/hg/MainBoardState_.hpp>
#include <unitree/idl/hg/MotorCmd_.hpp>
#include <unitree/idl/hg/MotorState_.hpp>
#include <unitree/idl/hg/PressSensorState_.hpp>

namespace unitree {

using stdStringPublisher = Publisher<std_msgs::msg::dds_::String_>;
std::unique_ptr<stdStringPublisher> new_std_string_publisher(rust::Str topic);
void publish_std_string_bytes(const stdStringPublisher& pub, rust::Vec<uint8_t> bytes);

using apiRequestPublisher = Publisher<unitree_api::msg::dds_::Request_>;
std::unique_ptr<apiRequestPublisher> new_api_request_publisher(rust::Str topic);
void publish_api_request_bytes(const apiRequestPublisher& pub, rust::Vec<uint8_t> bytes);

using apiRequestHeaderPublisher = Publisher<unitree_api::msg::dds_::RequestHeader_>;
std::unique_ptr<apiRequestHeaderPublisher> new_api_request_header_publisher(rust::Str topic);
void publish_api_request_header_bytes(const apiRequestHeaderPublisher& pub, rust::Vec<uint8_t> bytes);

using apiRequestIdentityPublisher = Publisher<unitree_api::msg::dds_::RequestIdentity_>;
std::unique_ptr<apiRequestIdentityPublisher> new_api_request_identity_publisher(rust::Str topic);
void publish_api_request_identity_bytes(const apiRequestIdentityPublisher& pub, rust::Vec<uint8_t> bytes);

using apiRequestLeasePublisher = Publisher<unitree_api::msg::dds_::RequestLease_>;
std::unique_ptr<apiRequestLeasePublisher> new_api_request_lease_publisher(rust::Str topic);
void publish_api_request_lease_bytes(const apiRequestLeasePublisher& pub, rust::Vec<uint8_t> bytes);

using apiRequestPolicyPublisher = Publisher<unitree_api::msg::dds_::RequestPolicy_>;
std::unique_ptr<apiRequestPolicyPublisher> new_api_request_policy_publisher(rust::Str topic);
void publish_api_request_policy_bytes(const apiRequestPolicyPublisher& pub, rust::Vec<uint8_t> bytes);

using apiResponsePublisher = Publisher<unitree_api::msg::dds_::Response_>;
std::unique_ptr<apiResponsePublisher> new_api_response_publisher(rust::Str topic);
void publish_api_response_bytes(const apiResponsePublisher& pub, rust::Vec<uint8_t> bytes);

using apiResponseHeaderPublisher = Publisher<unitree_api::msg::dds_::ResponseHeader_>;
std::unique_ptr<apiResponseHeaderPublisher> new_api_response_header_publisher(rust::Str topic);
void publish_api_response_header_bytes(const apiResponseHeaderPublisher& pub, rust::Vec<uint8_t> bytes);

using apiResponseStatusPublisher = Publisher<unitree_api::msg::dds_::ResponseStatus_>;
std::unique_ptr<apiResponseStatusPublisher> new_api_response_status_publisher(rust::Str topic);
void publish_api_response_status_bytes(const apiResponseStatusPublisher& pub, rust::Vec<uint8_t> bytes);

using goAudioDataPublisher = Publisher<unitree_go::msg::dds_::AudioData_>;
std::unique_ptr<goAudioDataPublisher> new_go_audio_data_publisher(rust::Str topic);
void publish_go_audio_data_bytes(const goAudioDataPublisher& pub, rust::Vec<uint8_t> bytes);

using goBmsCmdPublisher = Publisher<unitree_go::msg::dds_::BmsCmd_>;
std::unique_ptr<goBmsCmdPublisher> new_go_bms_cmd_publisher(rust::Str topic);
void publish_go_bms_cmd_bytes(const goBmsCmdPublisher& pub, rust::Vec<uint8_t> bytes);

using goBmsStatePublisher = Publisher<unitree_go::msg::dds_::BmsState_>;
std::unique_ptr<goBmsStatePublisher> new_go_bms_state_publisher(rust::Str topic);
void publish_go_bms_state_bytes(const goBmsStatePublisher& pub, rust::Vec<uint8_t> bytes);

using goErrorPublisher = Publisher<unitree_go::msg::dds_::Error_>;
std::unique_ptr<goErrorPublisher> new_go_error_publisher(rust::Str topic);
void publish_go_error_bytes(const goErrorPublisher& pub, rust::Vec<uint8_t> bytes);

using goGo2FrontVideoDataPublisher = Publisher<unitree_go::msg::dds_::Go2FrontVideoData_>;
std::unique_ptr<goGo2FrontVideoDataPublisher> new_go_go2_front_video_data_publisher(rust::Str topic);
void publish_go_go2_front_video_data_bytes(const goGo2FrontVideoDataPublisher& pub, rust::Vec<uint8_t> bytes);

using goHeightMapPublisher = Publisher<unitree_go::msg::dds_::HeightMap_>;
std::unique_ptr<goHeightMapPublisher> new_go_height_map_publisher(rust::Str topic);
void publish_go_height_map_bytes(const goHeightMapPublisher& pub, rust::Vec<uint8_t> bytes);

using goIMUStatePublisher = Publisher<unitree_go::msg::dds_::IMUState_>;
std::unique_ptr<goIMUStatePublisher> new_go_imu_state_publisher(rust::Str topic);
void publish_go_imu_state_bytes(const goIMUStatePublisher& pub, rust::Vec<uint8_t> bytes);

using goInterfaceConfigPublisher = Publisher<unitree_go::msg::dds_::InterfaceConfig_>;
std::unique_ptr<goInterfaceConfigPublisher> new_go_interface_config_publisher(rust::Str topic);
void publish_go_interface_config_bytes(const goInterfaceConfigPublisher& pub, rust::Vec<uint8_t> bytes);

using goLidarStatePublisher = Publisher<unitree_go::msg::dds_::LidarState_>;
std::unique_ptr<goLidarStatePublisher> new_go_lidar_state_publisher(rust::Str topic);
void publish_go_lidar_state_bytes(const goLidarStatePublisher& pub, rust::Vec<uint8_t> bytes);

using goLowCmdPublisher = Publisher<unitree_go::msg::dds_::LowCmd_>;
std::unique_ptr<goLowCmdPublisher> new_go_low_cmd_publisher(rust::Str topic);
void publish_go_low_cmd_bytes(const goLowCmdPublisher& pub, rust::Vec<uint8_t> bytes);

using goLowStatePublisher = Publisher<unitree_go::msg::dds_::LowState_>;
std::unique_ptr<goLowStatePublisher> new_go_low_state_publisher(rust::Str topic);
void publish_go_low_state_bytes(const goLowStatePublisher& pub, rust::Vec<uint8_t> bytes);

using goMotorCmdPublisher = Publisher<unitree_go::msg::dds_::MotorCmd_>;
std::unique_ptr<goMotorCmdPublisher> new_go_motor_cmd_publisher(rust::Str topic);
void publish_go_motor_cmd_bytes(const goMotorCmdPublisher& pub, rust::Vec<uint8_t> bytes);

using goMotorCmdsPublisher = Publisher<unitree_go::msg::dds_::MotorCmds_>;
std::unique_ptr<goMotorCmdsPublisher> new_go_motor_cmds_publisher(rust::Str topic);
void publish_go_motor_cmds_bytes(const goMotorCmdsPublisher& pub, rust::Vec<uint8_t> bytes);

using goMotorStatePublisher = Publisher<unitree_go::msg::dds_::MotorState_>;
std::unique_ptr<goMotorStatePublisher> new_go_motor_state_publisher(rust::Str topic);
void publish_go_motor_state_bytes(const goMotorStatePublisher& pub, rust::Vec<uint8_t> bytes);

using goMotorStatesPublisher = Publisher<unitree_go::msg::dds_::MotorStates_>;
std::unique_ptr<goMotorStatesPublisher> new_go_motor_states_publisher(rust::Str topic);
void publish_go_motor_states_bytes(const goMotorStatesPublisher& pub, rust::Vec<uint8_t> bytes);

using goPathPointPublisher = Publisher<unitree_go::msg::dds_::PathPoint_>;
std::unique_ptr<goPathPointPublisher> new_go_path_point_publisher(rust::Str topic);
void publish_go_path_point_bytes(const goPathPointPublisher& pub, rust::Vec<uint8_t> bytes);

using goReqPublisher = Publisher<unitree_go::msg::dds_::Req_>;
std::unique_ptr<goReqPublisher> new_go_req_publisher(rust::Str topic);
void publish_go_req_bytes(const goReqPublisher& pub, rust::Vec<uint8_t> bytes);

using goResPublisher = Publisher<unitree_go::msg::dds_::Res_>;
std::unique_ptr<goResPublisher> new_go_res_publisher(rust::Str topic);
void publish_go_res_bytes(const goResPublisher& pub, rust::Vec<uint8_t> bytes);

using goSportModeCmdPublisher = Publisher<unitree_go::msg::dds_::SportModeCmd_>;
std::unique_ptr<goSportModeCmdPublisher> new_go_sport_mode_cmd_publisher(rust::Str topic);
void publish_go_sport_mode_cmd_bytes(const goSportModeCmdPublisher& pub, rust::Vec<uint8_t> bytes);

using goSportModeStatePublisher = Publisher<unitree_go::msg::dds_::SportModeState_>;
std::unique_ptr<goSportModeStatePublisher> new_go_sport_mode_state_publisher(rust::Str topic);
void publish_go_sport_mode_state_bytes(const goSportModeStatePublisher& pub, rust::Vec<uint8_t> bytes);

using goTimeSpecPublisher = Publisher<unitree_go::msg::dds_::TimeSpec_>;
std::unique_ptr<goTimeSpecPublisher> new_go_time_spec_publisher(rust::Str topic);
void publish_go_time_spec_bytes(const goTimeSpecPublisher& pub, rust::Vec<uint8_t> bytes);

using goUwbStatePublisher = Publisher<unitree_go::msg::dds_::UwbState_>;
std::unique_ptr<goUwbStatePublisher> new_go_uwb_state_publisher(rust::Str topic);
void publish_go_uwb_state_bytes(const goUwbStatePublisher& pub, rust::Vec<uint8_t> bytes);

using goUwbSwitchPublisher = Publisher<unitree_go::msg::dds_::UwbSwitch_>;
std::unique_ptr<goUwbSwitchPublisher> new_go_uwb_switch_publisher(rust::Str topic);
void publish_go_uwb_switch_bytes(const goUwbSwitchPublisher& pub, rust::Vec<uint8_t> bytes);

using goWirelessControllerPublisher = Publisher<unitree_go::msg::dds_::WirelessController_>;
std::unique_ptr<goWirelessControllerPublisher> new_go_wireless_controller_publisher(rust::Str topic);
void publish_go_wireless_controller_bytes(const goWirelessControllerPublisher& pub, rust::Vec<uint8_t> bytes);

using hgBmsCmdPublisher = Publisher<unitree_hg::msg::dds_::BmsCmd_>;
std::unique_ptr<hgBmsCmdPublisher> new_hg_bms_cmd_publisher(rust::Str topic);
void publish_hg_bms_cmd_bytes(const hgBmsCmdPublisher& pub, rust::Vec<uint8_t> bytes);

using hgBmsStatePublisher = Publisher<unitree_hg::msg::dds_::BmsState_>;
std::unique_ptr<hgBmsStatePublisher> new_hg_bms_state_publisher(rust::Str topic);
void publish_hg_bms_state_bytes(const hgBmsStatePublisher& pub, rust::Vec<uint8_t> bytes);

using hgHandCmdPublisher = Publisher<unitree_hg::msg::dds_::HandCmd_>;
std::unique_ptr<hgHandCmdPublisher> new_hg_hand_cmd_publisher(rust::Str topic);
void publish_hg_hand_cmd_bytes(const hgHandCmdPublisher& pub, rust::Vec<uint8_t> bytes);

using hgHandStatePublisher = Publisher<unitree_hg::msg::dds_::HandState_>;
std::unique_ptr<hgHandStatePublisher> new_hg_hand_state_publisher(rust::Str topic);
void publish_hg_hand_state_bytes(const hgHandStatePublisher& pub, rust::Vec<uint8_t> bytes);

using hgIMUStatePublisher = Publisher<unitree_hg::msg::dds_::IMUState_>;
std::unique_ptr<hgIMUStatePublisher> new_hg_imu_state_publisher(rust::Str topic);
void publish_hg_imu_state_bytes(const hgIMUStatePublisher& pub, rust::Vec<uint8_t> bytes);

using hgLowCmdPublisher = Publisher<unitree_hg::msg::dds_::LowCmd_>;
std::unique_ptr<hgLowCmdPublisher> new_hg_low_cmd_publisher(rust::Str topic);
void publish_hg_low_cmd_bytes(const hgLowCmdPublisher& pub, rust::Vec<uint8_t> bytes);

using hgLowStatePublisher = Publisher<unitree_hg::msg::dds_::LowState_>;
std::unique_ptr<hgLowStatePublisher> new_hg_low_state_publisher(rust::Str topic);
void publish_hg_low_state_bytes(const hgLowStatePublisher& pub, rust::Vec<uint8_t> bytes);

using hgMainBoardStatePublisher = Publisher<unitree_hg::msg::dds_::MainBoardState_>;
std::unique_ptr<hgMainBoardStatePublisher> new_hg_main_board_state_publisher(rust::Str topic);
void publish_hg_main_board_state_bytes(const hgMainBoardStatePublisher& pub, rust::Vec<uint8_t> bytes);

using hgMotorCmdPublisher = Publisher<unitree_hg::msg::dds_::MotorCmd_>;
std::unique_ptr<hgMotorCmdPublisher> new_hg_motor_cmd_publisher(rust::Str topic);
void publish_hg_motor_cmd_bytes(const hgMotorCmdPublisher& pub, rust::Vec<uint8_t> bytes);

using hgMotorStatePublisher = Publisher<unitree_hg::msg::dds_::MotorState_>;
std::unique_ptr<hgMotorStatePublisher> new_hg_motor_state_publisher(rust::Str topic);
void publish_hg_motor_state_bytes(const hgMotorStatePublisher& pub, rust::Vec<uint8_t> bytes);

using hgPressSensorStatePublisher = Publisher<unitree_hg::msg::dds_::PressSensorState_>;
std::unique_ptr<hgPressSensorStatePublisher> new_hg_press_sensor_state_publisher(rust::Str topic);
void publish_hg_press_sensor_state_bytes(const hgPressSensorStatePublisher& pub, rust::Vec<uint8_t> bytes);

std::unique_ptr<SubscriberBase> make_subscriber(
    const std::string& topic, const std::string& type_name,
    std::function<void(rust::Vec<uint8_t>)> cb);

} // namespace unitree
