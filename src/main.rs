use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("hello.txt")
        .expect("fail to open file");

    let mut buffer: Vec<u8> = Vec::new();//String::new();

    f.read_to_end(&mut buffer)
        .expect("fail to read");

    println!("{:?} is content", buffer);
    /*
    let words: Vec<&str> = buffer.split(" ").collect();

    println!("{:?}", words);
    let mut frequencey: HashMap<&str, u32> = HashMap::new();

    for word in words {
        frequencey.entry(word).or_insert(0) += 1;
    }
    println!("{:?}", frequencey);
    */
}
