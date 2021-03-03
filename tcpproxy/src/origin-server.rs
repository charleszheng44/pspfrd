use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};
use std::{str, fmt, thread};
#[macro_use]
extern crate tcpproxy;
use tcpproxy::consts;

#[allow(dead_code)]
struct RequestLine {
    method: Option<String>,
    path: Option<String>,
    protocol: Option<String>,
}

#[derive(Debug)]
struct ParseRequestError{
    request: String
}

impl fmt::Display for ParseRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fail to parse the request: {}", self.request)
    }
}

fn gen_request_line<'a>(request_msg: &'a str) -> Result<RequestLine, ParseRequestError>{
    let fst_line_tokens: Vec<&'a str>= request_msg.split_whitespace().collect();
    if fst_line_tokens.len() != 3 {
        return Err(ParseRequestError{
            request: format!("invalid request message {}", request_msg),
        });
    }
    Ok(RequestLine{
        method: Some(String::from(fst_line_tokens[0])),
        path: Some(String::from(fst_line_tokens[1])),
        protocol: Some(String::from(fst_line_tokens[2])),
    })
}

fn parse_path<'a>(path: &'a str) -> Result<String, ParseRequestError> {
    let path_tokens: Vec<&'a str> = path.split('/').collect();
    // e.g. /order/status/1 
    if path_tokens.len() != 4 || 
       path_tokens[1] != "order" || 
       path_tokens[2] != "status" || 
       !is_string_numeric(path_tokens[3]){
        return Err(ParseRequestError{
            request: format!("invalid path {}", path),
        });
    }
    Ok(format!("the order {} has been shipped", path_tokens[3]))
}

fn is_string_numeric<'a>(inp_str: &'a str) -> bool {
    for c in inp_str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

fn gen_err_response<'a>(content: &'a str) -> String {
    format!("HTTP/1.1 404 Not Found
Content-Type: text/html
Content-Length:{}

{}", content.len(), content)
}

fn gen_ok_response<'a>(content: &'a str) -> String {
    format!("HTTP/1.1 200 OK
Content-Type: text/html
Content-Length:{}

{}",
content.len(),
content)
}

fn parse_request(rl: RequestLine) -> String {
    match rl.method.unwrap().as_ref() {
        "GET" => {
             match parse_path(&rl.path.unwrap()) {
                 Ok(rep_msg) => gen_ok_response(rep_msg.as_ref()),
                 Err(e) => gen_err_response(e.to_string().as_ref()),
             }
        },
        invalid_method @ _ => {
            let err_content = format!("invalid method {}", invalid_method);
            gen_err_response(err_content.as_ref())
        },
    }
}

fn handle_conn(mut stream: TcpStream) {
    // 1. read the first line from the stream 
    let mut fst_line = String::new();
    let mut buf_reader = BufReader::new(
        stream.try_clone().expect("TODO"));
    buf_reader.read_line(&mut fst_line).expect("TODO");
    let request_line = gen_request_line(fst_line.as_ref()).expect("TODO");
    let response = parse_request(request_line);
    stream.write(response.as_bytes()).expect("TODO");
}

fn main() {
    let listener = TcpListener::bind(consts::ORIG_SERVER_ADDR).unwrap();
    log!("this");
    log!("TCPServer is listening at {}", consts::ORIG_SERVER_ADDR);
    for stream in listener.incoming()  {
        let stream = stream.expect(""); 
        thread::spawn(move || handle_conn(stream));
    }
}
