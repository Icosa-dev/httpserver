// use httpserver::ThreadPool;
// use std::{
//     fs, io::{BufReader, prelude::*}, net::{TcpListener, TcpStream}, thread, time::Duration
// };

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
//     let pool = ThreadPool::new(4);

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         pool.execute(|| {
//             handle_connection(stream);
//         });
//     }

//     println!("Shutting down...");
// }

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     let (status_line, filename) = match &request_line[..] {
//         "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./public/index.html"),
//         "GET /sleep HTTP/1.1" => {
//             thread::sleep(Duration::from_secs(5));
//             ("HTTP/1.1 200 OK", "./public/index.html")
//         }
//         _ => ("HTTP/1.1 404 NOT FOUND", "./public/404.html"),
//     };

//     let contents = fs::read_to_string(filename).unwrap();
//     let length = contents.len();

//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

//     stream.write_all(response.as_bytes()).unwrap();
// }

use std::error::Error;
use std::time::Duration;

use tokio::fs;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::sleep;

use httpserver::get_server_config;
use httpserver::log::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_config = get_server_config()?;

    let listener = TcpListener::bind(server_config.get_socket()).await?;
    // TODO: Add logging instead of print
    log("Server running on 127.0.0.1:7878", LogStatus::Ok).await;

    loop {
        let (socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let (reader, _) = stream.split();

    let mut buf_reader = BufReader::new(reader);
    let mut request_line = String::new();

    if let Err(error) = buf_reader.read_line(&mut request_line).await {
        return Err(Box::new(error));
    }

    let (status_line, filename) = match request_line.trim_end() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./public/index.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(5)).await;
            ("HTTP/1.1 200 OK", "./public/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "./public/404.html"),
    };

    // TODO: this data should probably go in a struct
    let contents = fs::read_to_string(filename).await.unwrap(); // this should not unwrap
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
