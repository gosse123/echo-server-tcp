use std::{env, io::{self, Read, Write}, net::{TcpListener,TcpStream}};
use std::thread;


fn handler_client(mut stream:TcpStream) {
    let peer_addr = stream.peer_addr().map_or_else(|_|"unknown".to_string(), |addr|addr.to_string());
    println!("handling connection fromrfrv: {}",peer_addr);

    let mut buffer = [0;1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n)=>{
                if n == 0{
                    println!("client {} close connection ",peer_addr);
                    break; 
                }

                if let Err(e) = stream.write_all(&mut buffer[..n])  {
                    eprintln!("write error client {} {}",peer_addr,e);
                    break;
                }
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e)=>{
                match e.kind() {
                    io::ErrorKind::ConnectionReset=>{
                        println!("client {} reset connection ",peer_addr);
                    }
                    _=>{
                        eprintln!("read error from client {}, {}",peer_addr,e);
                    }
                }
                break;
            }
        }

    }
    println!("connection finish {}",peer_addr);
}
fn main() {
    let addr = env::args()
                    .nth(1)
                    .unwrap_or_else(| |"127.0.0.1:9000".to_string());   

    let listenner = TcpListener::bind(&addr).expect("failer to listnning server");
    println!("server listnning on {}",addr);

    for streame_tcp in listenner.incoming(){
        match streame_tcp {
            Ok(stream)=>{
                thread::spawn(move||{
                    handler_client(stream);
                });
            },
            Err(e)=>{
                eprintln!("faille to connected at a server:  {}",e);
            }
        }
    }
}