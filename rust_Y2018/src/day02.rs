use std::collections::HashMap;
pub fn sol() {
    let lines: Vec<&str> = include_str!("./data/day02.txt").lines().collect();

    let mut twos = 0;
    let mut threes = 0;
    
    for j in lines.iter() {
        let mut xs : HashMap<char, i32> = HashMap::new();
        for i in j.chars() {
            if xs.contains_key(&i) {
                let old_val = xs.get(&i).unwrap();
                xs.insert(i, old_val + 1); 
            } else {
                xs.insert(i, 1);
            }

        }
       let (a, b) = hash_counter(xs);
       twos += a;
       threes += b;
    }
    println!("Exercise 1 {}:", twos*threes);
    if let Some(test) = equal_but_one("hello", "hello") {
        println!("{}",test);
    }

    for i in 0 ..lines.len() {
        for j in i+1..lines.len() {
            match equal_but_one(&lines[i], &lines[j]) {
                None => (),
                Some(x) => println!("Exercise 2: {}", x)
            }
        }
    }
}

fn equal_but_one(x: &str, y: &str) -> Option<String> {

    let mut flag = false;
    for (i,j) in x.chars().zip(y.chars()) {
            if i != j {
                if flag == true {
                    return None
                }
            flag = true;
            }
         }
  return Some(
        x.chars().zip(y.chars()).filter(|&(c1, c2)| c1 == c2).map(|(c, _)| c).collect()
      )
}

fn hash_counter(xs : HashMap<char, i32>) -> (i32,i32) {
  
    let mut twos = 0;
    let mut threes = 0;
    for i in xs.values() {
        if *i == 2 { twos = 1; }
        if *i == 3 { threes = 1; }
    }
    (twos, threes)
}
