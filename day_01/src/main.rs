use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum:i32 = 0;
        for line in lines {
            if let Ok(ip) = line {
               let l =  ip.find(|c:char| {
                    // one way of handling Option
                    match c.to_digit(10) {
                        None => false,
                        Some(_) => true
                    }
                }).unwrap(); 
               let r = ip.rfind(|c:char| {
                    // second way of handling Option
                    if let Some(_) = c.to_digit(10) {
                        true
                    }else { false }
                }).unwrap();
                // another option to get a char as a char itself is using the nth(l) fuction on
                // iterator.. here the return type is &str.
                let ld:&str= ip.get(l..l+1).unwrap();
                let rd:&str = ip.get(r..r+1).unwrap();
                let s = ld.to_owned()+rd;
                // another way of converting is s.parse::<i32>().unwrap()
                let x:i32 = s.parse().unwrap();
                sum += x;
                println!("{ip}:{}",s);
                
            }
        }
        println!("{sum}");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
