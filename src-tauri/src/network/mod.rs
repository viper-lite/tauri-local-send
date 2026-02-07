mod ip;

pub use ip::get_local_ip;

pub fn get_free_port() -> u16 {
    use std::net::TcpListener;
    let listener = TcpListener::bind("0.0.0.0:0").ok().unwrap();
    listener.local_addr().ok().unwrap().port()
}
