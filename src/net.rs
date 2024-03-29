use strum_macros::Display;

#[derive(Debug, Display)]
pub enum ProtocolType {
    Icmp,
    Tcp,
    Udp,
    Unknown(i32),
}

impl From<i32> for ProtocolType {
    fn from(value: i32) -> Self {
        match value {
            libc::IPPROTO_ICMP => ProtocolType::Icmp,
            libc::IPPROTO_TCP => ProtocolType::Tcp,
            libc::IPPROTO_UDP => ProtocolType::Udp,
            _ => ProtocolType::Unknown(value),
        }
    }
}

#[derive(Debug, Display)]
pub enum TcpState {
    Established,
    SynSent,
    SynRecv,
    FinWait1,
    FinWait2,
    TimeWait,
    Close,
    CloseWait,
    LastAck,
    Listen,
    Closing,
    Unknown,
}

impl From<i32> for TcpState {
    fn from(value: i32) -> Self {
        match value {
            1 => TcpState::Established,
            2 => TcpState::SynSent,
            3 => TcpState::SynRecv,
            4 => TcpState::FinWait1,
            5 => TcpState::FinWait2,
            6 => TcpState::TimeWait,
            7 => TcpState::Close,
            8 => TcpState::CloseWait,
            9 => TcpState::LastAck,
            10 => TcpState::Listen,
            11 => TcpState::Closing,
            _ => TcpState::Unknown,
        }
    }
}
