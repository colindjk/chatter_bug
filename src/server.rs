extern crate mio;

use mio::tcp::*;
use mio::*;

use std::collections::HashMap;

use client::*;

pub struct WebSocketServer {
    socket: TcpListener,
    clients: HashMap<Token, TcpStream>,
    token_counter: usize,
}

const SERVER_TOKEN: Token = Token(0);

impl WebSocketServer {
    pub fn new(new_socket: TcpListener,
               new_clients: HashMap<Token, TcpStream>,
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
             token: Token, _events: EventSet)
    {
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

                self.clients.insert(new_token, client_socket);
                event_loop.register(&self.clients[&new_token], // wowzers in me trousers. Hashmaps!
                                    new_token, EventSet::readable(),
                                    PollOpt::edge() | PollOpt::oneshot()).unwrap();
            }
            token => {
                let mut client = self.clients.get_mut(&token.unwrap());
                client.read();
                event_loop.reregister(&client.socket, token, 
                                      client.interest,
                                      PollOpt::edge() | PollOpt::oneshot()).unwrap();
            }
            _ => panic!(),
        }
    }
}

//token => {
    //let mut client = self.clients.get_mut(&token).unwrap();
    //client.read();
    //event_loop.reregister(&client.socket, token, EventSet::readable(),
                          //PollOpt::edge() | PollOpt::oneshot()).unwrap();
//}
