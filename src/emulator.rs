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
    credit_buffer: [u8; 10],
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
            credit_buffer: [0u8; 10],
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
    pub fn create_message(&mut self, payload: Payload) -> Message {
        Message::new(1u8, self.address, payload, self.checksum_type)
    }
    pub fn read_messages(&mut self) -> Vec<Message> {
        let _received = self.client.read_messages();
        let received = match _received {
            Ok(data) => {
                // println!("Read: {:?}", data);
                data
            }
            Err(error) => panic!("Client error: {:?}", error),
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
                //println!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::SimplePoll | HeaderType::PerformSelfcheck => {
                let msg = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (vec![]),
                });
                self.client.send_message(&msg)
            }
            HeaderType::RequestProductCode => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (self.cc_prod_code.as_bytes().to_vec()),
                });
                self.client.send_message(&msg)
            }
            HeaderType::RequestBuildCode => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (self.cc_build_code.as_bytes().to_vec()),
                });
                self.client.send_message(&msg)
            }
            HeaderType::RequestManufacturerId => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (self.cc_manuf_id.as_bytes().to_vec()),
                });
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
                self.client.send_message(&msg)
            }
            HeaderType::RequestSoftwareRevision => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (self.cc_software_rev.as_bytes().to_vec()),
                });
                self.client.send_message(&msg)
            }
            HeaderType::RequestCommsRevision => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (vec![1u8, 4u8, 4u8]),
                });
                self.client.send_message(&msg)
            }
            HeaderType::ModifyInhibitStatus => {
                
                let msg = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (vec![]),
                });
                self.client.send_message(&msg)
            }
            _ => Ok(()),
        }
    }
}
