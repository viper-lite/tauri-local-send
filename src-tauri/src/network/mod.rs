pub fn get_local_ip() -> Option<String> {
    use std::net::TcpListener;
    let addr = TcpListener::bind("0.0.0.0:0").ok()?.local_addr().ok()?;
    if addr.ip().is_loopback() {
        None
    } else {
        Some(addr.ip().to_string())
    }
}

pub fn get_free_port() -> u16 {
    use std::net::TcpListener;
    let listener = TcpListener::bind("0.0.0.0:0").ok().unwrap();
    listener.local_addr().ok().unwrap().port()
}
