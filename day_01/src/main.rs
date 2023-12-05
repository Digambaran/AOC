use std::collections::{hash_map, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Default,Debug)]
struct TrieNode {
    value: Option<char>,
    is_end: bool,
    children: [Option<Box<TrieNode>>;26]
}

#[derive(Debug)]
struct Trie {
    root: Box<TrieNode>
}
struct SearchReturn {
    is_word:bool,
    is_a_prefix:bool
}
impl Trie {
    fn new()->Self {
       Trie {
            root: Box::new(TrieNode::default())
        }
    }
    fn insert(&mut self, word:&str )-> &mut Self {
        let mut c_node = &mut self.root;
        for c in word.chars() {
            // if char node exists 
            // c_node = char node
            // else create new node and assign and update c_node
            let x = c as u32;
            if let Some(cx) =  &c_node.children[x.rem_euclid(97) as usize] {
                // println!("{:?}",cx);
                c_node = c_node.children[x.rem_euclid(97) as usize].as_mut().unwrap();

            }else{
                let mut b = Box::new(TrieNode::default());
                b.value = Some(c);
                b.is_end = false;
                c_node.children[x.rem_euclid(97) as usize] = Some(b);
                c_node = c_node.children[x.rem_euclid(97) as usize].as_mut().unwrap();
            }
            // println!("{:?}",c_node.children[x.rem_euclid(97) as usize]);
        }
        c_node.is_end = true;
        // println!("{:?}",c_node);
        // Trie { root: Box::new(TrieNode::default()) }
         self
    }
    fn search(&mut self, word:&str)->SearchReturn{
         let mut c_node = &mut self.root;
        for c in word.chars() {
            // if char node exists 
            // c_node = char node
            // else create new node and assign and update c_node
            let x = c as u32;
            if let Some(cx) =  &c_node.children[x.rem_euclid(97) as usize] {
                // println!("{:?}",cx);
                c_node = c_node.children[x.rem_euclid(97) as usize].as_mut().unwrap();

            }else{
                return SearchReturn{
                    is_word:false,
                    is_a_prefix:false
                }
            }
            // println!("{:?}",c_node.children[x.rem_euclid(97) as usize]);
        }
        let nbnb = c_node.children.iter().any(|e|{
            match e {
                None => false,
                Some(e) => true
            }
        });
        SearchReturn {
        is_word:c_node.is_end,
        is_a_prefix:nbnb
        }
    }
}
fn first(){

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


fn second(){


    let mut record = Trie::new();
    record
        .insert("one")
        .insert("two")
        .insert("three")
        .insert("four")
        .insert("five")
        .insert("six")
        .insert("seven")
        .insert("eight")
        .insert("nine");

    let mut lookup = hash_map::HashMap::new();
    lookup.insert("one", 1);
    lookup.insert("two", 2);
    lookup.insert("three", 3);
    lookup.insert("four", 4);
    lookup.insert("five", 5);
    lookup.insert("six", 6);
    lookup.insert("seven", 7);
    lookup.insert("eight", 8);
    lookup.insert("nine", 9);

    let mut lp:Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        let mut sum = 0;
        for line in lines {
            let mut left_number:Option<char> = None;
            let mut right_number:Option<char> = None;
            if let Ok(word) = line {
                for ch in word.chars() {
                    // if current char is a number, keep setting right_number
                    // if current char is a number and left_number is not set, set it
                    if ch.is_ascii_digit() {
                        if left_number == None {
                            left_number = Some(ch);
                        }
                       right_number = Some(ch);
                    }else {
                        // it is a char, we need to check if it is a prefix of something
                        // println!("{} not digit",ch);
                        for v in lp.iter_mut(){
                            v.push(ch);
                            // println!("searching for {}",v);
                            let res = record.search(v.as_str());
                            if !res.is_a_prefix && !res.is_word {
                               v.clear(); 
                            }else if res.is_word {
                                // if it is a word, don't clear yet, could be a prefix also
                                // println!("{} is a number word",v);
                                if left_number == None {
                                    left_number = Some(char::from_digit(lookup[v.as_str()],10).unwrap());
                                }
                                    right_number = Some(char::from_digit(lookup[v.as_str()],10).unwrap());
                            }
                        }
                       lp.retain(|v| !v.is_empty());
                        lp.push(ch.to_string());
                        
                    }
                }
                let mut ss = String::new();
                ss.push(left_number.unwrap());
                ss.push(right_number.unwrap());
                let ss_number:u32 =  ss.parse().unwrap();
                sum += ss_number;
                // println!("{}",word);
           }
        }
                println!("{}",sum);
    }
}
fn main() {
    // first();
    second();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
