// ⚡ Auto-generated (msg → Rust). Regenerate by editing msgs/ or build.rs.

pub mod std_msgs {
  pub mod msg {
    pub mod dds_ {
      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct String {
          pub data: std::string::String,
      }

      impl crate::DdsType for String {
          fn dds_name() -> &'static str { "std_msgs::msg::dds_::String_" }
      }

    }
  }
}
pub mod unitree_api {
  pub mod msg {
    pub mod dds_ {
      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct Request {
          pub header: RequestHeader,
          pub parameter: std::string::String,
          pub binary: Vec<u8>,
      }

      impl crate::DdsType for Request {
          fn dds_name() -> &'static str { "unitree_api::msg::dds_::Request_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct RequestHeader {
          pub identity: RequestIdentity,
          pub lease: RequestLease,
          pub policy: RequestPolicy,
      }

      impl crate::DdsType for RequestHeader {
          fn dds_name() -> &'static str { "unitree_api::msg::dds_::RequestHeader_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct RequestIdentity {
          pub id: i64,
          pub api_id: i64,
      }

      impl crate::DdsType for RequestIdentity {
          fn dds_name() -> &'static str { "unitree_api::msg::dds_::RequestIdentity_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct RequestLease {
          pub id: i64,
      }

      impl crate::DdsType for RequestLease {
          fn dds_name() -> &'static str { "unitree_api::msg::dds_::RequestLease_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct RequestPolicy {
          pub priority: i32,
          pub noreply: bool,
      }

      impl crate::DdsType for RequestPolicy {
          fn dds_name() -> &'static str { "unitree_api::msg::dds_::RequestPolicy_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct Response {
          pub header: ResponseHeader,
          pub data: std::string::String,
          pub binary: Vec<i8>,
      }

      impl crate::DdsType for Response {
          fn dds_name() -> &'static str { "unitree_api::msg::dds_::Response_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct ResponseHeader {
          pub identity: RequestIdentity,
          pub status: ResponseStatus,
      }

      impl crate::DdsType for ResponseHeader {
          fn dds_name() -> &'static str { "unitree_api::msg::dds_::ResponseHeader_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct ResponseStatus {
          pub code: i32,
      }

      impl crate::DdsType for ResponseStatus {
          fn dds_name() -> &'static str { "unitree_api::msg::dds_::ResponseStatus_" }
      }

    }
  }
}
pub mod unitree_go {
  pub mod msg {
    pub mod dds_ {
      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct AudioData {
          pub time_frame: u64,
          pub data: Vec<u8>,
      }

      impl crate::DdsType for AudioData {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::AudioData_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct BmsCmd {
          pub off: u8,
          pub reserve: [u8; 3],
      }

      impl crate::DdsType for BmsCmd {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::BmsCmd_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct BmsState {
          pub version_high: u8,
          pub version_low: u8,
          pub status: u8,
          pub soc: u8,
          pub current: i32,
          pub cycle: u16,
          pub bq_ntc: [i8; 2],
          pub mcu_ntc: [i8; 2],
          pub cell_vol: [u16; 15],
      }

      impl crate::DdsType for BmsState {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::BmsState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct Error {
          pub source: u32,
          pub state: u32,
      }

      impl crate::DdsType for Error {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::Error_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct Go2FrontVideoData {
          pub time_frame: u64,
          pub video720p: Vec<u8>,
          pub video360p: Vec<u8>,
          pub video180p: Vec<u8>,
      }

      impl crate::DdsType for Go2FrontVideoData {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::Go2FrontVideoData_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct HeightMap {
          pub stamp: f64,
          pub frame_id: std::string::String,
          pub resolution: f32,
          pub width: u32,
          pub height: u32,
          pub origin: [f32; 2],
          pub data: Vec<f32>,
      }

      impl crate::DdsType for HeightMap {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::HeightMap_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct IMUState {
          pub quaternion: [f32; 4],
          pub gyroscope: [f32; 3],
          pub accelerometer: [f32; 3],
          pub rpy: [f32; 3],
          pub temperature: i8,
      }

      impl crate::DdsType for IMUState {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::IMUState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct InterfaceConfig {
          pub mode: u8,
          pub value: u8,
          pub reserve: [u8; 2],
      }

      impl crate::DdsType for InterfaceConfig {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::InterfaceConfig_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct LidarState {
          pub stamp: f64,
          pub firmware_version: std::string::String,
          pub software_version: std::string::String,
          pub sdk_version: std::string::String,
          pub sys_rotation_speed: f32,
          pub com_rotation_speed: f32,
          pub error_state: u8,
          pub cloud_frequency: f32,
          pub cloud_packet_loss_rate: f32,
          pub cloud_size: u32,
          pub cloud_scan_num: u32,
          pub imu_frequency: f32,
          pub imu_packet_loss_rate: f32,
          pub imu_rpy: [f32; 3],
          pub serial_recv_stamp: f64,
          pub serial_buffer_size: u32,
          pub serial_buffer_read: u32,
      }

