use std::net::SocketAddr;




// /proc/net/tcp
pub struct NetTcp {
    local_address: SocketAddr,
    rem_address: SocketAddr,
    st: u16,
    tx_queue: u32,
    rx_queue: u32,
    tr: u32,
    tm_when: u32,
    retrnsmt: u32,
    uid: u32,
    timeout: u32,
    inode: u32,
    reference: u32,
    sk_addr: u64,
    // ......
}