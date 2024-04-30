use std::io::ErrorKind;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Receiver, SendError, Sender, TryRecvError};
use std::time::Duration;

use botwfddata::Payload;
use deku::prelude::*;
use tungstenite::{Message, WebSocket, Error as WebSocketError};

/// Max number of clients that can be connected at once.
/// 
/// We are using 2 threads per client approach.
/// Too many clients can cause the server to slow down, which
/// is not ideal in a real-time application.
const MAX_CLIENTS: usize = 4;
/// Max continous write error before closing a client
const MAX_CONTINOUS_ERROR: u32 = 4;

/// Server side client handler
#[derive(Debug)]
struct Client {
    /// Client identifier
    id: u32,
    /// The thread handling the client connection
    thread: Option<JoinHandle<()>>,
    /// Sender used to send the payload to the client
    send: Option<Sender<Arc<Payload>>>,
}

impl Client {
    /// Start the client communication over the socket
    pub fn start(id: u32, mut socket: WebSocket<TcpStream>) -> Self {
        let (send, recv) = mpsc::channel::<Arc<Payload>>();
        let thread = thread::spawn(move || {
            //let socket = Arc::new(Mutex::new(socket));
            //let read_thread = client_read_thread(id, socket.clone());
            let id = id.to_string();
            
            // send loop
            // ok to block while waiting for payload since we hare handling reads from another thread
            let mut continuous_error_count = 0;

            for payload in recv {
                let bytes = match payload.to_bytes() {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        crate::error!("[client {id}] failed to serialize payload: {e}");
                        continue;
                    }
                };
                if let Err(e) = socket.send(Message::Binary(bytes)) {
                    match e {
                        WebSocketError::ConnectionClosed | WebSocketError::AlreadyClosed => {
                            // client is disconnected, break the loop to handle clean up
                            break;
                        }
                        other => {
                            crate::error!("[client {id}] failed to send message to client: {other}");
                            continuous_error_count += 1;
                            if continuous_error_count >= MAX_CONTINOUS_ERROR {
                                crate::error!("[client {id}] too many continuous errors, closing client");
                                break;
                            }
                            continue;
                        }
                    }
                }
                continuous_error_count = 0;
            }
            crate::info!("[client {id}] disconnecting client");
            continuous_error_count = 0;
            socket.close(None);
            crate::info!("[client {id}] completing websocket closing handshake");
            loop {
                if !socket.can_read() {
                    break;
                }
                match socket.read() {
                    Ok(_) => {
                        // we don't expect client to tell us anything, just ignore
                    }
                    Err(WebSocketError::ConnectionClosed) | Err(WebSocketError::AlreadyClosed) => {
                        // safe to drop the connection
                        break;
                    }
                    Err(e) => {
                        if continuous_error_count >= MAX_CONTINOUS_ERROR {
                            crate::warn!("[client {id}] too many continuous errors, force dropping the connection");
                            break;
                        }
                        continuous_error_count += 1;
                    }
                }
            }
            drop(socket);
            crate::info!("[client {id}] websocket closed");
            // if read_thread.join().is_err() {
            //     crate::warn!("[client {id}] read thread panicked");
            // }
            // crate::info!("[client {id}] stopped read thread");
            crate::info!("[client {id}] stopping thread");

            // sender will be closed on next send because the receiver is closed
        });
        
        Client {
            id,
            thread: Some(thread),
            send: Some(send),
        }
    }

    /// Close the connection with the client
    pub fn close(&mut self) {
        if let Some(send) = self.send.take() {
            // close the sender, which should allow the write thread to stop and send the close message
            drop(send);
            // wait for the write thread to stop
            if let Some(thread) = self.thread.take() {
                if thread.join().is_err() {
                    crate::warn!("[client {}] client thread panicked", self.id);
                }
            }
            crate::info!("[client {}] stopped client thread", self.id);
            crate::info!("[client {}] disconnected", self.id);
        }
    }

    pub fn is_closed(&self) -> bool {
        self.send.is_none()
    }

    /// Add a payload to the send queue to the client, returns the payload if the send failed. The client should be closed if this fails.
    pub fn send(&self, payload: Arc<Payload>) -> Result<(), Arc<Payload>> {
        match &self.send {
            Some(send) => {
                send.send(payload).map_err(|e| e.0)
            }
            None => {
                Err(payload)
            }
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.close();
    }
}

/// Server handler
/// cbindgen:ignore
pub struct Server {
    /// Server identifier
    id: u32,
    /// If the server host is exposed in network (i.e. binded to 0.0.0.0)
    expose_host: bool,
    /// Port the server is running on
    port: u16,
    /// If the server is running
    running: Arc<AtomicBool>,
    /// The thread handling the incoming connection
    server_thread: Option<JoinHandle<()>>,
    /// The thread handling the sending of payloads to clients
    send_thread: Option<JoinHandle<()>>,
    /// Sender used to send payloads to the clients
    send: Option<Sender<Payload>>,
}

