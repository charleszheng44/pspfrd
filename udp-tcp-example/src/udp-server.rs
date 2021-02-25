use std::net::UdpSocket;
use std::thread;
use std::str;
use udp_tcp_example::consts;

fn main() {
    let udp_skt = UdpSocket::bind(consts::SERVER_ADDR).
        expect(&format!("fail to bind udp socket at {}", consts::SERVER_ADDR));
    println!("udp server is listening at {}", consts::SERVER_ADDR);
    loop {
        let new_udp_skt = udp_skt.try_clone().expect("fail to clone udp socket");
        let mut buf  = [0; consts::BUFFER_SIZE];
        match new_udp_skt.recv_from(&mut buf) {
            Ok((bytes_read, src_addr)) => {
                thread::spawn(move || {
                    let content_rcv = str::from_utf8(&mut buf[..bytes_read]).
                        expect("fail to convert bytes array to string");
                    println!("receive content \"{}\" from {}", content_rcv, src_addr);
                    // reverse the string and send it back
                    let content_snd = content_rcv.chars().
                        rev().collect::<String>();
                    new_udp_skt.send_to(content_snd.as_bytes(), src_addr).
                        expect("fail to send response back to the client");
                    println!("successfully send response to client {}", src_addr)
                });
            },
            Err(e) => println!("fail to receive buffer: {}", e),
        }
    }
}
