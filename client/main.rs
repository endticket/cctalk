use cctalk::{device::CCTalkDevice, protocol::ChecksumType};
use clap::{value_t, App, Arg};
use std::time::Duration;

const PROGRAM: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const DESCRIPTION: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");

fn main() {
    env_logger::init();

    let matches = App::new(PROGRAM.unwrap_or("cctalk-client"))
        .version(VERSION.unwrap_or("unknown"))
        .about(DESCRIPTION.unwrap_or(""))
        .arg(
            Arg::with_name("serial")
                .short("s")
                .long("serial")
                .value_name("DEVICE")
                .help("Serial Device for ccTalk client (for example /dev/ttyUSB0 or COM3)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .value_name("TARGET_ADDRESS")
                .help("Address of the target device")
                .default_value("2"),
        )
        .get_matches();

    let dev = matches.value_of("serial").unwrap();

    let target_device_id = value_t!(matches.value_of("target"), u8).unwrap_or_else(|e| e.exit());

    let serial = serialport::new(dev, 9600)
        .timeout(Duration::from_millis(500))
        .open()
        .expect("Failed to open port");

    // As per ccTalk general usage, there is usually single "master"
    // which initiates the queries and its address is 1.
    let serial_dev = Box::new(cctalk::client::SerialClient::new(serial, 1).unwrap());

    let mut cctalk =
        CCTalkDevice::new(serial_dev, target_device_id, ChecksumType::SimpleChecksum).unwrap();

    println!("Querying device={}", target_device_id);

    let resp = cctalk.request_equipment_category().unwrap();

    println!("{}", resp);
}
