use crate::client::*;
use crate::protocol::*;

/// Coin Configuration Information
#[derive(Clone, Copy, Debug)]
pub struct CoinInfo {
    inhibit: bool,
    coin_value: &'static str,
    sort_path: u8,
}

impl Default for CoinInfo {
    fn default() -> Self {
        CoinInfo {
            inhibit: true,
            coin_value: "......",
            sort_path: 0,
        }
    }
}

impl CoinInfo {
    /// Create active coin value
    ///
    /// NB! `coin_value` field is not validated.
    pub fn new(coin_value: &'static str, sort_path: u8) -> Self {
        CoinInfo {
            inhibit: false,
            coin_value,
            sort_path,
        }
    }
}

/// Coin Table definition
pub struct CoinTable {
    slots: Box<[CoinInfo; 16]>,
}

impl Default for CoinTable {
    fn default() -> Self {
        CoinTable {
            slots: Box::new([CoinInfo::default(); 16]),
        }
    }
}

impl CoinTable {
    pub fn get_sort_path(&self, channel: u8) -> u8 {
        self.slots[channel as usize].sort_path
    }

    pub fn get_coin_value(&self, channel: u8) -> &str {
        self.slots[channel as usize].coin_value
    }

    pub fn get_inhibit(&self, channel: u8) -> bool {
        self.slots[channel as usize].inhibit
    }

    pub fn set_coininfo(&mut self, channel: u8, coin: CoinInfo) {
        self.slots[channel as usize] = coin;
    }
}

/// CCTalk Core and Core Plus Information fields
pub struct CoreInfo {
    // Core fields
    pub manufacturer: &'static str,
    // equipment_category_id is supplied with main device information
    pub product_code: &'static str,
    pub build_code: &'static str,
    // Core Plus fields
    // TODO: serial number handling is currently not really working properly
    // There are devices that return 3 bytes as serial.
    pub serial_number: u16,
    pub software_revision: &'static str,
    // TODO: comms_revision - hardcoded for now... ?
}

/// Basic Coin Accepter implementation
///
/// By default, "Coin Acceptor" devices use address=2,
/// extra addresses include 11-17.
pub struct CoinAcceptor {
    client: Box<dyn CCTalkClient + 'static>,
    address: Address,
    checksum_type: ChecksumType,
    counter: u8,
    cc_equipment_cat_id: String,
    cc_master_inhibit: bool,
    credit_buffer: Vec<u8>,
    coin_table: Box<CoinTable>,
    cctalk_info: Box<CoreInfo>,
}

