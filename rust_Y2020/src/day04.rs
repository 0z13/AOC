//
//
pub fn solve() {
    let xs =  include_str!("./data/day04.txt").lines();
    let mut tmp: Vec<String> = Vec::new();
    let mut data: Vec<String> = Vec::new();
    for lines in xs {
        if lines == "" {
            println!("nweline")
        } else {
            tmp.push(lines.to_string());
            println!("nothing here bubba");
        }
        
    }

}



