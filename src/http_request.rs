#[derive(Debug)]
pub struct HttpRequest {
    value: String,
}

impl HttpRequest {
    pub fn value(&self) -> String {
        self.value.to_string()
    }
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
