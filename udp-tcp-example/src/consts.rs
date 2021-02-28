use std::time::Duration;

pub const SERVER_ADDR: &'static str = "127.0.0.1:34567";
pub const BUFFER_SIZE: usize = 1024;
pub const CLIENT_MESSAGE: &'static str = "hello from client";
pub const NUM_CLIENT: usize = 10;
pub const TCP_SERVER_ADDR: &'static str = "127.0.0.1:45678";
pub const TCP_CLIENT_MESSAGE: &'static str = "hello from tcp client";
pub const TCP_CONN_TIMEOUT_SEC: Duration = Duration::from_secs(10);
