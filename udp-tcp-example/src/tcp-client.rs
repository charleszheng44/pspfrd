use std::net::{self, TcpStream};
use std::io::{Read, Write};
use udp_tcp_example::consts;
use std::env;
use std::thread;


fn connect_to_server(id: usize) {
    let server_socket_addr = &consts::TCP_SERVER_ADDR.
        parse::<net::SocketAddr>().
        unwrap_or_else(|_| 
            panic!("fail to convert string to the socket address {}", 
                consts::TCP_SERVER_ADDR));

    let mut stream = TcpStream::connect_timeout(
        server_socket_addr, 
        consts::TCP_CONN_TIMEOUT_SEC).
        expect("fail to connect to the server");

    write!(stream, "{} {}", consts::TCP_CLIENT_MESSAGE, id).
        expect("fail to send message to the server");

    let mut buf = String::new();
    stream.read_to_string(&mut buf).
        expect("fail to read message from the server");

    println!("the server reply: {}", buf);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len()==2, "invalid command, please invoke the command \
        in the format of \"./tcp-client <num-clients>\"");
    let num_clis = args[1].parse::<usize>().
        expect("fail to parse args[1] to usize");
    let mut cli_handlers = vec![];

    for i in 1..=num_clis {
        let handler = thread::spawn(move || connect_to_server(i));
        cli_handlers.push((i, handler));
    }

    for h in cli_handlers {
        let i = h.0;
        h.1.join().unwrap_or_else(|_| panic!("client {} fail to execute", i));
    }

    println!("all client exist")
}
