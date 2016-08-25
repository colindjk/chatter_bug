extern crate http_muncher;
use http_muncher::{ Parser, ParserHandler };

use mio::*;
use mio::tcp::*;
use std::net::SocketAddr;
use std::collections::HashMap;

use server::*;

struct HttpParser;
impl ParserHandler for HttpParser {  }

pub struct WebSocketClient {
    socket: TcpStream,
    http_parser: Parser<HttpParser>
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
                    if self.http_parser.is_upgrade() {
                        break;
                    }
                }
            }
        }
    }

    pub fn new(socket: TcpStream) -> WebSocketClient {
        WebSocketClient {
            socket: socket,
            http_parser: Parser::request(HttpParser)
        }
    }
}

