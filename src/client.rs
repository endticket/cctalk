use serial::prelude::*;
use std;
use std::convert;
use std::io::ErrorKind::TimedOut;
use std::io::{Read, Write};
use std::time::Duration;

use crate::protocol::*;

#[derive(Debug)]
pub enum ClientError {
    CCTalkError(ErrorType),
    SerialError(serial::Error),
    IOError(std::io::Error),
}

impl convert::From<serial::Error> for ClientError {
    fn from(e: serial::Error) -> ClientError {
        ClientError::SerialError(e)
    }
}

impl convert::From<ErrorType> for ClientError {
    fn from(e: ErrorType) -> ClientError {
        ClientError::CCTalkError(e)
    }
}

impl convert::From<std::io::Error> for ClientError {
    fn from(e: std::io::Error) -> ClientError {
        ClientError::IOError(e)
    }
}

impl Clone for ClientError {
    fn clone(&self) -> Self {
        match self {
            &ClientError::CCTalkError(ref e) => ClientError::CCTalkError(e.clone()),
            &ClientError::IOError(ref e) => {
                ClientError::IOError(std::io::Error::new(e.kind(), e.to_string()))
            }
            &ClientError::SerialError(ref e) => {
                ClientError::SerialError(serial::Error::new(e.kind(), e.to_string()))
            }
        }
    }
}

pub trait CCTalkClient {
    fn send_and_check_reply(&mut self, msg: &Message) -> Result<Payload, ClientError>;
    fn get_address(&self) -> Address;
    fn set_bill_event(&mut self, bill_event: BillEvent);
}

pub struct SerialClient {
    port: serial::SystemPort,
    pub address: Address,
    buffer: Vec<u8>,
}

#[allow(dead_code)]
impl SerialClient {
    pub fn new(
        port_name: &String,
        serial_settings: &serial::PortSettings,
    ) -> Result<SerialClient, ClientError> {
        let mut port_temp = serial::open(&port_name)?;

        port_temp.configure(&serial_settings)?;

        Ok(SerialClient {
            port: port_temp,
            address: 1,
            buffer: Vec::<u8>::new(),
        })
    }

    fn read_and_decode(
        &mut self,
        received: &mut Vec<u8>,
        messages: &mut Vec<Message>,
    ) -> Result<(), ClientError> {
        // log::debug!("Received: {:?}", received);
        self.buffer.append(received);
        // log::debug!("Buffer: {:?}", self.buffer);

        // decode will leave the remaining stuff in the buffer
        let decode_res = Message::decode(&mut self.buffer);
        match decode_res {
            Ok(message) => {
                if message.destination == self.address {
                    messages.push(message);
                    Ok(())
                } else {
                    // log::debug!("message to another recipient ignored");
                    Ok(())
                }
            }
            Err(ErrorType::PartialMessage) => {
                // log::debug!("Partial message");
                Ok(())
            }
            Err(e) => Err(ClientError::CCTalkError(e)),
        }
    }

    fn read_from_serial(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut buf: [u8; 260] = [0; 260];
        let mut rec: Vec<u8> = Vec::<u8>::new();

        let read_res = self.port.read(&mut buf);

        match read_res {
            Ok(usize) => {
                rec.extend_from_slice(&buf[..usize]);
                Ok(rec)
            }
            Err(ref e) if e.kind() == TimedOut => {
                // continue, this is not really an error
                Ok(rec)
            }
            Err(e) => Err(e),
        }
    }

    fn send(&mut self, msg: &Message) -> Result<(), std::io::Error> {
        let buf: Vec<u8> = msg.encode();
        // log::debug!("Sending CCTalk message: {:?}", msg);
        // log::debug!("Sending CCTalk message encoded: {:?}", buf);
        self.port.write_all(&buf[..])
    }

    fn read(&mut self) -> Result<Vec<Message>, ClientError> {
        let mut messages = Vec::<Message>::new();

        let mut counter = 0;

        while (messages.len() < 1) && (counter < 80) {
            let mut received = self.read_from_serial()?;
            // log::debug!("Received on serial: {:?} Counter: {}", received, counter);
            self.read_and_decode(&mut received, &mut messages)?;
            counter += 1;
        }

        Ok(messages)
    }

    fn read_all(&mut self, timeout: u64) -> Result<Vec<Message>, ClientError> {
        let old_timeout = self.port.timeout();

        self.port
            .set_timeout(Duration::from_millis(timeout))
            .unwrap();

        let mut messages = Vec::<Message>::new();

        let mut timeout = false;

        while !timeout {
            let mut received = self.read_from_serial()?;
            self.read_and_decode(&mut received, &mut messages)?;
            if (received.len() == 0) && (self.buffer.len() == 0) {
                timeout = true;
            }
        }

        self.port.set_timeout(old_timeout).unwrap();

        Ok(messages)
    }
}

impl CCTalkClient for SerialClient {
    fn send_and_check_reply(&mut self, msg: &Message) -> Result<Payload, ClientError> {
        self.send(msg)?;

        // log::debug!("Waiting for Reply");
        let received = self.read()?;
        if received.len() > 0 {
            let ref reply = received[0];
            match reply.payload.header {
                HeaderType::Reply => Ok(reply.payload.clone()),
                _ => Err(ClientError::CCTalkError(ErrorType::NotAReply)),
            }
        } else {
            if self.buffer.len() != 0 {
                log::debug!(
                    "Message not received in time, clearing partial message from buffer: {:?}",
                    self.buffer
                );
                self.buffer.clear();
            }
            Err(ClientError::CCTalkError(ErrorType::NoResponse))
        }
    }

    fn get_address(&self) -> Address {
        self.address
    }

    fn set_bill_event(&mut self, _: BillEvent) {}
}

pub struct DummyClient {
    counter: u8,
    bill_event: BillEvent,
    changed: bool,
}

impl DummyClient {
    pub fn new() -> DummyClient {
        log::warn!("Creating MOCK CCTalk Client");
        DummyClient {
            counter: 1,
            bill_event: BillEvent::MasterInhibitActive,
            changed: false,
        }
    }
}

impl CCTalkClient for DummyClient {
    fn send_and_check_reply(&mut self, msg: &Message) -> Result<Payload, ClientError> {
        match msg.payload.header {
            HeaderType::ReadBufferedBillEvents => {
                let (byte1, byte2) = self.bill_event.to_u8();

                if self.changed == true {
                    self.counter += 1;
                    self.changed = false;
                }

                Ok(Payload {
                    header: HeaderType::Reply,
                    data: vec![self.counter, byte1, byte2],
                })
            }
            _ => Ok(Payload {
                header: HeaderType::Reply,
                data: vec![],
            }),
        }
    }

    fn get_address(&self) -> Address {
        99
    }

    fn set_bill_event(&mut self, bill_event: BillEvent) {
        self.bill_event = bill_event;
        self.changed = true;
    }
}
