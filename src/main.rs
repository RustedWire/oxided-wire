use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, Shutdown, TcpStream, TcpListener, IpAddr};
use std::thread;

mod config;

enum TypeMessage {
    SendPb  = 0b00,
    SendH   = 0b01,
    MSG     = 0b10,
}

struct SMSProto {
    type_msg: TypeMessage,
    size: u8,
    data: [u8; 255],
}

fn handle_client(mut stream_in: TcpStream, mut stream_out: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream_in.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream_out.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream_out.peer_addr().unwrap());
            stream_out.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn listen(){

}

fn main() {
    let toml_str = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    let config = config::Config::from(toml_str);

    let local_addr: SocketAddr = config.local.into();
    let remote_addr: SocketAddr = config.remote.into(); 


    let operator_stream = TcpStream::connect(remote_addr).expect("Error connecting other operator");
    let listener = TcpListener::bind(local_addr).unwrap();

    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let clone_operator_stream = operator_stream.try_clone().expect("Error cloning the tcp stream");
                thread::spawn(move || {
                    handle_client(stream, clone_operator_stream);
                });
                    
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
