use local_ip_address::local_ip;

pub fn get_local_ip() -> Option<String> {
    // 使用 local_ip() 获取主 IP，它内部会自动过滤掉回环地址(Loopback)
    match local_ip() {
        Ok(ip) => Some(ip.to_string()),
        Err(_) => None,
    }
}