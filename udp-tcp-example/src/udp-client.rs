use std::net::UdpSocket;
use udp_tcp_example::consts;
use std::{str, thread, time};

fn main() {
    let mut handlers = vec![];
    for id in 0..consts::NUM_CLIENT {
        thread::sleep(time::Duration::from_micros(1));
        let handler = thread::spawn(move|| send_udp_msg(id));
        handlers.push(handler);
    }

    for h in handlers {
        h.join().expect("fail to wait the thread to complete");
    }
}

fn send_udp_msg(client_id: usize) {
    // send message to the server
    let udp_skt = UdpSocket::bind("0.0.0.0:0").
        expect("fail to bind to udp socket");
    udp_skt.send_to(
        format!("{} from {}", consts::CLIENT_MESSAGE, client_id).as_bytes(), 
        consts::SERVER_ADDR).
        expect("unable to send bytes to server");
    println!("successfully send message to server {}", consts::SERVER_ADDR);
    // wait for response from the server
    let mut buf = [0; consts::BUFFER_SIZE];
    match udp_skt.recv_from(&mut buf) {
        Ok((bytes_read, _)) => {
            let content_rcv = str::from_utf8(&mut buf[..bytes_read]).
                expect("fail to convert bytes array to str");
            println!("receive response \"{}\" from the server.", content_rcv);
        },
        Err(e) => panic!("fail to receive response from the server {}", e),
    }
}
