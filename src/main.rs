use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

fn send_welcome(mut stream: TcpStream) -> TcpStream {
  let welcome = "Welcome to the Rust Echo Server! \
  Your escape sequence is ctrl + [\n";
  let res = stream.write(welcome.as_bytes());
  match res {
    Ok(res) => {
      println!("wrote {:?} bytes", res);
    }
    Err(e) => {println!("sending welcome {:?}", e)}
  }
  return stream;
}


fn handle_client(mut stream: TcpStream) {
  println!("client handled");
  let datain = &mut[0; 128];
  let escape_str = "\u{1b}\r\n";
  stream = send_welcome(stream);
  loop {
    let res = stream.read(datain);
    match res{
      Ok(res) => {
        let bytes_in = res;
        let s = str::from_utf8(&datain[0..bytes_in]);
        match s {
          Ok(s) => {
            if s == escape_str {
              println!("escape sequence recv'd. Exiting");
              break;
            }
          }
          Err(e) => {println!("{:?}", e)}
        }
        let res = stream.write(&datain[0..bytes_in]);
        match res {
          Ok(res) => {}
          Err(e) => {println!("write error {:?}", e)}
        }
      }
      Err(e) => {println!("read error {:?}", e)}
    }
  }
}

fn main(){
  let listener = TcpListener::bind("0.0.0.0:8002").unwrap();
  let x = listener.local_addr();
  let t = x.unwrap();
  println!("{}", t);
  println!("accepting connections");
  // accept connections and process them, spawning a new thread for each one
  for stream in listener.incoming() {
    println!("connection recv'd");
    match stream {
      Ok(stream) => {
        thread::spawn(move|| {
          // connection succeeded
          handle_client(stream)
        });
      }
      Err(e) => {println!("stream iteration {:?}", e)}
    }
  }
  
  // close the socket server
  drop(listener);
}
