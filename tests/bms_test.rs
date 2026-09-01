use std::time::Duration;

use unitree_sdk2_rs::unitree_hg::msg::dds_::BmsState;
use unitree_sdk2_rs::{subscribe, Publisher};

#[tokio::main]
async fn main() {
    unitree_sdk2_rs::init_dds(0, "", "");

    let (_sub, rx) = subscribe::<BmsState>("rt/test_bms");

    tokio::time::sleep(Duration::from_millis(500)).await;

    let pub_ = Publisher::<BmsState>::new("rt/test_bms").expect("failed to create BmsPublisher");
    let bms = BmsState {
        version_high: 1,
        version_low: 0,
        r#fn: 0,
        cell_vol: [4000; 40],
        bmsvoltage: [42000, 0, 0],
        current: -2500,
        soc: 85,
        soh: 100,
        temperature: [32, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21],
        cycle: 1,
        manufacturer_date: 0,
        bmsstate: [0; 5],
        reserve: [0; 3],
    };
    pub_.publish(&bms);

    tokio::time::sleep(Duration::from_secs(1)).await;

    let last = rx.borrow().clone();
    match last {
        Some(d) => {
            println!(
                "received: SOC={}% voltage={}mV current={}mA temp={}°C",
                d.soc, d.bmsvoltage[0], d.current, d.temperature[0]
            );
            println!("BMS test passed ✓");
        }
        None => {
            eprintln!("BMS test failed ✗");
            std::process::exit(1);
        }
    }
}
