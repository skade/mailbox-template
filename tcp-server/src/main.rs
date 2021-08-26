use redisish;

use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug)]
enum ServerError {
    ParseError(redisish::Error),
    IoError(std::io::Error),
}

impl From<redisish::Error> for ServerError {
    fn from(e: redisish::Error) -> ServerError {
        ServerError::ParseError(e)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(e: std::io::Error) -> ServerError {
        ServerError::IoError(e)
    }
}

fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let storage = Arc::new(Mutex::new(VecDeque::new()));

    for stream in listener.incoming() {
    
        let thread_storage = storage.clone();
        std::thread::spawn(move || {
            let res = stream.map_err(|e| e.into() )
            .and_then(|mut s| {
               handle(&mut s, &thread_storage)
            });

            if let Err(e) = res {
                println!("Error occured: {:?}", e);
            }
        });
        
    }

    Ok(())
}

fn handle(stream: &mut TcpStream, storage: &Mutex<VecDeque<String>>) -> Result<(), ServerError> {
    let command = read_command(stream)?;
    match command {
        redisish::Command::Publish(message) => {
            storage.lock().unwrap().push_back(message);
            Ok(())
        }
        redisish::Command::Retrieve => {
            let data = storage.lock().unwrap().pop_front();
            match data {
                Some(message) => write!(stream, "{}", message).map_err( |e| e.into() ),
                None => write!(stream, "No message in inbox!\n").map_err( |e| e.into() )
            }
        }
    }
}

fn read_command(stream: &mut TcpStream) -> Result<redisish::Command, ServerError> {
    let mut read_buffer = String::new();
    let mut buffered_stream = BufReader::new(stream);
    buffered_stream.read_line(&mut read_buffer)?;
    redisish::parse(&read_buffer).map_err( |e| e.into() )
}