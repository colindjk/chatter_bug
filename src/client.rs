extern crate http_muncher;
use std::collections::HashMap;

struct HttpParser;
impl ParserHandler for HttpParser {  }

pub struct WebSocketClient {
    socket: TcpStream,
    http_parser: Parser<HttpParser>
}

impl WebSocketClient {
    fn read(&mut self) {
        loop {
            let mut buf = [0; 2048];
            match self.socket.try_read(&buf) {
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

    fn new(socket: TcpStream) -> WebSocketClient {
        WebSocketClient {
            socket: socket,
            http_parser: Parser::request(HttpParser)
        }
    }
}
