use cctalk::{
    device::{CoinAcceptor, CoinTable, CoreInfo},
    protocol::{ChecksumType, Message},
};
use clap::{App, Arg};
use std::thread;
use std::time::Duration;

const PROGRAM: Option<&'static str> = option_env!("CARGO_PKG_NAME");
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const DESCRIPTION: Option<&'static str> = option_env!("CARGO_PKG_DESCRIPTION");

fn main() {

    env_logger::init();
    
    let matches = App::new(PROGRAM.unwrap_or("cctalk-emulator"))
        .version(VERSION.unwrap_or("unknown"))
        .about(DESCRIPTION.unwrap_or(""))
        .arg(
            Arg::new("serial")
                .short('s')
                .long("serial")
                .value_name("DEVICE")
                .help("Serial Device for ccTalk")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let dev = matches.value_of("serial").unwrap();

    let serial = serialport::new(dev, 9600)
        .timeout(Duration::from_millis(20))
        .open()
        .expect("Failed to open port");

    let device_address = 2;
    let serial_dev = Box::new(cctalk::client::SerialClient::new(serial, device_address).unwrap());
    let device_info = CoreInfo {
        manufacturer: "NONE",
        product_code: "Test",
        build_code: "A0",
        software_revision: "000-000",
        // TODO: This needs to be fixed..
        serial_number: 123u16,
    };

    let mut cctalk = CoinAcceptor::init(
        serial_dev,
        ChecksumType::SimpleChecksum,
        // TODO: Supply empty coin table for now..
        Box::new(CoinTable::default()),
        Box::new(device_info),
    )
    .unwrap();

    println!(
        "Example CoinAcceptor listening at address = {}",
        device_address
    );

    loop {
        let mut msg: Vec<Message> = cctalk.read_messages();

        while msg.len() > 0 {
            log::info!("<- {:?}", &msg);
            cctalk.reply_message(&msg.remove(0)).unwrap();

            thread::sleep(Duration::from_millis(20));
        }

        thread::sleep(Duration::from_millis(50));
    }
}
