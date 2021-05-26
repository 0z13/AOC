pub fn solve() {
    println!("ans 1 {}", tree_counter (1, 3));
    println!("ans 2 {}", tree_counter (1, 1)*tree_counter (1, 3)*tree_counter (1, 5)*tree_counter (1, 7)*tree_counter (2, 1));
}

fn tree_counter(down: usize, right: usize) -> i64 {
    let xs: Vec<&str> = include_str!("./data/day03.txt").lines().collect();
    let l = xs[0].len();
    let mut counter:usize = 0;
    let mut other_counter_lol = 0;
    let mut trees = 0;
    for i in xs {
        
        if other_counter_lol % down != 0 {
            other_counter_lol = other_counter_lol + 1;
            continue
        }


        let x:Vec<char> = i.chars().collect();
        if x[counter % l] == '#' {
            trees += 1;
        }

        counter = counter + right;
        other_counter_lol = other_counter_lol + 1;
    }


    return trees
}
