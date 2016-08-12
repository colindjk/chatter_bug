extern crate mio;

use mio::*;
use mio::tcp::*;
use std::net::SocketAddr;
use std::collections::HashMap;

mod socket;
mod server;

fn main() {
    // Note: unwrap() for Option::Some
    let mut event_loop = EventLoop::new().unwrap();
    let mut handler = socket::WebSocketServer;

    let address = "0.0.0.0:10000".parse::<SocketAddr>().unwrap();
    let server_socket = TcpListener::bind(&address).unwrap();

    let mut server = server::
        WebSocketServer::new(server_socket, HashMap::new(), 1);

    event_loop.register(server.get_socket(),
                        Token(0),
                        EventSet::readable(),
                        PollOpt::edge()).unwrap();

    event_loop.run(&mut handler).unwrap();

}

