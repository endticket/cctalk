use crate::client::*;
use crate::protocol::*;

#[allow(dead_code)]
struct CoinDef {
    inhibit: bool,
    coin_id: String,
    sort_path: u8,
}

#[allow(dead_code)]
pub struct CCTalkEmu {
    client: Box<dyn CCTalkClient + 'static>,
    address: Address,
    checksum_type: ChecksumType,
    pub counter: u8,
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

impl CCTalkEmu {
    pub fn new(
        client: Box<dyn CCTalkClient + 'static>,
        address: Address,
        checksum_type: ChecksumType,
    ) -> Result<CCTalkEmu, ClientError> {
        Ok(CCTalkEmu {
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
                    inhibit: true,
                    coin_id: "EU020A".to_string(),
                    sort_path: 3u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "EU050A".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "EU100A".to_string(),
                    sort_path: 2u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "EU200A".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "SE100C".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "SE200B".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
                    coin_id: "SE500B".to_string(),
                    sort_path: 1u8,
                },
                CoinDef {
                    inhibit: true,
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
                        
                    },
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
            HeaderType::SimplePoll | HeaderType::PerformSelfcheck => {
                self.ack()
            }
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
                let bitmask =u16::from_le_bytes([message.payload.data[0], message.payload.data[1]]);
                for i in 0..16 {
                    if bitmask & (1<<i) != 0 {
                        self.cc_coin_table[i].inhibit = false;
                    } else {
                        self.cc_coin_table[i].inhibit = true;
                    }
                }
                self.ack()
            }
            HeaderType::RequestInhibitStatus => {
                let mut bitmask:u16 = 0;
                for i in 0..16 {
                    if self.cc_coin_table[i].inhibit == false {
                        bitmask = bitmask | (1<<i);
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
                let status:u8;
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
                    data: (self.cc_coin_table[usize::from(message.payload.data[0]-1)].coin_id.as_bytes().to_vec()),
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
                let msg = self.create_message(Payload{
                    header:(HeaderType::Reply),
                    data: (data),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::RequestDataStorageAvailability => {
                let data: Vec<u8> = vec![0, 0, 0, 0, 0];
                let msg = self.create_message(Payload{
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
                let msg = self.create_message(Payload{
                    header: (HeaderType::Reply),
                    data: (data),
                });
                self.client.send_message(&msg)
            }
            HeaderType::RequestDatabaseVersion => {
                // Remote programming not supported
                let data: Vec<u8> = vec![0];
                let msg = self.create_message(Payload{
                    header: (HeaderType::Reply),
                    data: (data),
                });
                self.client.send_message(&msg)
            }
            HeaderType::RequestSorterPaths => {
                let msg = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (vec![self.cc_coin_table[usize::from(message.payload.data[0]-1)].sort_path]),
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
        self.credit_buffer.insert(0, self.cc_coin_table[usize::from(channel-1)].sort_path);
        self.credit_buffer.insert(0, channel);
        self.credit_buffer.truncate(10);
    }
}
