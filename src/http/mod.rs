pub struct Response {
    pub status_code: StatusCode,
    pub content: String,
    pub length: usize,
}

pub enum StatusCode {
    PageNotFound,
    Ok,
}

fn status_code_to_string(status_code: &StatusCode) -> String {
    return match status_code {
        StatusCode::PageNotFound => String::from("404 NOT FOUND"),
        StatusCode::Ok => String::from("200 OK"),
    };
}

impl Response {
    pub fn to_bytes(&self) -> Vec<u8> {
        let response_status = status_code_to_string(&self.status_code);
        let response_length = &self.length;
        let response_content = &self.content;

        let response = format!(
            "HTTP/1.1 {response_status}\r\nContent-Length: {response_length}\r\n\r\n{response_content}"
        );

        response.into_bytes()
    }
}
