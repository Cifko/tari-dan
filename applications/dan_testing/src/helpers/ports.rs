use std::net::TcpListener;

pub fn get_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to address");
    let port = listener.local_addr().expect("There is no free port").port();
    return port;
}
