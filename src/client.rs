extern crate http_muncher;
use http_muncher::{ Parser, ParserHandler };

use mio::*;
use mio::tcp::*;
use std::net::SocketAddr;
use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;

use server::*;

struct HttpParser {
    current_key: Option<String>,
    headers: Rc<RefCell<HashMap<String, String>>>
}

impl ParserHandler for HttpParser {

    fn on_header_field(&mut self, s: &[u8]) -> bool {
        self.current_key = Some(std::str::from_utf8(s).unwrap().to_string());
        true
    }

    fn on_header_value(&mut self, s: &[u8]) -> bool {
        self.headers.borrow_mut()
            .insert(self.current_key.clone().unwrap(),
                    std::str::from_utf8(s).unwrap().to_string());
        true
    }

    fn on_headers_complete(&mut self) -> bool {
        false
    }
}

pub struct WebSocketClient {
    socket: TcpStream,
    http_parser: Parser<HttpParser>,
    headers: Rc<RefCell<HashMap<String, String>>>,

    interest: EventSet,
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
    fn read(&mut self) {
        loop {
            let mut buf = [0 as u8; 2048];
            match self.socket.try_read(&mut buf) {
                Err(e) => {
                    println!("Error reading socket number {:?}", e);
                    return
                }
                Ok(None) => break, // out of bytes
                Ok(Some(len)) => {
                    self.http_parser.parse(&buf[0..len]);
                    self.state = ClientState::HandshakeResponse;
                    self.interest.remove(EventSet::readable());
                    self.interest.insert(EventSet::writable());
                    break;
                }
            }
        }
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
}

//if self.http_parser.is_upgrade() {
    //break;
//}
