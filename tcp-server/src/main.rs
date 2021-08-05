use redisish;

use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // TODO: initialize storage

    for connection in listener.incoming() {
        let stream = match connection {
            Ok(stream) => stream,
            Err(e) => {
                println!("Error occurred: {:?}", e);
                continue;
            }
        };

        let res = handle(stream);

        if let Err(e) = res {
            println!("Error occurred: {:?}", e);
        }
    }

    Ok(())
}

fn handle(mut stream: TcpStream) -> Result<(), ServerError> {
    todo!("read from stream and add data to storage")
}
