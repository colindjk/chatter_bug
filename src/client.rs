extern crate http_muncher;
use http_muncher::{ Parser, ParserHandler };

use mio::*;
use mio::tcp::*;
use std::net::SocketAddr;
use std::collections::HashMap;
use std::fmt;

use std::str;
use std::cell::RefCell;
use std::rc::Rc;

use server::*;
use serializer::*;

struct HttpParser {
    current_key: Option<String>,
    headers: Rc<RefCell<HashMap<String, String>>>
}

impl ParserHandler for HttpParser {

    fn on_header_field(&mut self, s: &[u8]) -> bool {
        self.current_key = Some(str::from_utf8(s).unwrap().to_string());
        true
    }

    fn on_header_value(&mut self, s: &[u8]) -> bool {
        self.headers.borrow_mut()
            .insert(self.current_key.clone().unwrap(),
                    str::from_utf8(s).unwrap().to_string());
        true
    }

    fn on_headers_complete(&mut self) -> bool {
        false
    }
}

pub struct WebSocketClient {
    pub socket: TcpStream,
    pub interest: EventSet,

    http_parser: Parser<HttpParser>,
    headers: Rc<RefCell<HashMap<String, String>>>,

    state: ClientState,
}

/// Enumeration defining all possible states of a particular client
#[derive(PartialEq)]
enum ClientState {
    AwaitingHandshake,
    HandshakeResponse,
    Connected
}


impl WebSocketClient {
    pub fn read(&mut self) {
        loop {
            let mut buf = [0 as u8; 2048];
            match self.socket.try_read(&mut buf) {
                Err(e) => {
                    println!("Error reading socket number {:?}", e);
                    return
                }
                Ok(None) => break, // out of bytes
                Ok(Some(len)) => {
                    //self.http_parser.parse(&buf[0..len]);

                    self.state = ClientState::HandshakeResponse;
                    self.interest.remove(EventSet::readable());
                    self.interest.insert(EventSet::writable());
                    break;
                }
            }
        }
    }

    pub fn write(&mut self) {
        let headers = self.headers.borrow();

        let response_key = gen_key(&headers.get("Sec-WebSocket-Key").unwrap());

        let response = fmt::format(format_args!("HTTP/1.1 101 Switching Protocols\r\n\
                                                 Connection: Upgrade\r\n\
                                                 Sec-WebSocket-Accept: {}\r\n\
                                                 Upgrade: websocket\r\n\r\n\
                                                ", response_key));

        self.socket.try_write(response.as_bytes()).unwrap();

        // Change the 'state'
        self.state = ClientState::Connected;

        // And change the interest back to readable, as it should be post-write
        self.interest.remove(EventSet::writable());
        self.interest.insert(EventSet::readable());
    }

    pub fn new(socket: TcpStream) -> WebSocketClient {
        let headers = Rc::new(RefCell::new(HashMap::new()));

        WebSocketClient {
            socket: socket,
            // clone of the first headers, there will be multiple
            // references to headers (Rc is immutable)
            headers: headers.clone(),

            http_parser: Parser::request(HttpParser {
                current_key: None,
                // to write new headers
                headers: headers.clone()
            }),

            // Initial events
            interest: EventSet::readable(),
            // initial state of a client is awaiting handshake
            state: ClientState::AwaitingHandshake
        }
    }

    //pub fn get_socket(&self) -> &TcpStream {
        //&self.socket
    //}

    //pub fn get_interest(&self) -> &EventSet {
        //&self.interest
    //}

}

//if self.http_parser.is_upgrade() {
    //break;
//}
