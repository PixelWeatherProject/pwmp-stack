use crate::error::Error;
use log::debug;
use pwmp_types::{mac::Mac, request::Request, response::Response, Message, NodeId};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

const RCV_BUFFER_SIZE: usize = 128;
type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Client {
    id: Option<NodeId>,
    mac: Option<Mac>,
    socket: TcpStream,
    buf: [u8; RCV_BUFFER_SIZE],
}

impl Client {
    pub fn new(socket: TcpStream) -> Result<Self> {
        let mut client = Self {
            id: None,
            mac: None,
            socket,
            buf: [0; RCV_BUFFER_SIZE],
        };

        debug!("{}: Awaiting greeting", client.peer_addr_str());
        let mac = client.handle_hello()?;
        client.mac = Some(mac);
        debug!("{}: Is {}?", client.peer_addr_str(), client.mac());

        Ok(client)
    }

    pub fn id(&self) -> NodeId {
        self.id.unwrap()
    }

    pub fn set_id(&mut self, id: NodeId) {
        assert!(self.id.is_none(), "Cannot update node ID");
        self.id = Some(id);
    }

    pub fn mac(&self) -> Mac {
        self.mac.unwrap()
    }

    pub fn peer_addr(&self) -> Option<SocketAddr> {
        self.socket.peer_addr().ok()
    }

    fn peer_addr_str(&self) -> String {
        self.peer_addr()
            .map_or_else(|| "Unknown".to_string(), |addr| addr.to_string())
    }

    pub fn send_response(&mut self, resp: Response) -> Result<()> {
        let message = Message::Response(resp);
        debug!(
            "{}: responding with {:?} ({} bytes)",
            self.id
                .map_or_else(|| self.mac().to_string(), |id| id.to_string()),
            message.response().unwrap(),
            message.size()
        );
        self.socket.write_all(&message.to_raw())?;
        self.socket.flush()?;

        Ok(())
    }

    fn handle_hello(&mut self) -> Result<Mac> {
        let req = self.await_request()?;
        let Request::Hello { mac } = req else {
            return Err(Error::NotHello);
        };

        Ok(mac)
    }

    fn await_next_message(&mut self) -> Result<Message> {
        let read = self.socket.read(&mut self.buf)?;
        let message = Message::from_raw(&self.buf[..read]).ok_or(Error::MessageParse)?;

        Ok(message)
    }

    pub fn await_request(&mut self) -> Result<Request> {
        self.await_next_message()?
            .to_request()
            .ok_or(Error::NotRequest)
    }
}