impl CoinAcceptor {
    pub fn init(
        client: Box<dyn CCTalkClient + 'static>,
        checksum_type: ChecksumType,
        coin_table: Box<CoinTable>,
        cctalk_info: Box<CoreInfo>,
    ) -> Result<CoinAcceptor, ClientError> {
        let addr = &client.get_address();
        Ok(CoinAcceptor {
            client,
            address: *addr,
            checksum_type,
            counter: 0,
            cc_master_inhibit: true,
            cc_equipment_cat_id: "Coin Acceptor".to_string(),
            credit_buffer: vec![0u8; 10],
            coin_table,
            cctalk_info,
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
                    data: (self.cctalk_info.product_code.as_bytes().to_vec()),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::RequestBuildCode => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (self.cctalk_info.build_code.as_bytes().to_vec()),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::RequestManufacturerId => {
                let msg: Message = self.create_message(Payload {
                    header: (HeaderType::Reply),
                    data: (self.cctalk_info.manufacturer.as_bytes().to_vec()),
                });
                log::trace!("Sent: {:?}", msg);
                self.client.send_message(&msg)
            }
            HeaderType::RequestSerialNumber => {
                // TODO: Fix this to take arbitrary bytes
                let mut serial: Vec<u8> = Vec::new();
                let cc_serial = self.cctalk_info.serial_number;
                serial.push(cc_serial.checked_rem_euclid(256).expect("SN error") as u8);
                serial.push(cc_serial.checked_div_euclid(256).expect("SN error") as u8);
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
                    data: (self.cctalk_info.software_revision.as_bytes().to_vec()),
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
                        self.coin_table.slots[i].inhibit = false;
                    } else {
                        self.coin_table.slots[i].inhibit = true;
                    }
                }
                self.ack()
            }
            HeaderType::RequestInhibitStatus => {
                let mut bitmask: u16 = 0;
                for i in 0..16 {
                    if !self.coin_table.get_inhibit(i) {
                        bitmask |= 1 << i;
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
                    data: (self
                        .coin_table
                        .get_coin_value(message.payload.data[0] - 1)
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
                    data: (vec![self.coin_table.get_sort_path(message.payload.data[0] - 1)]),
                });
                self.client.send_message(&msg)
            }
            _ => {
                log::warn!("Received unimplemented packet: {:?}", message.payload);
                Ok(())
            }
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
            .insert(0, self.coin_table.get_sort_path(channel - 1));
        self.credit_buffer.insert(0, channel);
        self.credit_buffer.truncate(10);
    }

    pub fn get_master_inhibit(&mut self) -> bool {
        self.cc_master_inhibit
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

    fn fullflow_cctalk_info() -> CoreInfo {
        // Core Information for fullflow test below
        CoreInfo {
            manufacturer: "CPS",
            product_code: "Colibri",
            build_code: "DE0",
            software_revision: "412-005",
            // TODO: This needs to be fixed..
            serial_number: 123u16,
        }
    }
    fn fullflow_cointable() -> CoinTable {
        let mut table = CoinTable::default();

        table.set_coininfo(0, CoinInfo::new("EU020A", 3u8));
        table.set_coininfo(1, CoinInfo::new("EU050A", 1u8));
        table.set_coininfo(2, CoinInfo::new("EU100A", 2u8));
        table.set_coininfo(3, CoinInfo::new("EU200A", 1u8));
        table.set_coininfo(4, CoinInfo::new("SE100C", 1u8));
        table.set_coininfo(5, CoinInfo::new("SE200B", 1u8));
        table.set_coininfo(6, CoinInfo::new("SE500B", 1u8));
        table.set_coininfo(7, CoinInfo::new("SE1K0A", 1u8));
        table
    }

    #[test]
    fn test_full_initialization_flow() {
        let (btx, brx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();
        let (mtx, mrx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

        let client = MPSCTestClient::new(brx, mtx);
        let mut cctalk = CoinAcceptor::init(
            Box::new(client),
            ChecksumType::SimpleChecksum,
            Box::new(fullflow_cointable()),
            Box::new(fullflow_cctalk_info()),
        )
        .unwrap();

        let channels = (&btx, &mrx);

        // Request equipment category id
        let resp = send!(cctalk, channels, vec![2, 0, 1, 245, 8]);
        assert_eq!(
            resp.encode(),
            vec![1, 13, 2, 0, 67, 111, 105, 110, 32, 65, 99, 99, 101, 112, 116, 111, 114, 22]
        );

        // Request serial number
        /* TODO: Implement it properly..
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
        assert_eq!(cctalk.get_master_inhibit(), cctalk.cc_master_inhibit);
        assert_eq!(cctalk.get_master_inhibit(), true);

        // Request polling priority
        let resp = send!(cctalk, channels, vec![2, 0, 1, 249, 4]);
        assert_eq!(
            resp.encode(),
            // 10ms*20=200ms
            vec![1, 2, 2, 0, 2, 20, 229]
        );

        // Request manufacturer id
        let resp = send!(cctalk, channels, vec![2, 0, 1, 246, 7]);
        assert_eq!(
            resp.encode(),
            // ASCII:"CPS"
            vec![1, 3, 2, 0, 67, 80, 83, 20]
        );

        // Request product code
        let resp = send!(cctalk, channels, vec![2, 0, 1, 244, 9]);
        assert_eq!(
            resp.encode(),
            // ASCII:"Colibri"
            vec![1, 7, 2, 0, 67, 111, 108, 105, 98, 114, 105, 50]
        );

        // Request database version
        let resp = send!(cctalk, channels, vec![2, 0, 1, 243, 10]);
        assert_eq!(
            resp.encode(),
            // 0 = remote programming not available
            vec![1, 1, 2, 0, 0, 252]
        );
        // Request software revision
        let resp = send!(cctalk, channels, vec![2, 0, 1, 241, 12]);
        assert_eq!(
            resp.encode(),
            // ASCII:"412-005"
            vec![1, 7, 2, 0, 52, 49, 50, 45, 48, 48, 53, 157]
        );

        // Request build code
        let resp = send!(cctalk, channels, vec![2, 0, 1, 192, 61]);
        assert_eq!(
            resp.encode(),
            // ASCII:"DE0"
            vec![1, 3, 2, 0, 68, 69, 48, 65]
        );

        // Request master inhibit status
        let resp = send!(cctalk, channels, vec![2, 0, 1, 227, 26]);
        assert_eq!(
            resp.encode(),
            // Master inhibit active
            vec![1, 1, 2, 0, 0, 252]
        );
        assert_eq!(cctalk.get_master_inhibit(), true);

        // Request channel inhibit status
        let resp = send!(cctalk, channels, vec![2, 0, 1, 230, 23]);
        assert_eq!(
            resp.encode(),
            // Channels 0-7 active
            vec![1, 2, 2, 0, 255, 0, 252]
        );
        for i in 0..=7 {
            let cc = &cctalk.coin_table.slots[i];
            assert_eq!(cc.inhibit, false);
        }
        for i in 8..cctalk.coin_table.slots.len() {
            let cc = &cctalk.coin_table.slots[i];
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

        // Request coin ids from channels 9..16 (all are empty)
        for i in 9..=16 {
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
        for (i, cc) in cctalk.coin_table.slots.iter().enumerate() {
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
        let mut cctalk = CoinAcceptor::init(
            Box::new(client),
            ChecksumType::SimpleChecksum,
            Box::new(fullflow_cointable()),
            Box::new(fullflow_cctalk_info()),
        )
        .unwrap();

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
        let mut cctalk = CoinAcceptor::init(
            Box::new(client),
            ChecksumType::SimpleChecksum,
            Box::new(CoinTable::default()),
            Box::new(fullflow_cctalk_info()),
        )
        .unwrap();

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
