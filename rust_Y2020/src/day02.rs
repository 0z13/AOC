pub fn solve() -> anyhow::Result<()> {
   let g = include_str!("./data/day02.txt")
       .lines()
       .map(parse_line)
       .map(Result::unwrap)
       .filter(|x| is_valid_pw2(x))
       .count();

   println!("fst ans: {}", g);
   Ok(())

}

// XD gotta learn nom or something
fn parse_line(s: &str) -> anyhow::Result<PasswordAndPolicy> {
    let mut s = s.split(" ");
    let fst = s.next();
    let mut range = fst.unwrap().split("-");
    let lo = range.next().unwrap().parse()?;
    let hi = range.next().unwrap().parse()?;
    let mut c = s.next().unwrap().to_string();
    let ch = c.pop();
    let pw = s.next().unwrap();
    Ok(PasswordAndPolicy{hi:hi, lo:lo, password:pw.to_string(), character: c })
}

#[derive(Debug)]
struct PasswordAndPolicy {
    password: String,
    character: String,
    hi: i32,
    lo: i32,
}

fn is_valid_pw(x: &PasswordAndPolicy) -> bool{
        let h = x.password.chars().filter(|g| g.to_string() == x.character).count();
        (x.lo..=x.hi).contains(&(h as i32))
}

fn is_valid_pw2(x: &PasswordAndPolicy) -> bool {
    let xs: Vec<char> = x.password.chars().collect();
     if (xs[(x.hi -1) as usize].to_string() == x.character && xs[(x.lo - 1) as usize].to_string() != x.character) ||
     (xs[(x.hi -1) as usize].to_string() != x.character && xs[(x.lo -1) as usize].to_string() == x.character) {
        println!("password: {} char: {} hi: {} lo: {}", x.password, x.character,x.hi,x.lo);
        return true;
     } else {
        return false;
     }
}



