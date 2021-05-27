pub fn solve() {
    let s: Vec<i32> = include_str!("./data/day01.txt").lines().map(|x| x.parse::<i32>().unwrap()).collect();

    for i in 0..s.len() {
        for j in 1..s.len() {
        
            if (s[i] + s[j] != 2020) {
                continue;
            } else {
                println!("ans 1 {}", s[i]*s[j]);
                break;
        }
    }
  } 

    for i in 0..s.len() {
        for j in 1..s.len() {
            for z in 2..s.len() {
                if s[i] + s[j] + s[z] != 2020 {
                    continue;
                } else {
                    println!("ans 2 {}", s[i]*s[j]*s[z]);
                    break;
                }
            }
        }
    }

    println!("just use one of the answers ;-)")
}
