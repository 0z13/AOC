pub fn solve() {
    let xs = include_str!("./data/day05.txt").lines();
    
    let xss = "BFFFBBFRR";
    get_num(xss);
    /*
    let max : u8= 0; 
    for i in xs{
       let x = get_num(i);
       if x > max {
         max = x;
       }
    }
    */
    
}

fn get_num(x: &str )  {
    let mut end : i32 = 127;
    let mut start : i32 = 0;


    for i in x.chars() {
        if (i == 'F') {
            start = (end / 2);
            println!("{}", start)
        } else {
            end = (end / 2);
            println!("{}", end) 
        }
    }

    println!("{}-{}", end, start);
}


