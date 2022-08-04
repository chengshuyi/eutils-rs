

mod kernel_version;
pub mod timestamp;
pub mod proc;
pub mod delay_distribution;
pub mod helpers;
pub mod net;
pub use {
    self::kernel_version::KernelVersion,
};


use anyhow::{bail, Result};

pub fn bump_memlock_rlimit() -> Result<()> {
    let rlimit = libc::rlimit {
        rlim_cur: 128 << 20,
        rlim_max: 128 << 20,
    };

    if unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlimit) } != 0 {
        bail!("Failed to increase rlimit");
    }

    Ok(())
}