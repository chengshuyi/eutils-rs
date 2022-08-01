use anyhow::{bail, Result};
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::ops::Sub;
use std::path::Path;
use std::str::FromStr;

#[derive(Default, Debug, Clone)]
pub struct Snmp {
    hm: HashMap<(String, String), isize>,
}

impl FromStr for Snmp {
    type Err = anyhow::Error;
    fn from_str(content: &str) -> Result<Self> {
        let mut snmp = Snmp::default();

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

                snmp.insert((prefix.clone(), k.to_string()), v);
            }
        }

        Ok(snmp)
    }
}

impl Sub for Snmp {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut snmp = Snmp::default();

        for (k, v) in self.hm.iter() {
            assert_eq!(other.hm.contains_key(k), true);
            snmp.insert(k.clone(), v - other.hm[k]);
        }

        snmp
    }
}

impl Snmp {
    pub fn from_file<P>(path: P) -> Result<Snmp>
    where
        P: AsRef<Path>,
    {
        let string = read_to_string(path)?;
        Snmp::from_str(&string)
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
    fn test_snmp_from_file() {
        let snmp = Snmp::from_file("/proc/net/snmp");
        assert_eq!(snmp.is_ok(), true);
    }

    #[test]
    fn test_snmp_ops_sub() {
        let snmp1 = Snmp::from_file("/proc/net/snmp").unwrap();
        let snmp2 = Snmp::from_file("/proc/net/snmp").unwrap();
        let delta = snmp1 - snmp2;
    }

    #[test]
    fn test_snmp_show_non_zero() {
        let snmp1 = Snmp::from_file("/proc/net/snmp").unwrap();
        let snmp2 = Snmp::from_file("/proc/net/snmp").unwrap();
        let delta = snmp1 - snmp2;
        delta.show_non_zero();
    }
}
