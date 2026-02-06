pub fn get_local_ip() -> Option<String> {
    let interfaces = get_if_addrs::get_if_addrs().ok()?;
    for iface in interfaces {
        // 只返回 IPv4、非回环地址
        if !iface.is_loopback() {
            if let get_if_addrs::IfAddr::V4(ref addr) = iface.addr {
                let ip = addr.ip.to_string();
                // 过滤掉非局域网地址 (127.x.x.x)
                if !ip.starts_with("127.") {
                    return Some(ip);
                }
            }
        }
    }
    None
}
