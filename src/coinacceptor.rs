use crate::client::*;
use crate::protocol::*;

#[derive(Debug)]
struct CoinDef {
    inhibit: bool,
    coin_id: String,
    sort_path: u8,
}

#[allow(dead_code)]
pub struct CoinAcceptor {
    client: Box<dyn CCTalkClient + 'static>,
    address: Address,
    checksum_type: ChecksumType,
    counter: u8,
    cc_equipment_cat_id: String,
    cc_serial: u16,
    cc_master_inhibit: bool,
    cc_manuf_id: String,
    cc_prod_code: String,
    cc_software_rev: String,
    cc_build_code: String,
    cc_coin_table: [CoinDef; 16],
    credit_buffer: Vec<u8>,
}

impl CoinAcceptor {
    pub fn new(
        client: Box<dyn CCTalkClient + 'static>,
        address: Address,
        checksum_type: ChecksumType,
    ) -> Result<CoinAcceptor, ClientError> {
        Ok(CoinAcceptor {
            client,
            address: address,
            checksum_type: checksum_type,
            counter: 0,
            cc_equipment_cat_id: "Coin Acceptor".to_string(),
            cc_serial: 123u16,
            cc_master_inhibit: true,
            cc_manuf_id: "PAF".to_string(),
            cc_prod_code: "Emulator".to_string(),
            cc_software_rev: "EMU-000".to_string(),
            cc_build_code: "EE0".to_string(),
            credit_buffer: vec![0u8; 10],
            cc_coin_table: [
                CoinDef {
                    inhibit: false,
                    coin_id: "EU020A".to_string(),
                    sort_path: 3u8,
                },
                CoinDef {
                    inhibit: false,
                    coin_id: "EU050A".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: false,
                    coin_id: "EU100A".to_string(),
                    sort_path: 2u8,
                },
                CoinDef {
                    inhibit: false,
                    coin_id: "EU200A".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: false,
                    coin_id: "SE100C".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: false,
                    coin_id: "SE200B".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: false,
                    coin_id: "SE500B".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: false,
                    coin_id: "SE1K0A".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "EU500A".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "EU1K0A".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "EU2K0A".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "EU4K0A".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "......".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "......".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "......".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "......".to_string(),
                    sort_path: 1u8,
                },
            ],
        })
    }
    fn ack(&mut self) -> Result<(), ClientError> {
        let msg = self.create_message(Payload {
            header: (HeaderType::Reply),
            data: (vec![]),
        });
        self.client.send_message(&msg)
    }
    fn create_message(&mut self, payload: Payload) -> Message {
        // TODO: Fix the hardcoded destination address
        Message::new(1u8, self.address, payload, self.checksum_type)
    }
    pub fn read_messages(&mut self) -> Vec<Message> {
        let _received = self.client.read_messages();
        let received = match _received {
            Ok(data) => {
                log::trace!("Read: {:?}", data);
                data
            }
            Err(error) => {
                match error {
                    ClientError::CCTalkError(ErrorType::ChecksumError) => {
                        println!("Checksum error");
                    }
                    _ => panic!("Client error: {:?}", error),
                }
                vec![]
            }
        };
        received
    }
    pub fn reply_message(&mut self, message: &Message) -> Result<(), ClientError> {
        let header = message.payload.header;
        match header {
            HeaderType::RequestEquipmentCategoryId => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (self.cc_equipment_cat_id.as_bytes().to_vec()),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::SimplePoll | HeaderType::PerformSelfcheck => self.ack(),
            HeaderType::RequestProductCode => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (self.cc_prod_code.as_bytes().to_vec()),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::RequestBuildCode => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (self.cc_build_code.as_bytes().to_vec()),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::RequestManufacturerId => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (self.cc_manuf_id.as_bytes().to_vec()),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::RequestSerialNumber => {
                let mut serial: Vec<u8> = Vec::new();
                serial.push(self.cc_serial.checked_rem_euclid(256).expect("SN error") as u8);
                serial.push(self.cc_serial.checked_div_euclid(256).expect("SN error") as u8);
                serial.push(0u8);

                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (serial),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::RequestSoftwareRevision => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (self.cc_software_rev.as_bytes().to_vec()),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::RequestCommsRevision => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (vec![1u8, 4u8, 4u8]),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::ModifyInhibitStatus => {
                let bitmask =
                    u16::from_le_bytes([message.payload.data[0], message.payload.data[1]]);
                for i in 0..16 {
                    if bitmask & (1 << i) != 0 {
                        self.cc_coin_table[i].inhibit = false;
                    } else {
                        self.cc_coin_table[i].inhibit = true;
                    }
                }
                self.ack()
            }
            HeaderType::RequestInhibitStatus => {
                let mut bitmask: u16 = 0;
                for i in 0..16 {
                    if self.cc_coin_table[i].inhibit == false {
                        bitmask = bitmask | (1 << i);
                    }
                }
                let msg = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (bitmask.to_le_bytes().to_vec()),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::RequestMasterInhibitStatus => {
                let status: u8;
                if self.cc_master_inhibit {
                    status = 0u8;
                } else {
                    status = 1u8;
                }
                let msg = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (vec![status]),
                });
                self.client.send_message(&msg)
            }
            HeaderType::ModifyMasterInhibitStatus => {
                if message.payload.data[0] & 1u8 != 0 {
                    self.cc_master_inhibit = false;
                } else {
                    self.cc_master_inhibit = true;
                }
                self.ack()
            }
            HeaderType::RequestCoinId => {
                let msg = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (self.cc_coin_table[usize::from(message.payload.data[0] - 1)]
                        .coin_id
                        .as_bytes()
                        .to_vec()),
                });
                self.client.send_message(&msg)
            }
            HeaderType::ReadBufferedCreditOrErrorCodes => {
                let mut data: Vec<u8> = vec![];
                let mut buffer = self.credit_buffer.clone();
                data.push(self.counter);
                data.append(&mut buffer);

                //println!("Data: {:?}", data);
                //println!("CB: {:?}", self.credit_buffer);
                let msg = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (data),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::RequestDataStorageAvailability => {
                let data: Vec<u8> = vec![0, 0, 0, 0, 0];
                let msg = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (data),
                });
                self.client.send_message(&msg)
            }
            HeaderType::ResetDevice => {
                self.counter = 0;
                self.credit_buffer = vec![0u8; 10];
                self.cc_master_inhibit = true;
                self.ack()
            }
            HeaderType::RequestPollingPriority => {
                // Polling in 200ms intervals
                let data: Vec<u8> = vec![2, 20];
                let msg = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (data),
                });
                self.client.send_message(&msg)
            }
            HeaderType::RequestDatabaseVersion => {
                // Remote programming not supported
                let data: Vec<u8> = vec![0];
                let msg = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (data),
                });
                self.client.send_message(&msg)
            }
            HeaderType::RequestSorterPaths => {
                let msg = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (vec![
                        self.cc_coin_table[usize::from(message.payload.data[0] - 1)].sort_path,
                    ]),
                });
                self.client.send_message(&msg)
            }
            _ => Ok(()),
        }
    }
    pub fn add_credit(&mut self, channel: u8) {
        let (cnt, roll) = self.counter.overflowing_add(1);
        if roll {
            self.counter = 1u8;
        } else {
            self.counter = cnt;
        }
        self.credit_buffer
            .insert(0, self.cc_coin_table[usize::from(channel - 1)].sort_path);
        self.credit_buffer.insert(0, channel);
        self.credit_buffer.truncate(10);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::Message;

    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};

    pub struct MPSCTestClient {
        buf: Receiver<Vec<u8>>,
        msg: Sender<Message>,
    }

    impl MPSCTestClient {
        pub fn new(buf: Receiver<Vec<u8>>, msg: Sender<Message>) -> Self {
            MPSCTestClient { buf, msg }
        }
    }

    impl CCTalkClient for MPSCTestClient {
        fn send_and_check_reply(&mut self, _msg: &Message) -> Result<Payload, ClientError> {
            Ok(Payload {
                header: HeaderType::Unknown(0),
                data: vec![],
            })
        }
        fn get_address(&self) -> Address {
            2
        }
        fn set_bill_event(&mut self, _bill_event: BillEvent) {}
        fn read_messages(&mut self) -> Result<Vec<Message>, ClientError> {
            let mut buf = self.buf.recv().unwrap();
            let msg = Message::decode(&mut buf)?;
            Ok(vec![msg])
        }
        fn send_message(&mut self, msg: &Message) -> Result<(), ClientError> {
            self.msg.send(msg.clone()).unwrap();
            Ok(())
        }
    }

    macro_rules! send {
        ($cctalk: expr, $channels: expr, $data: expr) => {{
            $channels.0.send($data).unwrap();
            let msg = $cctalk.read_messages().pop().unwrap();
            $cctalk.reply_message(&msg).unwrap();
            $channels.1.try_recv().unwrap()
        }};
    }

    #[test]
    fn test_full_initialization_flow() {
        let (btx, brx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        let (mtx, mrx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

        let client = MPSCTestClient::new(brx, mtx);
        let mut cctalk =
            CoinAcceptor::new(Box::new(client), 2, ChecksumType::SimpleChecksum).unwrap();

        let channels = (&btx, &mrx);

        // Request equipment category id
        let resp = send!(cctalk, channels, vec![2, 0, 1, 245, 8]);
        assert_eq!(
            resp.encode(),
            vec![1, 13, 2, 0, 67, 111, 105, 110, 32, 65, 99, 99, 101, 112, 116, 111, 114, 22]
        );

        /* TODO: Implement functionality to customize this
        // Request serial number
        let resp = send!(cctalk, channels, vec![2, 0, 1, 242, 11]);
        assert_eq!(
            resp.encode(),
            vec![1, 3, 2, 0, 149, 48, 16, 37]
        );
        */

        // Modify master inhibit status: inhibit active
        let resp = send!(cctalk, channels, vec![2, 1, 1, 228, 0, 24]);
        assert_eq!(
            resp.encode(),
            // ACK
            vec![1, 0, 2, 0, 253]
        );
        assert_eq!(cctalk.cc_master_inhibit, true);

        // Request polling priority
        let resp = send!(cctalk, channels, vec![2, 0, 1, 249, 4]);
        assert_eq!(
            resp.encode(),
            // 10ms*20=200ms
            vec![1, 2, 2, 0, 2, 20, 229]
        );

        // TODO: Implement customization functionality
        // Request manufacturer id
        /*
        let resp = send!(cctalk, channels, vec![2, 0, 1, 246, 7]);
        assert_eq!(
            resp.encode(),
            // ASCII:"CPS"
            vec![1, 3, 2, 0, 67, 80, 83, 20]
        );
        */

        // TODO: Implement customization functionality
        // Request product code
        /*
        let resp = send!(cctalk, channels, vec![2, 0, 1, 244, 9]);
        assert_eq!(
            resp.encode(),
            // ASCII:"Colibri"
            vec![1, 7, 2, 0, 67, 111, 108, 105, 98, 114, 105, 50]
        );
        */

        // Request database version
        let resp = send!(cctalk, channels, vec![2, 0, 1, 243, 10]);
        assert_eq!(
            resp.encode(),
            // 0 = remote programming not available
            vec![1, 1, 2, 0, 0, 252]
        );
        // TODO: Make it customizable
        // Request software revision
        /*
        let resp = send!(cctalk, channels, vec![2, 0, 1, 241, 12]);
        assert_eq!(
            resp.encode(),
            // ASCII:"412-005"
            vec![1, 7, 2, 0, 52, 49, 50, 45, 48, 48, 53, 157]
        );
        */

        // TODO: Make it customizable
        // Request build code
        /*
        let resp = send!(cctalk, channels, vec![2, 0, 1, 192, 61]);
        assert_eq!(
            resp.encode(),
            // ASCII:"DE0"
            vec![1, 3, 2, 0, 68, 69, 48, 65]
        );
        */

        // Request master inhibit status
        let resp = send!(cctalk, channels, vec![2, 0, 1, 227, 26]);
        assert_eq!(
            resp.encode(),
            // Master inhibit active
            vec![1, 1, 2, 0, 0, 252]
        );
        assert_eq!(cctalk.cc_master_inhibit, true);

        // Request inhibit status
        let resp = send!(cctalk, channels, vec![2, 0, 1, 230, 23]);
        assert_eq!(
            resp.encode(),
            // Channels 0-7 active
            vec![1, 2, 2, 0, 255, 0, 252]
        );
        for i in 0..=7 {
            let cc = &cctalk.cc_coin_table[i];
            assert_eq!(cc.inhibit, false);
        }
        for i in 8..cctalk.cc_coin_table.len() {
            let cc = &cctalk.cc_coin_table[i];
            assert_eq!(cc.inhibit, true);
        }

        // Request coin id from channel 1
        let resp = send!(cctalk, channels, vec![2, 1, 1, 184, 1, 67]);
        assert_eq!(
            resp.encode(),
            // ASCII:"EU020A"
            vec![1, 6, 2, 0, 69, 85, 48, 50, 48, 65, 138]
        );

        // Request coin id from channel 2
        let resp = send!(cctalk, channels, vec![2, 1, 1, 184, 2, 66]);
        assert_eq!(
            resp.encode(),
            // ASCII:"EU050A"
            vec![1, 6, 2, 0, 69, 85, 48, 53, 48, 65, 135]
        );

        // Request coin id from channel 3
        let resp = send!(cctalk, channels, vec![2, 1, 1, 184, 3, 65]);
        assert_eq!(
            resp.encode(),
            // ASCII:"EU100A"
            vec![1, 6, 2, 0, 69, 85, 49, 48, 48, 65, 139]
        );

        // Request coin id from channel 4
        let resp = send!(cctalk, channels, vec![2, 1, 1, 184, 4, 64]);
        assert_eq!(
            resp.encode(),
            // ASCII:"EU200A"
            vec![1, 6, 2, 0, 69, 85, 50, 48, 48, 65, 138]
        );

        // Request coin id from channel 5
        let resp = send!(cctalk, channels, vec![2, 1, 1, 184, 5, 63]);
        assert_eq!(
            resp.encode(),
            // ASCII:"SE100C"
            vec![1, 6, 2, 0, 83, 69, 49, 48, 48, 67, 139]
        );

        // Request coin id from channel 6
        let resp = send!(cctalk, channels, vec![2, 1, 1, 184, 6, 62]);
        assert_eq!(
            resp.encode(),
            // ASCII:"SE200B"
            vec![1, 6, 2, 0, 83, 69, 50, 48, 48, 66, 139]
        );

        // Request coin id from channel 7
        let resp = send!(cctalk, channels, vec![2, 1, 1, 184, 7, 61]);
        assert_eq!(
            resp.encode(),
            // ASCII:"SE500B"
            vec![1, 6, 2, 0, 83, 69, 53, 48, 48, 66, 136]
        );

        // Request coin id from channel 8
        let resp = send!(cctalk, channels, vec![2, 1, 1, 184, 8, 60]);
        assert_eq!(
            resp.encode(),
            // ASCII:"SE1K0A"
            vec![1, 6, 2, 0, 83, 69, 49, 75, 48, 65, 114]
        );

        // TODO: Skip coin ids from channels 9..12, until we have
        // customization functionality to match the real data

        // Request coin ids from channels 13..16 (all are empty)
        for i in 13..16 {
            let resp = send!(cctalk, channels, vec![2, 1, 1, 184, i, 68 - i]);
            assert_eq!(
                resp.encode(),
                // ASCII:"......"
                vec![1, 6, 2, 0, 46, 46, 46, 46, 46, 46, 227]
            );
        }

        /*
         TODO...
        2 1 1 209 1 42 ; Request sorter paths
        1 4 2 0 3 4 4 4 234
        2 1 1 209 2 41 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 3 40 ; Request sorter paths
        1 4 2 0 2 4 4 4 235
        2 1 1 209 4 39 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 5 38 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 6 37 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 7 36 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 8 35 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 9 34 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 10 33 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 11 32 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 12 31 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 13 30 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 14 29 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 15 28 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        2 1 1 209 16 27 ; Request sorter paths
        1 4 2 0 1 4 4 4 236
        */

        // Modify inhibit status (enables all configured coin channels 0..7)
        let resp = send!(cctalk, channels, vec![2, 2, 1, 231, 255, 0, 21]);
        assert_eq!(
            resp.encode(),
            // Inhibit disabled
            vec![1, 0, 2, 0, 253]
        );
        for (i, cc) in cctalk.cc_coin_table.iter().enumerate() {
            // Channels 0..=7 are enabled
            let status = match i {
                0..=7 => false,
                _ => true,
            };
            assert_eq!(cc.inhibit, status);
        }

        // Request inhibit status
        let resp = send!(cctalk, channels, vec![2, 0, 1, 230, 23]);
        assert_eq!(
            resp.encode(),
            // Inhibit disabled on channels 0-7
            vec![1, 2, 2, 0, 255, 0, 252]
        );

        // Read buffered credit or error codes
        let resp = send!(cctalk, channels, vec![2, 0, 1, 229, 24]);
        assert_eq!(
            resp.encode(),
            vec![1, 11, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 242]
        );
    }

    #[test]
    fn test_coin_insert() {
        let (btx, brx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        let (mtx, mrx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

        let client = MPSCTestClient::new(brx, mtx);
        let mut cctalk =
            CoinAcceptor::new(Box::new(client), 2, ChecksumType::SimpleChecksum).unwrap();

        let channels = (&btx, &mrx);

        // Read buffered credit or error codes
        let resp = send!(cctalk, channels, vec![2, 0, 1, 229, 24]);
        assert_eq!(
            resp.encode(),
            vec![1, 11, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 242]
        );

        // 1.
        cctalk.add_credit(3);
        let resp = send!(cctalk, channels, vec![2, 0, 1, 229, 24]);
        assert_eq!(
            resp.encode(),
            vec![1, 11, 2, 0, 1, 3, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0xEC]
        );

        // 2..
        cctalk.add_credit(2);
        let resp = send!(cctalk, channels, vec![2, 0, 1, 229, 24]);
        assert_eq!(
            resp.encode(),
            vec![1, 11, 2, 0, 2, 2, 1, 3, 2, 0, 0, 0, 0, 0, 0, 0xE8]
        );

        // 3...
        cctalk.add_credit(3);
        let resp = send!(cctalk, channels, vec![2, 0, 1, 229, 24]);
        assert_eq!(
            resp.encode(),
            vec![1, 11, 2, 0, 3, 3, 2, 2, 1, 3, 2, 0, 0, 0, 0, 0xE2]
        );

        // 4....
        cctalk.add_credit(3);
        let resp = send!(cctalk, channels, vec![2, 0, 1, 229, 24]);
        assert_eq!(
            resp.encode(),
            vec![1, 11, 2, 0, 4, 3, 2, 3, 2, 2, 1, 3, 2, 0, 0, 0xDC]
        );

        // 5....
        cctalk.add_credit(2);
        let resp = send!(cctalk, channels, vec![2, 0, 1, 229, 24]);
        assert_eq!(
            resp.encode(),
            vec![1, 11, 2, 0, 5, 2, 1, 3, 2, 3, 2, 2, 1, 3, 2, 0xD8]
        );

        // 6......
        cctalk.add_credit(1);
        let resp = send!(cctalk, channels, vec![2, 0, 1, 229, 24]);
        assert_eq!(
            resp.encode(),
            vec![1, 11, 2, 0, 6, 1, 3, 2, 1, 3, 2, 3, 2, 2, 1, 0xD8]
        );

        // 7.......
        cctalk.add_credit(2);
        let resp = send!(cctalk, channels, vec![2, 0, 1, 229, 24]);
        assert_eq!(
            resp.encode(),
            vec![1, 11, 2, 0, 7, 2, 1, 1, 3, 2, 1, 3, 2, 3, 2, 0xD7]
        );
    }

    #[test]
    fn test_counter_wrap() {
        // TODO: We don't need Sender/Receiver here..
        let (_btx, brx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        let (mtx, _mrx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

        let client = MPSCTestClient::new(brx, mtx);
        let mut cctalk =
            CoinAcceptor::new(Box::new(client), 2, ChecksumType::SimpleChecksum).unwrap();

        // On startup, credit counter is 0
        assert_eq!(cctalk.counter, 0);

        for _i in 0..255 {
            cctalk.add_credit(1);
        }
        assert_eq!(cctalk.counter, 255);

        // After overflow, we must skip 0
        cctalk.add_credit(1);
        assert_eq!(cctalk.counter, 1);
    }
}