impl Server {
    /// Start the server thread
    pub fn start(server_id: u32, expose_host: bool, port: u16) -> Self {
        let addr = if expose_host { "0.0.0.0" } else { "localhost" };
        crate::info!("starting server on {addr}:{port}");
        let (client_send, client_recv) = mpsc::channel::<Client>();
        
        let running = Arc::new(AtomicBool::new(true));
        let server_thread = {
            let addr = addr.to_string();
            let running = running.clone();
            thread::spawn(move || {
                crate::info!("[server {server_id}] starting server thread");
                let server = TcpListener::bind(format!("{addr}:{port}").as_str()).unwrap();
                let mut id: u32 = 1;
                for stream in server.incoming() {
                    if !running.load(Ordering::SeqCst) {
                        break;
                    }
                    let stream = match stream {
                        Ok(stream) => stream,
                        Err(e) => {
                            crate::error!("[server {server_id}] failed to accept client: {e}");
                            continue;
                        }
                    };
                    // if let Err(e) = stream.set_nonblocking(true) {
                    //     crate::error!("[server {server_id}] failed to set stream to non-blocking: {e}");
                    //     continue;
                    // }
    
                    let ws = match tungstenite::accept(stream) {
                        Ok(ws) => ws,
                        Err(e) => {
                            crate::error!("[server {server_id}] failed to establish websocket handshake: {e}");
                            continue;
                        }
                    };
                    let current_id = id;
                    id += 1;
                    crate::info!("[server {server_id}] connecting client {current_id}");
                    let client = Client::start(current_id, ws);
                    if let Err(e) = client_send.send(client) {
                        crate::error!("[server {server_id}] failed to send client to server thread: {e}");
                        // if the recv is closed, the server is not working and should be stopped
                        break;
                    }
                    
                }
                crate::info!("[server {server_id}] stopping server thread");
                // stop sending clients
                drop(client_send);
                
                // close the server
                drop(server);

                running.store(false, Ordering::SeqCst);
            })
        };
        let (send, recv) = mpsc::channel::<Payload>();
        let send_thread = thread::spawn(move || {
                let mut clients = Vec::with_capacity(MAX_CLIENTS);

                for payload in recv {
                    // check if there are new clients
                    match client_recv.try_recv() {
                        Ok(mut client) => {
                            if clients.len() >= MAX_CLIENTS {
                                crate::warn!("[server {server_id}] max clients reached, not accepting new client");
                                client.close();
                            } else {
                                let id = client.id;
                                clients.push(client);
                                crate::info!("[server {server_id}] client {id} connected");
                            }
                        }
                        Err(TryRecvError::Empty) => {}
                        Err(TryRecvError::Disconnected) => {
                            break;
                        }
                    }
                    let payload = Arc::new(payload);
                    let mut need_cleanup = false;
                    for client in &mut clients {
                        if client.send(payload.clone()).is_err() {
                            crate::error!("[server {server_id}] failed to send payload to client {}", client.id);
                            crate::info!("[server {server_id}] disconnecting client {}", client.id);
                            client.close();
                            need_cleanup = true
                        }
                    }
                    // clean up closed clients
                    if need_cleanup {
                        clients.retain(|c| !c.is_closed());
                    }
                }
                // close all clients
                for client in clients.iter_mut() {
                    client.close();
                }
                crate::info!("[server {server_id}] stopping send thread");
            });
        
        Server {
            id: server_id,
            expose_host,
            port,
            running,
            server_thread: Some(server_thread),
            send_thread: Some(send_thread),
            send: Some(send),
        }
    }
    pub fn shutdown(&mut self) {
        if self.running.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            crate::warn!("[server {}] server is already shutdown!", self.id);
            return;
        }
        crate::info!("[server {}] shutting down server", self.id);
        if let Some(send) = self.send.take() {
            // close the sender, which should allow the receiver thread to close
            drop(send);
        }
        // connect to itself so the server thread can exit
        let _ = TcpStream::connect(format!("localhost:{}", self.port));
        // join the threads
        if let Some(thread) = self.server_thread.take() {
            if thread.join().is_err() {
                crate::warn!("[server {}] server thread panicked", self.id);
            }
            crate::info!("[server {}] stopped server thread", self.id);
        }
        if let Some(thread) = self.send_thread.take() {
            if thread.join().is_err() {
                crate::warn!("[server {}] send thread panicked", self.id);
            }
            crate::info!("[server {}] stopped send thread", self.id);
        }

        crate::info!("[server {}] stopped", self.id);
    }

    /// Return true if the server is running, if this returns false, it's might not be fully stopped yet unless shutdown() is returned
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn is_exposed(&self) -> bool {
        self.expose_host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn send(&self, payload: Payload) {
        if let Some(send) = &self.send {
            let _ = send.send(payload);
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.shutdown();
    }
}
