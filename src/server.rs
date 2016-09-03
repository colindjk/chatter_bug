extern crate mio;

use mio::*;
use mio::tcp::*;

use std::collections::HashMap;
use std::sync::mpsc;

use std::io::Read;

use client::WebSocketClient;

pub struct WebSocketServer {
    pub socket: TcpListener,
    clients: HashMap<Token, WebSocketClient>,
    token_counter: usize,
}

const SERVER_TOKEN: Token = Token(0);

impl WebSocketServer {
    pub fn new(new_socket: TcpListener,
               new_clients: HashMap<Token, WebSocketClient>,
               new_token_counter: usize) -> WebSocketServer {
        WebSocketServer {
            socket: new_socket,
            clients: new_clients,
            token_counter: new_token_counter
        }
    }

    pub fn get_socket(&self) -> &TcpListener {
        &self.socket
    }
}

impl Handler for WebSocketServer {
    type Timeout = usize;
    type Message = ();

    fn ready(&mut self, event_loop: &mut EventLoop<WebSocketServer>,
             token: Token, events: EventSet) {

        if events.is_readable() {  
            match token {
                SERVER_TOKEN => {
                    let client_socket = match self.socket.accept() {
                        Err(e) => {
                            println!("Accept error: {}", e);
                            return;
                        },
                        Ok(None) => unreachable!("Accept has returned 'None'"),
                        Ok(Some((sock, _addr))) => sock
                    };

                    self.token_counter += 1;
                    let new_token = Token(self.token_counter);

                    self.clients.insert(new_token, WebSocketClient::new(client_socket));
                    event_loop.register(&self.clients[&new_token].socket,
                                        // wowzers in me trousers, hashmaps!
                                        new_token, EventSet::readable(),
                                        PollOpt::edge() | PollOpt::oneshot()).unwrap();
                }
                token => {
                    let mut client = self.clients.get_mut(&token).unwrap();
                    client.read();
                    event_loop.reregister(&client.socket, token,
                                          client.interest,
                                          PollOpt::edge() | PollOpt::oneshot()).unwrap();
                }
            }

        }
        // Handle write events for when the socket becomes available, edge based!
        if events.is_writable() {
            let mut client = self.clients.get_mut(&token).unwrap();
            client.write();
            event_loop.reregister(&client.socket, token,
                                  client.interest,
                                  PollOpt::edge() | PollOpt::oneshot()).unwrap();
        }
    }
}

//token => {
    //let mut client = self.clients.get_mut(&token).unwrap();
    //client.read();
    //event_loop.reregister(&client.socket, token, EventSet::readable(),
                          //PollOpt::edge() | PollOpt::oneshot()).unwrap();
//}
