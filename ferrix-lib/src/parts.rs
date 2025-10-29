/* parts.rs
 *
 * Copyright 2025 Michail Krasnov <mskrasnov07@ya.ru>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! Get information about mounted partitions

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::Path;

use crate::traits::ToJson;
use crate::utils::Size;

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct StorageInfo {
//     pub device: String,
//     pub mnt_point: String,
//     pub fs: String,
//     pub total_size: Size,
//     pub used: Size,
//     pub available: Size,
//     pub device_model: Option<String>,
//     pub block_size: usize,
// }

/// List of partitions
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Partitions {
    pub parts: Vec<Partition>,
}

impl Partitions {
    pub fn new() -> Result<Self> {
        let contents = read_to_string("/proc/partitions")?;
        Self::from_str(&contents)
    }

    fn from_str(s: &str) -> Result<Self> {
        let lines = s.lines().skip(1).filter(|s| {
            !s.is_empty() && !s.starts_with('m') && !s.contains("loop") && !s.contains("ram")
        });

        let mut parts = Vec::new();
        for line in lines {
            match Partition::try_from(line) {
                Ok(part) => parts.push(part),
                Err(why) => return Err(anyhow!("{why}")),
            }
        }

        Ok(Self { parts })
    }
}

impl ToJson for Partitions {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Partition {
    pub major: usize,
    pub minor: usize,
    pub blocks: usize,
    pub name: String,
    pub dev_info: DeviceInfo,
}

impl Partition {
    pub fn get_logical_size(&self) -> Option<Size> {
        let lbsize = self.dev_info.logical_block_size;
        match lbsize {
            Some(lbsize) => {
                let blocks = self.blocks;
                Some(Size::B(blocks * lbsize))
            }
            None => None,
        }
    }
}

impl TryFrom<&str> for Partition {
    type Error = String;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let mut chs = value.split_whitespace();

        match (chs.next(), chs.next(), chs.next(), chs.next()) {
            (Some(major), Some(minor), Some(blocks), Some(name)) => {
                let major = major.parse::<usize>().map_err(|err| format!("{err}"))?;
                let minor = minor.parse::<usize>().map_err(|err| format!("{err}"))?;
                let blocks = blocks.parse::<usize>().map_err(|err| format!("{err}"))?;

                Ok(Self {
                    major,
                    minor,
                    blocks,
                    name: name.to_string(),
                    dev_info: DeviceInfo::get(name),
                })
            }
            _ => Err(format!("String '{value}' parsing error")),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeviceInfo {
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub serial: Option<String>,
    pub logical_block_size: Option<usize>,
}

impl DeviceInfo {
    pub fn get(devname: &str) -> Self {
        let path = Path::new("/sys/block/").join(devname);
        let device = path.join("device");
        let queue = path.join("queue");

        let model = device.join("model");
        let vendor = device.join("vendor");
        let serial = device.join("serial");

        let logical_block_size = queue.join("logical_block_size");
        let logical_block_size = match read_to_string(logical_block_size) {
            Ok(lbs) => lbs.trim().parse::<usize>().ok(),
            Err(_) => None,
        };

        Self {
            model: read_to_string(model).ok(),
            vendor: read_to_string(vendor).ok(),
            serial: read_to_string(serial).ok(),
            logical_block_size,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileSystemStats {
    pub total: usize,
    pub used: usize,
    pub free: usize,
    pub usage_percentage: f32,
}

impl FileSystemStats {}

#[cfg(test)]
mod tests {
    use super::*;

    const PARTITIONS: &str = "major minor  #blocks  name

 259        0  250059096 nvme0n1
 259        1     102400 nvme0n1p1
 259        2      16384 nvme0n1p2
 259        3  249068548 nvme0n1p3
 259        4     866304 nvme0n1p4
   8        0  468851544 sda
   8        1     614400 sda1
   8        2   73138176 sda2
   8        3  337163264 sda3
   8        4   57933824 sda4
 253        0    3976960 zram0";

    #[test]
    fn partitions_from_str_test() {
        let parts = Partitions::from_str(PARTITIONS).unwrap();
        dbg!(&parts);
        assert_eq!(parts.parts.len(), 10);
        assert_eq!(&parts.parts[0].name, "nvme0n1");
        assert_eq!(parts.parts[0].major, 259);
        assert_eq!(parts.parts[0].minor, 0);
        assert_eq!(parts.parts[0].blocks, 250059096);
    }

    #[test]
    fn partition_invalid_str_test() {
        let s = "256 0 nvme";
        let part = Partition::try_from(s);
        assert!(part.is_err());
    }

    #[test]
    fn partition_valid_str_test() {
        let s = "255 4 666 sda";
        let part = Partition::try_from(s);
        assert!(part.is_ok());
    }
}
