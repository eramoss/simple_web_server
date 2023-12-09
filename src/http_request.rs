#[derive(Debug)]
pub struct HttpRequest {
    value: String,
}

impl HttpRequest {
    pub fn value(&self) -> String {
        self.value.to_string()
    }
}

/// Used to create a new HttpRequest instance from a Vec<String>.
/// The buffer read from the stream of TcpStream is returned as a Vec<String>
///
/// # Example
/// ``` rust
///   use simple_web_server::http_request::HttpRequest;
///   
///   let some_vec: Vec<String> = vec!["foo".to_string(), "bar".to_string(), "test".to_string()];
///   let http_req = HttpRequest::from_iter(some_vec);
///   assert_eq!(http_req.value(), String::from("foo\nbar\ntest"))
impl FromIterator<String> for HttpRequest {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self {
            value: iter
                .into_iter()
                .reduce(|acc, e| format!("{}\n{}", acc, e).to_string())
                .unwrap(),
        }
    }
}
