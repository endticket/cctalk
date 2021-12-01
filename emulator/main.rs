use cctalk::{
    device::CoinAcceptor,
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
            Arg::with_name("serial")
                .short("s")
                .long("serial")
                .value_name("DEVICE")
                .help("Serial Device for ccTalk")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let dev = matches.value_of("serial").unwrap();

    let serial = serialport::new(dev, 9600)
        .timeout(Duration::from_millis(500))
        .open()
        .expect("Failed to open port");

    // TODO: Revisit this
    let our_address = 2;
    let target_address = 1;
    let serial_dev = Box::new(cctalk::client::SerialClient::new(serial, our_address).unwrap());

    let mut cctalk =
        CoinAcceptor::new(serial_dev, target_address, ChecksumType::SimpleChecksum).unwrap();

    println!("Set up device with address = {}", our_address);

    loop {
        let mut msg: Vec<Message> = cctalk.read_messages();

        while msg.len() > 0 {
            log::info!("<- {:?}", &msg);
            cctalk.reply_message(&msg.remove(0)).unwrap();

            thread::sleep(Duration::from_millis(20));
        }

        thread::sleep(Duration::from_millis(100));
    }
}
