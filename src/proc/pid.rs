use super::{PidFd, PidNs};
use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Pid {
    // parse from /proc/<PID>/fd
    pub fds: Vec<PidFd>,

    // parse from /proc/<PID>/ns
    pub nss: Vec<PidNs>,
}

impl Pid {
    pub fn from_file<P>(path: P) -> Pid
    where
        P: AsRef<Path>,
    {
        let mut pb = PathBuf::new();
        let mut fds = Vec::default();
        pb.push(path);

        // /proc/<PID>/fd
        pb.push("fd");
        match fs::read_dir(&pb) {
            Ok(entrys) => {
                for entry in entrys {
                    match entry {
                        Ok(ent) => match PidFd::from_file(ent.path()) {
                            Ok(fd) => {
                                log::debug!("{:?}: {:?}", ent.path(), fd);
                                fds.push(fd);
                            }
                            Err(e) => {
                                println!("failed to parse file: {:?}, error: {}", ent.path(), e);
                            }
                        },
                        Err(e) => {
                            println!("failed to get entry: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("failed to readdir: directory-{:?}, error-{}", pb, e);
            }
        }

        pb.pop();

        // /proc/<PID>/ns
        pb.push("ns");
        for entry in fs::read_dir(&pb) {}
        pb.pop();

        Pid {
            fds,
            nss: Vec::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pid_from_file() {
        let pid = Pid::from_file("/proc/1/");
        // assert_eq!(pid.is_ok(), true);
    }
}
