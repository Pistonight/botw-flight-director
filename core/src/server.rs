use std::net::TcpListener;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};

use botwfddata::Payload;

pub struct Client {
    pub stream: TcpStream,
}

#[repr(C)]
pub struct Server {
    pub port: u16,
    pub accepting_thread: JoinHandle<()>,
    pub handling_thread: JoinHandle<()>,
    pub command_sender: Option<Sender<Payload>>,
    pub client_receiver: Option<Receiver<Client>>,
    pub client_sender: Option<Sender<Client>>,
}

impl Server {
    pub fn new(addr: &str, port: u16) -> Self {
        let (send, recv) = mpsc::channel();
        let thread = thread::spawn(move || {
            let server = TcpListener::bind(format!("{addr}:{port}").as_str()).unwrap();
        });
        Server {
            port,
            thread,
            send: Some(send),
        }
    }
    pub fn shutdown(&mut self) {
        if let Some(send) = self.send.take() {
            // close the sender, which should allow the server thread to close
            drop(send);
        }
        let _ = self.thread.join();
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.shutdown();
    }
}

fn create_server() {
    let server = TcpListener::bind("0.0.0.0:9004").unwrap();
    for stream in server.incoming() {

    }
}