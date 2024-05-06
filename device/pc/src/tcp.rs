use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str,
    sync::mpsc::{self, channel, Receiver, Sender},
    thread,
};

pub struct TcpServer {
    rx: Option<mpsc::Receiver<Vec<u8>>>,
    tx: Option<mpsc::Sender<Vec<u8>>>,
    com: mpsc::Receiver<(mpsc::Receiver<Vec<u8>>, mpsc::Sender<Vec<u8>>)>,
    rec_str: Vec<u8>,
}

impl TcpServer {
    pub fn new(addr: &str) -> Self {
        let listener = TcpListener::bind(addr).unwrap();
        let (tx, rx) = mpsc::channel();

        let mut socket = Socket { listener };
        thread::spawn(move || socket.connect(tx));
        TcpServer { rx: None, tx: None, com: rx, rec_str: Vec::new() }
    }

    fn check_connection(&mut self) {
        match self.com.try_recv() {
            Ok((rx, tx)) => {
                self.rx = Some(rx);
                self.tx = Some(tx);
            },
            Err(_) => (),
        }
    }

    pub fn recv(&mut self) -> Option<Vec<u8>> {
        self.check_connection();
        match &self.rx {
            Some(rx) => {
                match rx.try_recv() {
                    Ok(s) => self.rec_str.extend(s),
                    Err(_) => (),
                }
            }
            None => (),
        }

        if let Some(idx) = self.rec_str.iter().position(|&x| x==b'\n') {
            let mut r = self.rec_str.clone();
            self.rec_str = r.split_off(idx+1);

            if r.len() > 0 {
                Some(r)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn send(&mut self, s: &[u8]) {
        self.check_connection();
        let s = Vec::from(s);
        match &self.tx {
            Some(tx) => {
                let _ = tx.send(s);
            },
            None => (),
        }
    }
}

struct Socket {
    listener: TcpListener,
}

impl Socket {
    fn connect(&mut self, com: mpsc::Sender<(mpsc::Receiver<Vec<u8>>, mpsc::Sender<Vec<u8>>)>) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("[TcpServer] new connection");

                    // create 2 channels
                    let (tx, rx) = channel();
                    let (tx2, rx2) = channel();
                    let tx_stream = stream.try_clone().unwrap();

                    // spawn rx and tx thread
                    thread::spawn(move || { rx_handler(stream, tx);});
                    thread::spawn(move || { tx_handler(tx_stream, rx2);});

                    // inform TcpServer of new connection
                    com.send((rx, tx2)).unwrap();
                }
                Err(_) => {
                    println!("Error");
                }
            }
        }
    }
}

fn rx_handler(mut stream: TcpStream, tx: Sender<Vec<u8>>) {
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    println!("[TcpServer] connection closed");
                    break;
                }
                let s = &read[0..n];
                tx.send(s.to_vec()).unwrap();
            }
            Err(_) => (),
        }
    }
}

fn tx_handler(mut stream: TcpStream, rx: Receiver<Vec<u8>>) {
    loop {
        match rx.try_recv() {
            Ok(s) => {
                let _ = stream.write(&s[..]);
            }
            Err(_) => (),
        }
    }
}

