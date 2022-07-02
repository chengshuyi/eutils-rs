use super::FdType;
use anyhow::Result;

use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct PidFd {
    fdtype: FdType,
}

impl PidFd {
    pub fn from_file<P>(path: P) -> Result<PidFd>
    where
        P: AsRef<Path>,
    {
        let inf = fs::read_link(path)?;
        let mut fdtype = FdType::Unknown;

        if let Some(name) = inf.to_str() {
            // socket:[inum]
            if name.starts_with("socket") {
                fdtype = FdType::SocketFd(name[8..name.len() - 1].parse()?);
            }
        }
        Ok(PidFd { fdtype })
    }

    pub fn fdtype(&self) -> FdType {
        self.fdtype
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pidfs_from_file() {
        let pidfd = PidFd::from_file("/proc/1/fd/70");
        assert_eq!(pidfd.is_ok(), true);
    }
}
