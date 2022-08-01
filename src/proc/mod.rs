

mod kallsyms;
mod snmp;
mod netstat;
mod net_tcp;
mod pid_fd;
mod pid_ns;
mod pid;
pub use {
    self::kallsyms::Kallsyms,
    self::snmp::Snmp,
    self::net_tcp::NetTcp,
    self::pid_fd::PidFd,
    self::pid_ns::PidNs,
    self::pid::Pid,
};

#[derive(Debug, Clone, Copy)]
pub enum FdType {

    SocketFd(u32),
    Cgroup(u32),
    Ipc(u32),
    Mnt(u32),
    Net(u32),
    Pid(u32),
    Uts(u32),
    Unknown,
}