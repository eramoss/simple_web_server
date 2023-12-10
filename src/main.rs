use simple_web_server::http_request::HttpRequest;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

fn handle_connection(stream: &TcpStream) -> Result<HttpRequest, ()> {
    let buffer_read = BufReader::new(stream);
    let request: HttpRequest = buffer_read
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    Ok(request)
}

fn send_response(buf: &[u8], mut stream: &TcpStream) -> std::io::Result<()> {
    stream
        .write_all(buf)
        .expect(format!("cannot write on stream {:#?}", stream).as_str());

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", 8080))?;

    for stream in listener.incoming() {
        let stream = stream?;
        let request =
            handle_connection(&stream).expect("unable to handle connection to read stream");

        send_response(b"hello warudo", &stream)?;
        println!("{}", request.value());
    }

    Ok(())
}