      impl crate::DdsType for LidarState {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::LidarState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct LowCmd {
          pub head: [u8; 2],
          pub level_flag: u8,
          pub frame_reserve: u8,
          pub sn: [u32; 2],
          pub version: [u32; 2],
          pub bandwidth: u16,
          pub motor_cmd: [MotorCmd; 20],
          pub bms_cmd: BmsCmd,
          #[serde(with = "serde_big_array::BigArray")]
          pub wireless_remote: [u8; 40],
          pub led: [u8; 12],
          pub fan: [u8; 2],
          pub gpio: u8,
          pub reserve: u32,
          pub crc: u32,
      }

      impl crate::DdsType for LowCmd {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::LowCmd_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct LowState {
          pub head: [u8; 2],
          pub level_flag: u8,
          pub frame_reserve: u8,
          pub sn: [u32; 2],
          pub version: [u32; 2],
          pub bandwidth: u16,
          pub imu_state: IMUState,
          pub motor_state: [MotorState; 20],
          pub bms_state: BmsState,
          pub foot_force: [i16; 4],
          pub foot_force_est: [i16; 4],
          pub tick: u32,
          #[serde(with = "serde_big_array::BigArray")]
          pub wireless_remote: [u8; 40],
          pub bit_flag: u8,
          pub adc_reel: f32,
          pub temperature_ntc1: i8,
          pub temperature_ntc2: i8,
          pub power_v: f32,
          pub power_a: f32,
          pub fan_frequency: [u16; 4],
          pub reserve: u32,
          pub crc: u32,
      }

      impl crate::DdsType for LowState {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::LowState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct MotorCmd {
          pub mode: u8,
          pub q: f32,
          pub dq: f32,
          pub tau: f32,
          pub kp: f32,
          pub kd: f32,
          pub reserve: [u32; 3],
      }

      impl crate::DdsType for MotorCmd {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::MotorCmd_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct MotorCmds {
          pub cmds: Vec<MotorCmd>,
      }

      impl crate::DdsType for MotorCmds {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::MotorCmds_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct MotorState {
          pub mode: u8,
          pub q: f32,
          pub dq: f32,
          pub ddq: f32,
          pub tau_est: f32,
          pub q_raw: f32,
          pub dq_raw: f32,
          pub ddq_raw: f32,
          pub temperature: i8,
          pub lost: u32,
          pub reserve: [u32; 2],
      }

      impl crate::DdsType for MotorState {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::MotorState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct MotorStates {
          pub states: Vec<MotorState>,
      }

      impl crate::DdsType for MotorStates {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::MotorStates_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct PathPoint {
          pub t_from_start: f32,
          pub x: f32,
          pub y: f32,
          pub yaw: f32,
          pub vx: f32,
          pub vy: f32,
          pub vyaw: f32,
      }

      impl crate::DdsType for PathPoint {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::PathPoint_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct Req {
          pub uuid: std::string::String,
          pub body: std::string::String,
      }

      impl crate::DdsType for Req {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::Req_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct Res {
          pub uuid: std::string::String,
          pub data: Vec<u8>,
          pub body: std::string::String,
      }

      impl crate::DdsType for Res {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::Res_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct SportModeCmd {
          pub mode: u8,
          pub gait_type: u8,
          pub speed_level: u8,
          pub foot_raise_height: f32,
          pub body_height: f32,
          pub position: [f32; 2],
          pub euler: [f32; 3],
          pub velocity: [f32; 2],
          pub yaw_speed: f32,
          pub bms_cmd: BmsCmd,
          pub path_point: [PathPoint; 30],
      }

      impl crate::DdsType for SportModeCmd {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::SportModeCmd_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct SportModeState {
          pub stamp: TimeSpec,
          pub error_code: u32,
          pub imu_state: IMUState,
          pub mode: u8,
          pub progress: f32,
          pub gait_type: u8,
          pub foot_raise_height: f32,
          pub position: [f32; 3],
          pub body_height: f32,
          pub velocity: [f32; 3],
          pub yaw_speed: f32,
          pub range_obstacle: [f32; 4],
          pub foot_force: [i16; 4],
          pub foot_position_body: [f32; 12],
          pub foot_speed_body: [f32; 12],
      }

      impl crate::DdsType for SportModeState {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::SportModeState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct TimeSpec {
          pub sec: i32,
          pub nanosec: u32,
      }

      impl crate::DdsType for TimeSpec {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::TimeSpec_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct UwbState {
          pub version: [u8; 2],
          pub channel: u8,
          pub joy_mode: u8,
          pub orientation_est: f32,
          pub pitch_est: f32,
          pub distance_est: f32,
          pub yaw_est: f32,
          pub tag_roll: f32,
          pub tag_pitch: f32,
          pub tag_yaw: f32,
          pub base_roll: f32,
          pub base_pitch: f32,
          pub base_yaw: f32,
          pub joystick: [f32; 2],
          pub error_state: u8,
          pub buttons: u8,
          pub enabled_from_app: u8,
      }

