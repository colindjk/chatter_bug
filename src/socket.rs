extern crate mio;

use mio::*;

pub struct WebSocketServer;

/// Handler interface only REQUIRES two values to be defined
impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();
}



