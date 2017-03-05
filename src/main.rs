use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut f = File::open("sample/hello.txt")
        .expect("fail to open file");

    let mut buffer: Vec<u8> = Vec::new();//String::new();

    f.read_to_end(&mut buffer)
        .expect("fail to read");

    println!("{:?} is content", buffer);

    let mut frequencey = HashMap::new();

    for word in buffer {
        let counter = frequencey.entry(word).or_insert(0);
        *counter += 1;
    }
    println!("{:?}", frequencey);
}
