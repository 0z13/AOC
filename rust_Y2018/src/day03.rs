extern crate lazy_static;
use lazy_static::lazy_static;
use std::error::Error;
use regex::Regex;
use std::str::FromStr;
use std::fmt;
pub fn sol() {
    let s = include_str!("./data/day03.txt").lines();
    let mut things: Vec<&str> = Vec::new();
    
        // approach
    // get all coords in a list
    // get all repetition with map
    // length of hashmap is square metres 
    #[derive(Debug)]
    struct Square {
        x: i32,
        y: i32,
        h: i32,
        l: i32
    }
    
    impl FromStr for Square {
        type Err = Box<dyn Error>;
        fn from_str(s: &str) -> Result<Square, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?x)
                \#
                (?P<x>[0-9]+),(?P<y>[0-9]+):
                \s+
                (?P<width>[0-9]+)x(?P<height>[0-9]+)
            ").unwrap();
        }



        let cap = match RE.captures(s) {
            None => return Err(Box::new("regex match".into())),
            Some(s) => s,
        };

        Ok(Square {
            x: cap["x"].parse()?,
            y: cap["y"].parse()?,
            h: cap["height"].parse()?,
            l: cap["width"].parse()?,
        })
      }
    }


    // parse squares
    for i in s {
      things.push(&i[8..])
    }

    for i in things {
        //println!("{:?}", (i));
        println!("{:?}", Square::from_str(i));
    }

}
