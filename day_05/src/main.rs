
#![feature(coroutines, coroutine_trait)]
use std::collections::HashMap;
use::std::fs::File;
use::std::io::{BufRead,BufReader,Lines,Result};
use std::ops::Index;
use::std::path::Path;
use::std::pin::Pin;
use::std::cmp::Ordering::{Less,Greater,Equal};
use::std::ops::{CoroutineState,Coroutine};
use std::u128;

fn create_gen(constraints:&mut Vec<(u128,u128,u128)>,limit:u128)-> (u128,u128){
   // sort contraints based on second element
    constraints.sort_by(|a,b| {
        if a.1 < b.1 { return Less};
        if a.1 > b.1 { return Greater};
        Equal

    }) ; 

    let mut r = (0,0);
    let mut ns = 0;
    let mut count_ns = 0;
    let mut curr = 0;
    for (index,v) in constraints.iter().enumerate() {
        if  v.1 < limit  {
            r = (v.1,v.0);
            curr = index;
        }
    }


    println!("limit {}",limit);
    println!("Starting with {} & {:?}",curr,r);
    let mn = constraints.get(curr).unwrap();
    println!("mn {:?}",mn);
    if limit > mn.1 {
        let diff:u128 = limit - mn.1;
        if diff < mn.2 {
            r = (mn.1+diff,mn.0+diff); 
        }else{
            r=(limit,limit);
        }
    }
    else {
        r = (limit,limit);
    }

    println!("after opt {},{:?}",curr,r);
    // let mut generator= move |limit| {
    //     loop {
    //         let temp_tuple = constraints.get(curr).unwrap_or_else(|| constraints.get(curr-1).unwrap());
    //         // println!("{:?}",temp_tuple);
    //         if r.0 == temp_tuple.1  {
    //             r.0 = temp_tuple.1;
    //             r.1 = temp_tuple.0;
    //             ns = temp_tuple.2;
    //             count_ns = 0;
    //             curr += 1;
    //         }
    //         // println!("{:?}{:?}",ns,count_ns);
    //         if count_ns == ns {
    //             r.1 = r.0;
    //             count_ns = 0;
    //         }
    //         yield r;
    //         if r.0 == limit { break; }
    //         count_ns += 1;
    //         r.0 += 1;
    //         r.1 += 1;
    //     
    //     }
    //
    //     r
    // };
    
    // let mut res = (0,0);
    // loop{
    //     match Pin::new(&mut generator).resume(limit) {
    //         CoroutineState::Yielded(a) => {
    //             res = a;
    //         },
    //         CoroutineState::Complete(a) =>{
    //             res = a;
    //             break;
    //         }
    //     }
    // }
    //
    // return res;
    r
    
}
fn first(){
    let mut seeds:Vec<u128> = Vec::new();
    let mut seed_to_soil:Vec<(u128,u128,u128)> = Vec::new();
    let mut soil_to_fertilizer:Vec<(u128,u128,u128)> = Vec::new();
    let mut fertilizer_to_water:Vec<(u128,u128,u128)> = Vec::new();
    let mut water_to_light:Vec<(u128,u128,u128)> = Vec::new();
    let mut light_to_temperature:Vec<(u128,u128,u128)> = Vec::new();
    let mut temperature_to_humidity:Vec<(u128,u128,u128)> = Vec::new();
    let mut humidity_to_location:Vec<(u128,u128,u128)> = Vec::new();
    
    let mut dict = HashMap::new();
    let kkeys = ["seed-to-soil","soil-to-fertilizer","fertilizer-to-water","water-to-light","light-to-temperature","temperature-to-humidity","humidity-to-location"];

    dict.insert("seed-to-soil", seed_to_soil);
    dict.insert("soil-to-fertilizer", soil_to_fertilizer);
    dict.insert("fertilizer-to-water", fertilizer_to_water);
    dict.insert("water-to-light", water_to_light);
    dict.insert("light-to-temperature", light_to_temperature);
    dict.insert("temperature-to-humidity", temperature_to_humidity);
    dict.insert("humidity-to-location", humidity_to_location);

    if let Ok(lines) = read_lines("./input.txt"){
        let mut curr_vec = String::from("seed-to-soil");
        for line in lines {
            if let Ok(l) = line {
            if l.len()==0 {
                curr_vec = "".to_string();    
                println!("curr_vec cleared");
            }
            else if l.len()>0 && l.contains("map:") {
                curr_vec = l.split(' ').collect::<Vec<&str>>()[0].to_owned();
                println!("curr_vec updated to {}",curr_vec);
            }
            else if l.len()>0 && l.contains("seeds:") {
                println!("{}",l);
                seeds = l.split(' ').skip(1).map(|v| {
                        println!("{}",v);
                        v.parse::<u128>().unwrap()
                    }).collect();
            }
            else {
               let t_vec = l.split(' ').collect::<Vec<&str>>();
                let x = dict.get_mut(curr_vec.as_str()).unwrap();
               x.push((t_vec[0].parse::<u128>().unwrap(),t_vec[1].parse::<u128>().unwrap(),t_vec[2].parse::<u128>().unwrap()));
            }

        }
            
        }

        // println!("{:?}",dict);
        let mut ans:Vec<u128> = Vec::new();
        for i in seeds {
            let mut x = i;
            for k in kkeys {
                let vv = dict.get_mut(k).unwrap();
                let a = create_gen(vv,x).to_owned();
                // println!("dict {:?}",vv);
                // println!("seeds {},mapped {}, key {}, a {:?}",i,x,k,a);
                x = a.1;
                // println!("{:?}",a);
            }
            println!("seed {} maps to {}",i,x);
            ans.push(x);
        }

        println!("ans is {}",ans.iter().min().unwrap());
    }
}
fn main() {
    first(); 
    let mut r = (0,1);
    let mut cons = vec![(50,98,2),(52,50,48)];
    // cons.sort_by(|a,b| {
    //     if a.1 < b.1 { return Less};
    //     if a.1 > b.1 { return Greater};
    //     Equal
    //
    // }) ; 
    // println!("{:?}",cons);
    let mut ns = 0;
    let mut count_ns = 0;
    let mut curr = 0;
    // let mut generator= move |ar| {
    //     loop {
    //         let temp_tuple = cons.get(curr).unwrap_or_else(|| cons.get(curr-1).unwrap());
    //         // println!("{:?}",temp_tuple);
    //         if r.0 == temp_tuple.1  {
    //             r.0 = temp_tuple.1;
    //             r.1 = temp_tuple.0;
    //             ns = temp_tuple.2;
    //             count_ns = 0;
    //             curr += 1;
    //         }
    //         // println!("{:?}{:?}",ns,count_ns);
    //         if count_ns == ns {
    //             r.1 = r.0;
    //             count_ns = 0;
    //         }
    //         yield r;
    //         count_ns += 1;
    //         r.0 += 1;
    //         r.1 += 1;
    //         if r.1 > ar { break; }
    //     
    //     }
    //
    //     r
    // };
    // let seed_to_soil = create_gen(&mut cons,150);
    // loop {
    //
    //     let mut ret = (0,1);
    //     match Pin::new(&mut ssgenerator).resume(150) {
    //         CoroutineState::Yielded(a) => {
    //             ret = a;
    //         },
    //         CoroutineState::Complete(a) =>{
    //             ret = a;
    //             break;
    //         }
    //     }
    //     println!("{:?}",ret);
    // }
    // println!("{:?}",seed_to_soil)

}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
