use cctalk::{device::CCTalkDevice, protocol::ChecksumType};
use clap::{Arg, Command};
use std::time::Duration;

const PROGRAM: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const DESCRIPTION: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");

fn master_inhibit(dev: &str, id: u8, status: Option<&str>) {
    let serial = serialport::new(dev, 9600)
        .timeout(Duration::from_millis(500))
        .open()
        .expect("Failed to open port");

    let serial_dev = Box::new(cctalk::client::SerialClient::new(serial, 1).unwrap());
    let mut cctalk = CCTalkDevice::new(serial_dev, id, ChecksumType::SimpleChecksum).unwrap();

    if let Some(b) = status {
        let x = match b {
            "1" => 1,
            _ => 0,
        };
        println!("{:?}", cctalk.modify_master_inhibit_status(x).unwrap());
    }
}

fn request_info(dev: &str, id: u8, field: Option<&str>) {
    let serial = serialport::new(dev, 9600)
        .timeout(Duration::from_millis(500))
        .open()
        .expect("Failed to open port");

    let serial_dev = Box::new(cctalk::client::SerialClient::new(serial, 1).unwrap());

    let mut cctalk = CCTalkDevice::new(serial_dev, id, ChecksumType::SimpleChecksum).unwrap();

    match field {
        Some("equipment_category_id") => {
            println!("{:?}", cctalk.request_equipment_category().unwrap());
        }
        _ => {}
    }
}

fn main() {
    env_logger::init();

    let matches = Command::new(PROGRAM.unwrap_or("cctalk-host"))
        .version(VERSION.unwrap_or("unknown"))
        .about(DESCRIPTION.unwrap_or(""))
        .arg(
            Arg::new("serial")
                .short('s')
                .long("serial")
                .value_name("DEVICE")
                .help("ccTalk serial device (for example /dev/ttyUSB0 or COM3)")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::new("target")
                .short('t')
                .long("target")
                .value_name("TARGET_ADDRESS")
                .help("Address of the client device")
                .default_value("2"),
        )
        .subcommand(
            Command::new("request")
                .arg(
                    Arg::new("field")
                        //.possible_values(["manufacturer_id", "equipment_category_id", "serial_number"])
                        .possible_values(["equipment_category_id"]),
                )
                .about("Request information from device (status, manufacturer id, ...)"),
        )
        .subcommand(
            Command::new("set_master_inhibit").arg(
                Arg::new("state")
                    .takes_value(true)
                    .possible_values(["1", "0"])
                    .required(true),
            ),
        )
        .get_matches();

    let dev = matches.value_of("serial").unwrap();
    let _id: u8 = matches.value_of_t("target").unwrap_or_else(|e| e.exit());

    match matches.subcommand() {
        Some(("request", sub_matches)) => request_info(dev, _id, sub_matches.value_of("field")),
        Some(("set_master_inhibit", sub_matches)) => {
            master_inhibit(dev, _id, sub_matches.value_of("state"));
        }
        _ => {
            println!("Expecting subcommand...");
        }
    }
}
