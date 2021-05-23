use std::collections::HashSet;

pub fn sol() {
    let mut s:Vec<&str> = include_str!("./data/day01.txt").split("\n").collect();
    let _g = s.pop(); // remove trailing whitespace
    let mut r = 0;
    for i in &s {
        let zz = i.parse::<i32>().unwrap();
        r += zz;
    }
    println!("exercise one: {}", r);

    // part2  
   let mut sum = 0;
   let mut bag:HashSet<i32> = HashSet::new();
   bag.insert(0);
   loop {
    for i in &s {
       let zz = i.parse::<i32>().unwrap();
       sum += zz;
       if bag.contains(&sum) {
         println!("exercise two: {}", sum);
         return ();
       } else {
        bag.insert(sum);
      }
    }
   }
}

