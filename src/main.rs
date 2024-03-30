use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

struct Response {
    status: String,
    content: String,
    length: usize,
}

impl Response {
    fn to_bytes(&self) -> Vec<u8> {
        let response_status = &self.status;
        let response_length = &self.length;
        let response_content = &self.content;

        let response = format!(
            "{response_status}\r\nContent-Length: {response_length}\r\n\r\n{response_content}"
        );

        response.into_bytes()
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn not_found() -> Vec<u8> {
    let response_content = fs::read_to_string("404.html").unwrap();
    let response = Response {
        content: response_content.to_owned(),
        length: response_content.len(),
        status: "HTTP/1.1 404 NOT FOUND".to_owned(),
    };

    return response.to_bytes();
}

fn get_uri(request_line: &String) -> String {
    let mut uri = String::from("");

    for (i, char) in request_line.chars().enumerate() {
        if i > 3 && request_line.chars().nth(i) != Some(' ') {
            uri.push(char);
        } else if i > 4 {
            break;
        }
    }
    return uri;
}

fn handle_connection(mut stream: TcpStream) {
    let buff_reader = BufReader::new(&mut stream);
    let request_line = &buff_reader.lines().next().unwrap().unwrap();

    let uri = get_uri(request_line);

    let response = match request_line.as_str() {
        "GET / HTTP/1.1" => handle_get_request(uri),
        _ => not_found(),
    };

    stream.write_all(&response).unwrap();
}

fn handle_get_request(uri: String) -> Vec<u8> {
    match uri.as_str() {
        "/" => {
            let response_content = fs::read_to_string("index.html").unwrap();
            let response = Response {
                content: response_content.to_owned(),
                length: response_content.len(),
                status: "HTTP/1.1 200 OK".to_owned(),
            };

            return response.to_bytes();
        }
        _ => not_found(),
    }
}
