use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
struct HttpRequest {
    value: String,
}
impl FromIterator<String> for HttpRequest {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self {
            value: iter
                .into_iter()
                .reduce(|acc, e| format!("{}{}", acc, e))
                .unwrap(),
        }
    }
}
#[test]
fn http_request_from_vec_string() {
    let some_vec: Vec<String> = vec!["foo".to_string(), "bar".to_string(), "test".to_string()];
    let http_req = HttpRequest::from_iter(some_vec);

    assert_eq!(http_req.value, String::from("foobartest"))
}

fn handle_connection(stream: &TcpStream) -> Result<HttpRequest, ()> {
    let buffer_read = BufReader::new(stream);
    let request: HttpRequest = buffer_read
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    Ok(request)
}

fn main() -> std::io::Result<()> {
    let listener =
        TcpListener::bind(("127.0.0.1", 8080)).expect("unable to bind in localhost:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connected to stream '{:?}'", stream);
        let request =
            handle_connection(&stream).expect("unable to handle connection to read stream");

        println!("{:?}", request.value);
    }

    Ok(())
}
