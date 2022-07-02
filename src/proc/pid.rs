use super::{PidFd, PidNs};
use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Pid {
    // parse from /proc/<PID>/fd
    fds: Vec<PidFd>,

    // parse from /proc/<PID>/ns
    nss: Vec<PidNs>,
}

impl Pid {
    pub fn from_file<P>(path: P) -> Result<Pid>
    where
        P: AsRef<Path>,
    {
        let mut pb = PathBuf::new();
        let mut fds = Vec::default();
        pb.push(path);

        // /proc/<PID>/fd
        pb.push("fd");
        for entry in fs::read_dir(&pb)? {
            let entry = entry?;
            let fd = PidFd::from_file(entry.path())?;
            log::debug!("{:?}: {:?}", entry.path(), fd);
            fds.push(fd);
        }
        pb.pop();

        // /proc/<PID>/ns
        pb.push("ns");
        for entry in fs::read_dir(&pb) {}
        pb.pop();

        Ok(Pid {
            fds,
            nss: Vec::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pid_from_file() {
        let pid = Pid::from_file("/proc/1/");
        assert_eq!(pid.is_ok(), true);
    }
}

