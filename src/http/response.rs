#[derive(Debug)]
pub struct Response {
    status: u16,
    content: String,
    headers: Vec<(String, String)>
}

impl Response {
    pub fn new() -> Response {
        Response {
            status: 200,
            content: "".to_string(),
            headers: Vec::new(),
        }
    }

    pub fn set_status(&mut self, status: u16) {
        self.status = status;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn add_header(&mut self, header: String, value: String) {
        self.headers.push((header, value))
    }

    pub fn to_string(&self) -> String {
        let contents = &self.content;
        let length = contents.len();
        let status = self.status;
        let status_msg = self.status_string();
        let status_line = format!("HTTP/1.1 {status} {status_msg}");
        let headers = self.headers.iter()
            .map(|(header, value)| format!("{}: {}", header, value))
            .collect::<Vec<String>>()
            .join("\r\n");
        return format!(
            "{status_line}\r\nContent-Length: {length}\r\n{headers}\r\n\r\n{contents}"
        );
    }

    fn status_string(&self) -> &str {
        match self.status {
            200 => return "OK",
            400 => return "Bad Request",
            401 => return "Unauthorized",
            403 => return "Forbidden",
            404 => return "Not Found",
            500 => return "Internal Server Error",
            _ => return "unknown",
        }
    }
}

