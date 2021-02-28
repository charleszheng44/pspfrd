use std::net::{TcpStream, TcpListener};
use std::io::{Write, BufReader, BufRead};
use udp_tcp_example::consts;
use std::thread;

fn handle_connect(mut tcp_stream: TcpStream) {
    println!("accept connection from {}", 
        tcp_stream.peer_addr().expect("fail to get the client address"));
    // read message from the client by line
    let mut cli_msg = String::new();
    let mut line_reader = BufReader::new(tcp_stream.try_clone().unwrap());
    line_reader.read_line(&mut cli_msg).
        expect("fail to read bytes from the TCP stream");
    println!("the client said: {}", cli_msg);
    // reverse the client message and send it back
    cli_msg = cli_msg.trim_end_matches('\n').to_string();
    let mut reply_msg = cli_msg.chars().rev().collect::<String>();
    reply_msg.push('\n');
    tcp_stream.write(reply_msg.as_bytes()).
        expect("fail to write bytes to the TCP stream");   
}

fn main() {
    let tcp_listener = TcpListener::bind(consts::TCP_SERVER_ADDR).
        expect("fail to bind to socket");
    println!("TCP server is listening at {}...", consts::TCP_SERVER_ADDR);
    for stream in tcp_listener.incoming() {
        let tcp_stream = stream.unwrap();
        thread::spawn(move|| handle_connect(tcp_stream));
    }
}
