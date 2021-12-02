use crate::client::*;
use crate::protocol::*;
// Reimports
pub use crate::coinacceptor::{CoinAcceptor, CoinInfo, CoinTable};

pub struct CCTalkDevice {
    pub client: Box<dyn CCTalkClient + 'static>,
    address: Address,
    checksum_type: ChecksumType,
    pub counter: u8,
}

impl CCTalkDevice {
    pub fn new(
        client: Box<dyn CCTalkClient>,
        address: Address,
        checksum_type: ChecksumType,
    ) -> Result<CCTalkDevice, ClientError> {
        Ok(CCTalkDevice {
            client,
            address,
            checksum_type,
            counter: 0,
        })
    }

    pub fn set_bill_event(&mut self, bill_event: BillEvent) {
        self.client.set_bill_event(bill_event);
    }

    pub fn create_message(&mut self, payload: Payload) -> Message {
        Message::new(
            self.address,
            self.client.get_address(),
            payload,
            self.checksum_type,
        )
    }

    pub fn reset(&mut self) -> Result<Payload, ClientError> {
        let message = self.create_message(Payload {
            header: HeaderType::ResetDevice,
            data: Vec::<u8>::new(),
        });
        self.client.send_and_check_reply(&message)
    }

    pub fn simple_poll(&mut self) -> Result<Payload, ClientError> {
        let message = self.create_message(Payload {
            header: HeaderType::SimplePoll,
            data: Vec::<u8>::new(),
        });
        self.client.send_and_check_reply(&message)
    }

    pub fn modify_inhibit_status(
        &mut self,
        inhibit_status: Vec<u8>,
    ) -> Result<Payload, ClientError> {
        let message = self.create_message(Payload {
            header: HeaderType::ModifyInhibitStatus,
            data: inhibit_status,
        });
        self.client.send_and_check_reply(&message)
    }

    pub fn modify_master_inhibit_status(
        &mut self,
        inhibit_status: u8,
    ) -> Result<Payload, ClientError> {
        let message = self.create_message(Payload {
            header: HeaderType::ModifyMasterInhibitStatus,
            data: vec![inhibit_status],
        });
        self.client.send_and_check_reply(&message)
    }

    pub fn read_buffered_credit(&mut self) -> Result<Payload, ClientError> {
        let message = self.create_message(Payload {
            header: HeaderType::ReadBufferedCreditOrErrorCodes,
            data: Vec::<u8>::new(),
        });
        self.client.send_and_check_reply(&message)
    }

    pub fn interpret_buffered_bill_payload(
        payload: Payload,
    ) -> Result<(u8, BillEvent), ClientError> {
        let mut data = payload.data;
        if data.len() < 3 {
            return Err(ClientError::CCTalkError(ErrorType::ParseError));
        }
        let counter: u8 = data.remove(0);
        Ok((counter, BillEvent::from_u8((data[0], data[1]))))
    }

    pub fn read_buffered_bill(&mut self) -> Result<(u8, BillEvent), ClientError> {
        let message = self.create_message(Payload {
            header: HeaderType::ReadBufferedBillEvents,
            data: Vec::<u8>::new(),
        });
        let response = self.client.send_and_check_reply(&message)?;
        let res = CCTalkDevice::interpret_buffered_bill_payload(response)?;
        Ok(res)
    }

    pub fn modify_bill_operating_mode(&mut self, mode: u8) -> Result<Payload, ClientError> {
        let message = self.create_message(Payload {
            header: HeaderType::ModifyBillOperatingMode,
            data: vec![mode],
        });
        self.client.send_and_check_reply(&message)
    }

    pub fn route_bill(&mut self, route: u8) -> Result<Payload, ClientError> {
        let message = self.create_message(Payload {
            header: HeaderType::RouteBill,
            data: vec![route],
        });
        self.client.send_and_check_reply(&message)
    }

    pub fn request_equipment_category(&mut self) -> Result<String, ClientError> {
        let message = self.create_message(Payload {
            header: HeaderType::RequestEquipmentCategoryId,
            data: Vec::<u8>::new(),
        });
        let payload = self.client.send_and_check_reply(&message);

        match payload {
            Ok(equipment_category) => Ok(equipment_category.as_str().unwrap()),
            Err(e) => Err(e),
        }
    }

    pub fn read_coin_ids(&mut self) -> Result<(), ClientError> {
        for x in 1..10 {
            let message = self.create_message(Payload {
                header: HeaderType::RequestCoinId,
                data: vec![x],
            });
            let payload = self.client.send_and_check_reply(&message);

            match payload {
                Ok(coin_id) => {
                    // log::debug!("Coin id raw: {:?}", coin_id);
                    log::debug!("Coin id: {}", coin_id.as_str().unwrap());
                }
                Err(e) => log::error!("Response error: {:?}", e),
            }
        }
        Ok(())
    }

    pub fn read_bill_ids(&mut self) -> Result<(), ClientError> {
        for x in 1..10 {
            let message = self.create_message(Payload {
                header: HeaderType::RequestBillId,
                data: vec![x],
            });
            let payload = self.client.send_and_check_reply(&message);

            match payload {
                Ok(bill_id) => {
                    log::debug!("Bill id: {}", bill_id.as_str().unwrap());
                }
                Err(e) => log::error!("Response error: {:?}", e),
            }
        }
        Ok(())
    }

    pub fn request_scaling_factor(&mut self) -> Result<Payload, ClientError> {
        let message = self.create_message(Payload {
            header: HeaderType::RequestCountryScalingFactor,
            data: Vec::<u8>::new(),
        });
        self.client.send_and_check_reply(&message)
    }
}
