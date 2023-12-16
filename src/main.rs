use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn handle_connection(stream: &TcpStream) -> Result<String, ()> {
    let buffer_read = BufReader::new(stream);
    let request = buffer_read
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    Ok(request)
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(("0.0.0.0", 8080))?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        let _request =
            handle_connection(&stream).expect("unable to handle connection to read stream");

        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }

    Ok(())
}
