extern crate FastMap;

use std::io;
use std::io::prelude::*;

fn main() {
    let mut hash_map = FastMap::Table::new();

    let n = io::stdin();
    for line in n.lock().lines() {
        if let Ok(line) = line {
            let mut cmd = line.split(" ");
            match cmd.next() {
                Some("LOOKUP") => {
                    if let Some(key) = cmd.next() {
                        let _ = hash_map.get(key.parse::<u64>().unwrap());
                    } else {
                        continue;
                    }
                }
                Some("INSERT") => {
                    if let Some(key) = cmd.next() {
                        if let Some(val) = cmd.next() {
                            let _ = hash_map.insert(key.parse::<u64>().unwrap(), val.parse::<u64>().unwrap());
                        }
                    } else {
                        continue;
                    }
                }
                _ => continue,
            }
        } else {
            break;
        }
    }
}