      impl crate::DdsType for UwbState {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::UwbState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct UwbSwitch {
          pub enabled: u8,
      }

      impl crate::DdsType for UwbSwitch {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::UwbSwitch_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct WirelessController {
          pub lx: f32,
          pub ly: f32,
          pub rx: f32,
          pub ry: f32,
          pub keys: u16,
      }

      impl crate::DdsType for WirelessController {
          fn dds_name() -> &'static str { "unitree_go::msg::dds_::WirelessController_" }
      }

    }
  }
}
pub mod unitree_hg {
  pub mod msg {
    pub mod dds_ {
      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct BmsCmd {
          pub cmd: u8,
          #[serde(with = "serde_big_array::BigArray")]
          pub reserve: [u8; 40],
      }

      impl crate::DdsType for BmsCmd {
          fn dds_name() -> &'static str { "unitree_hg::msg::dds_::BmsCmd_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct BmsState {
          pub version_high: u8,
          pub version_low: u8,
          pub r#fn: u8,
          #[serde(with = "serde_big_array::BigArray")]
          pub cell_vol: [u16; 40],
          pub bmsvoltage: [u32; 3],
          pub current: i32,
          pub soc: u8,
          pub soh: u8,
          pub temperature: [i16; 12],
          pub cycle: u16,
          pub manufacturer_date: u16,
          pub bmsstate: [u32; 5],
          pub reserve: [u32; 3],
      }

      impl crate::DdsType for BmsState {
          fn dds_name() -> &'static str { "unitree_hg::msg::dds_::BmsState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct HandCmd {
          pub motor_cmd: Vec<MotorCmd>,
          pub reserve: [u32; 4],
      }

      impl crate::DdsType for HandCmd {
          fn dds_name() -> &'static str { "unitree_hg::msg::dds_::HandCmd_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct HandState {
          pub motor_state: Vec<MotorState>,
          pub press_sensor_state: Vec<PressSensorState>,
          pub imu_state: IMUState,
          pub power_v: f32,
          pub power_a: f32,
          pub system_v: f32,
          pub device_v: f32,
          pub error: [u32; 2],
          pub reserve: [u32; 2],
      }

      impl crate::DdsType for HandState {
          fn dds_name() -> &'static str { "unitree_hg::msg::dds_::HandState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct IMUState {
          pub quaternion: [f32; 4],
          pub gyroscope: [f32; 3],
          pub accelerometer: [f32; 3],
          pub rpy: [f32; 3],
          pub temperature: i16,
      }

      impl crate::DdsType for IMUState {
          fn dds_name() -> &'static str { "unitree_hg::msg::dds_::IMUState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct LowCmd {
          pub mode_pr: u8,
          pub mode_machine: u8,
          #[serde(with = "serde_big_array::BigArray")]
          pub motor_cmd: [MotorCmd; 35],
          pub reserve: [u32; 4],
          pub crc: u32,
      }

      impl crate::DdsType for LowCmd {
          fn dds_name() -> &'static str { "unitree_hg::msg::dds_::LowCmd_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct LowState {
          pub version: [u32; 2],
          pub mode_pr: u8,
          pub mode_machine: u8,
          pub tick: u32,
          pub imu_state: IMUState,
          #[serde(with = "serde_big_array::BigArray")]
          pub motor_state: [MotorState; 35],
          #[serde(with = "serde_big_array::BigArray")]
          pub wireless_remote: [u8; 40],
          pub reserve: [u32; 4],
          pub crc: u32,
      }

      impl crate::DdsType for LowState {
          fn dds_name() -> &'static str { "unitree_hg::msg::dds_::LowState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct MainBoardState {
          pub fan_state: [u16; 6],
          pub temperature: [i16; 6],
          pub value: [f32; 6],
          pub state: [u32; 6],
      }

      impl crate::DdsType for MainBoardState {
          fn dds_name() -> &'static str { "unitree_hg::msg::dds_::MainBoardState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct MotorCmd {
          pub mode: u8,
          pub q: f32,
          pub dq: f32,
          pub tau: f32,
          pub kp: f32,
          pub kd: f32,
          pub reserve: u32,
      }

      impl crate::DdsType for MotorCmd {
          fn dds_name() -> &'static str { "unitree_hg::msg::dds_::MotorCmd_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct MotorState {
          pub mode: u8,
          pub q: f32,
          pub dq: f32,
          pub ddq: f32,
          pub tau_est: f32,
          pub temperature: [i16; 2],
          pub vol: f32,
          pub sensor: [u32; 2],
          pub motorstate: u32,
          pub reserve: [u32; 4],
      }

      impl crate::DdsType for MotorState {
          fn dds_name() -> &'static str { "unitree_hg::msg::dds_::MotorState_" }
      }

      #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
      pub struct PressSensorState {
          pub pressure: [f32; 12],
          pub temperature: [f32; 12],
          pub lost: u32,
          pub reserve: u32,
      }

      impl crate::DdsType for PressSensorState {
          fn dds_name() -> &'static str { "unitree_hg::msg::dds_::PressSensorState_" }
      }

    }
  }
}
