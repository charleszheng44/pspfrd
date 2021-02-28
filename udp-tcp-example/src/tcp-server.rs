use std::net::TcpListener;
use std::io::{Read, Write};
use udp_tcp_example::consts;
use std::thread;

fn main() {
    let tcp_listener = TcpListener::bind(consts::TCP_SERVER_ADDR).
        expect("fail to bind to socket");
    for stream in tcp_listener.incoming() {
        let mut tcp_stream = stream.unwrap();
        thread::spawn(move|| {
            println!("accept connection from {}", 
                tcp_stream.peer_addr().expect("TODO"));
            let mut cli_msg = String::new();
            tcp_stream.read_to_string(&mut cli_msg).
                expect("TODO");
            println!("the client said: {}", cli_msg);
            let reply_msg = cli_msg.chars().rev().collect::<String>();
            tcp_stream.write(reply_msg.as_bytes()).
                expect("TODO");
        });
    }
}
