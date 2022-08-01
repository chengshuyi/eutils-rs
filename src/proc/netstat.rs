use anyhow::{bail, Result};
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::ops::Sub;
use std::path::Path;
use std::str::FromStr;

#[derive(Default, Debug, Clone)]
pub struct Netstat {
    hm: HashMap<(String, String), isize>,
}

impl FromStr for Netstat {
    type Err = anyhow::Error;
    fn from_str(content: &str) -> Result<Self> {
        let mut netstat = Netstat::default();

        let lines = content.split('\n').collect::<Vec<&str>>();

        for i in 0..lines.len() / 2 {
            let line1 = lines[i * 2];
            let line2 = lines[i * 2 + 1];

            let mut iter1 = line1.split_whitespace();
            let mut iter2 = line2.split_whitespace();

            let prefix;
            if let Some(x) = iter1.next() {
                prefix = x.to_string();
            } else {
                bail!("failed to parse: prefix not found")
            }
            iter2.next();
            loop {
                let k;
                let v: isize;
                if let Some(x) = iter1.next() {
                    k = x;
                } else {
                    break;
                }

                if let Some(x) = iter2.next() {
                    v = x.parse()?;
                } else {
                    bail!("failed to parse: number of item is not match.")
                }

                netstat.insert((prefix.clone(), k.to_string()), v);
            }
        }

        Ok(netstat)
    }
}

impl Sub for Netstat {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut netstat = Netstat::default();

        for (k, v) in self.hm.iter() {
            assert_eq!(other.hm.contains_key(k), true);
            netstat.insert(k.clone(), v - other.hm[k]);
        }

        netstat
    }
}

impl Netstat {
    pub fn from_file<P>(path: P) -> Result<Netstat>
    where
        P: AsRef<Path>,
    {
        let string = read_to_string(path)?;
        Netstat::from_str(&string)
    }

    pub fn insert(&mut self, k: (String, String), v: isize) {
        self.hm.insert(k, v);
    }

    pub fn lookup(&self, k: &(String, String)) -> Option<&isize> {
        self.hm.get(k)
    }

    pub fn show_non_zero(&self) {
        for (k, v) in &self.hm {
            if *v != 0 {
                print!("{}{}: {} ", k.0, k.1, v);
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_netstat_from_file() {
        let netstat = Netstat::from_file("/proc/net/netstat");
        assert_eq!(netstat.is_ok(), true);
    }

    #[test]
    fn test_netstat_ops_sub() {
        let netstat1 = Netstat::from_file("/proc/net/netstat").unwrap();
        let netstat2 = Netstat::from_file("/proc/net/netstat").unwrap();
        let delta = netstat1 - netstat2;
    }

    #[test]
    fn test_netstat_show_non_zero() {
        let netstat1 = Netstat::from_file("/proc/net/netstat").unwrap();
        let netstat2 = Netstat::from_file("/proc/net/netstat").unwrap();
        let delta = netstat2 - netstat1;
        delta.show_non_zero();
    }
}