use std::io::prelude::*;
use std::io::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn dmain() {
    let mut vec = Vec::new();
    let mut tmp = Vec::new();
    if let Ok(lines) = read_lines("../../../data_2020/day04.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip != "" {
                    println!("Pushed {}", ip);
                    tmp.push(ip)
                }
                else {
                    vec.push(tmp);
                    tmp = Vec::new();

            }
        }
        
     }
   }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

