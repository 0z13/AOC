//j
//
use regex::Regex;

pub fn solve() {
    let xs =  include_str!("./data/day04.txt").lines();
    let mut tmp: Vec<String> = Vec::new();
    let mut data: Vec<Vec<String>> = Vec::new();
    for lines in xs {
        if lines == ""  {
            data.push(tmp.clone());
            tmp.clear();
        } else {
            tmp.push(lines.to_string());
        }
    }


    let x : Vec<&Vec<String>> = data.iter().filter(|x| pass_control(x)).collect();
    println!("{}", x.len())

}

fn pass_control(xs : &Vec<String>) -> bool {
    let re = Regex::new(r"^cid:\d+$").unwrap();
    let mut counter: i32 = 0;
    let mut flag : bool = false;
    for i in xs {
        for j in i.split(' ') {
            counter = counter + 1;
            if re.is_match(j) {
                flag = true; 
                println!("{}", j);
            }
        }
    }
    if counter == 8 || (counter == 7 && flag) {
        return true;
    }
    return false;

}


