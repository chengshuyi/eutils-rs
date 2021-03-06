use anyhow::{bail, Result};

fn get_current_kernel_version() -> Result<String> {
    let mut info = unsafe { std::mem::MaybeUninit::<libc::utsname>::zeroed().assume_init() };
    let mut release_version = Vec::with_capacity(info.release.len());
    let ret = unsafe { libc::uname(&mut info as *mut libc::utsname) };
    if ret < 0 {
        bail!("failed to call function: libc::uname, error code: {}", ret)
    }

    for i in info.release {
        release_version.push(i as u8);
    }

    Ok(String::from_utf8(release_version)?)
}

// see: https://doc.rust-lang.org/book/appendix-03-derivable-traits.html?highlight=ord#partialord-and-ord-for-ordering-comparisons
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct KernelVersion {
    major_revision: i32,
    minor_revision: i32,
    patch_number: i32,
}

impl TryFrom<&str> for KernelVersion {
    type Error = anyhow::Error;
    fn try_from(version_string: &str) -> Result<Self> {
        let mut start_pos = 0;
        let mut major_revision = 0;
        let mut minor_revision = 0;
        let mut patch_number = 0;

        if let Some(pos) = version_string[start_pos..version_string.len()].find('.') {
            let part = &version_string[start_pos..pos];
            major_revision = part.parse()?;

            start_pos = pos + 1;
        } else {
            bail!("failed to parse {}", version_string)
        }

        if let Some(pos) = version_string[start_pos..version_string.len()].find('.') {
            let part = &version_string[start_pos..start_pos + pos];
            minor_revision = part.parse()?;

            start_pos = start_pos + pos + 1;
        } else {
            bail!("failed to parse {}", version_string)
        }

        for i in start_pos..version_string.len() + 1 {
            let mut char = '-' as u8;

            if i < version_string.len() {
               char =  version_string.as_bytes()[i];
            }
            if char < '0' as u8 || char > '9' as u8 {
                let part = &version_string[start_pos..i];
                patch_number = part.parse()?;
                break;
            }
        }

        Ok(KernelVersion {
            major_revision,
            minor_revision,
            patch_number,
        })
    }
}

impl KernelVersion {
    pub fn current() -> Result<KernelVersion> {
        let str = get_current_kernel_version()?;
        KernelVersion::try_from(str.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_current_kernel_version() {
        assert_eq!(get_current_kernel_version().is_ok(), true);
    }

    #[test]
    fn test_kernel_version_try_from1() {
        let kv = KernelVersion::try_from("3.10.10").unwrap();
        assert_eq!(kv.major_revision, 3);
        assert_eq!(kv.minor_revision, 10);
        assert_eq!(kv.patch_number, 10);
    }

    #[test]
    fn test_kernel_version_try_from2() {
        let kv = KernelVersion::try_from("3.10.10-xx").unwrap();
        assert_eq!(kv.major_revision, 3);
        assert_eq!(kv.minor_revision, 10);
        assert_eq!(kv.patch_number, 10);
    }

    #[test]
    fn test_kernel_version_current() {
        assert_eq!(
            KernelVersion::current().is_ok(),
            true,
            "{:?}",
            KernelVersion::current()
        );
    }

    #[test]
    fn test_kernel_version_ord_eq() {
        let v1 = KernelVersion::try_from("3.10.10").unwrap();
        let v2 = KernelVersion::try_from("3.10.10").unwrap();
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_kernel_version_ord_lt() {
        let v1 = KernelVersion::try_from("2.10.10").unwrap();
        let v2 = KernelVersion::try_from("3.10.10").unwrap();
        assert_eq!(v1 < v2, true);
    }
}
