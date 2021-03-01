use std::net::TcpListener;
use std::io::Read;
use tcpproxy::consts;
use std::str;
use std::fmt;

struct RequestLine {
    method: Option<String>,
    path: Option<String>,
    protocol: Option<String>,
}

struct ParseRequestError{
    request: String
}

impl fmt::Display for ParseRequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fail to parse the request {}", self.request)
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

fn main() {
    let listener = TcpListener::bind(consts::ORIG_SERVER_ADDR).unwrap();
    let (mut stream, cli_addr) = listener.accept().unwrap();
    let mut buf  = [0; consts::BUFFER_SIZE];
    stream.read(&mut buf).unwrap();
    println!("receive message from client {}:\n {}", 
        cli_addr, str::from_utf8(&buf).unwrap())
}
