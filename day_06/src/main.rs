use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, self},
    path::Path,
};

use crate::{first::first_answer, second::second_answer};

pub mod first;
pub mod second;
fn main() {
    let input_path = Path::new("./input.txt");
    // println!("{}",input_path.to_str().unwrap());
    if let Ok(mut lines) =  read_lines(input_path) {
        while let Some(line)  = lines.next()  {
           println!("{}",line.unwrap()); 
        }
    }else {
        println!("Error reading from file");
        panic!("Failed");
    }
    let first_answer: u128 = first_answer();
    let second_answer: u128 = second_answer();
    println!("{}{}",first_answer,second_answer);
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>, io::Error>
where
    P: AsRef<Path>,
{
    let f = File::open(filename)?;
    Ok(BufReader::new(f).lines())
}
