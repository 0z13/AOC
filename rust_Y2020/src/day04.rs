use nom::{IResult, bytes::complete::tag};

struct Id {
    eyr: i32,
    ecl: String,
    hgt: String,
    pid: i64, 
    cid: i32,
    hcl: String,
    iyr: String,
}

// Combinators

fn p_eyr(s: &str) -> IResult<&str, &str> {
    tag("eyr: ")(s)
}


impl Id {
    parse(xs: String) -> Self {
        
    }
}

pub fn solve() {
    let xs =  include_str!("./data/day04.txt").lines();
    let mut tmp: Vec<String> = Vec::new();
    let mut buffer: Vec<Id> = Vec::new();

    for i in xs {
        if i == "" {
            let mut res = String::new();
            for i in &tmp {
                res.push_str(i.as_str());
            }
            
            println!("{}", res);
            println!("");
            tmp.clear();
        
        } else {
            tmp.push(i.to_string());
    }
}}