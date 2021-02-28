use std::net::{self, TcpStream};
use std::io::{Write, BufReader, BufRead};
use udp_tcp_example::consts;
use std::env;
use std::thread;

fn connect_to_server(id: usize) {
    let server_socket_addr = &consts::TCP_SERVER_ADDR.
        parse::<net::SocketAddr>().
        unwrap_or_else(|_| 
            panic!("fail to convert string to the socket address {}", 
                consts::TCP_SERVER_ADDR));

    // write message to server by line
    let mut stream = TcpStream::connect_timeout(
        server_socket_addr, 
        consts::TCP_CONN_TIMEOUT_SEC).
        expect("fail to connect to the server");
    println!("connect to server from {}", stream.local_addr().unwrap());
    write!(stream, "{} {}\n", consts::TCP_CLIENT_MESSAGE, id).unwrap();
    
    // wait for reply from server
    let mut buf = String::new();
    let mut line_reader = BufReader::new(stream.try_clone().unwrap());
    line_reader.read_line(&mut buf).
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
