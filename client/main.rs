use clap::{App, Arg};
use std::time::Duration;
use cctalk::{device::CCTalkDevice, protocol::ChecksumType};

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
                .help("Serial Device for ccTalk client")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let dev = matches.value_of("serial").unwrap();

    let serial = serialport::new(dev, 9600)
        .timeout(Duration::from_millis(500))
        .open()
        .expect("Failed to open port");

    // TODO: Device emulator currently sends responses only to address 1
    let our_address = 1u8;
    let client_address = 20u8;

    let serial_dev = Box::new(cctalk::client::SerialClient::new(serial, our_address).unwrap());

    let mut cctalk = CCTalkDevice::new(serial_dev, client_address, ChecksumType::SimpleChecksum).unwrap();

    let resp = cctalk.request_equipment_category().unwrap();

    println!("{}", resp);
}
