use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

pub mod first;
pub mod second;

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>, io::Error>
where
    P: AsRef<Path>,
{
    let f = File::open(filename)?;
    Ok(BufReader::new(f).lines())
}

//
// ss is true if strings need to be concatinated and considered as one big number
fn read_and_transform(p: &str, ss: bool) -> Option<(Vec<i64>, Vec<i64>)> {
    let transform = |s: String| {
        if ss {
            let x = s.split(" ").skip(1).fold(String::new(), |mut acc, e| {
                acc.push_str(e);
                return acc;
            });
            println!("{:?}", x.clone());
            return vec![x.parse::<i64>().unwrap()];
        }
        s.split(" ")
            .filter_map(|v| v.parse::<i64>().ok())
            .collect::<Vec<i64>>()
    };
    let input_path = Path::new(p);
    let lines_buff = read_lines(input_path);
    match lines_buff {
        Ok(mut lines) => {
            // collect time array elements
            let time_string: String = lines.next().unwrap().unwrap();
            let time_array: Vec<i64> = transform(time_string);
            let distance_string: String = lines.next().unwrap().unwrap();
            let distance_array: Vec<i64> = transform(distance_string);
            let re = (time_array, distance_array);
            return Some(re);
        }
        Err(err) => {
            println!("{:?}", err);
            None
        }
    }
}
fn solve(ans: Option<(Vec<i64>, Vec<i64>)>) -> i64 {
    let (times, distance) = ans.unwrap();
    let mut final_ways: i64 = 1;
    for i in 0..times.len() {
        let mut number_of_ways = 0;
        let mut j = 0;
        loop {
            j += 1;
            if times[i] - j == 0 {
                break;
            }
            if (times[i] - j) * j > distance[i] {
                number_of_ways += 1;
            }
        }
        final_ways *= number_of_ways;
    }

    return final_ways;
}
fn main() {
    let sample_set = read_and_transform("./sample.txt", false);
    let first_set = read_and_transform("./first.txt", false);
    let second_set = read_and_transform("./second.txt", true);

    let sample_ans = solve(sample_set);
    let first_ans = solve(first_set);
    let second_ans = solve(second_set);

    println!("sample answer - {}", sample_ans);
    println!("first answer - {}", first_ans);
    println!("second answer - {}", second_ans);
}
