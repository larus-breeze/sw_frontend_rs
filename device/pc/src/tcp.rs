use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    time::Duration, vec::Vec,
};

const READ_TIMEOUT: Option<Duration> = Some(Duration::from_micros(1));

pub struct TcpInterface {
    opt_socket: Option<TcpStream>,
    listener: TcpListener,
    data: Vec<String>,
}

impl TcpInterface {
    pub fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).unwrap();
        TcpInterface { opt_socket: None, listener, data: Vec::new() }
    }

    pub fn send(&mut self, sentence: &str) {
        if self.opt_socket.is_some() {
            self.data.push(String::from(sentence));
        }
    }

    pub fn flush(&mut self) {
        if self.opt_socket.is_none() {
            match self.listener.accept() {
                Ok((socket, _addr)) => {
                    let r = socket.set_read_timeout(READ_TIMEOUT);
                    println!("socket.set_read_timeout() {:?}", r);
                    self.opt_socket = Some(socket);
                    println!("OK  Socket");
                }
                Err(e) => println!("NOK accept() {:?}", e),
            }
        }
        if self.send_data().is_err() {
            self.opt_socket = None;
        } 
        self.receive_data();
    }

    fn send_data(&mut self) -> Result<(), ()> {
        match self.opt_socket {
            Some(ref mut socket) => {
                while self.data.len() > 0 {
                    let s = self.data.pop().ok_or(())?;
                    socket.write_all(s.as_bytes()).map_err(|_| ())?;
                }
                Ok(())
            }
            None => Err(())
        }
    }

    fn receive_data(&mut self) {
        if let Some(ref mut socket) = self.opt_socket {
            let mut s = String::new();
            let r = socket.read_to_string(&mut s);
            match r {
                Ok(size) => println!("OK  size {}, s {}", size, s),
                Err(e) => println!("NOK {:?}", e),
            }
        }
    }

}
