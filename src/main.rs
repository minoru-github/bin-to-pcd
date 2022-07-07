use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    //let args: Vec<String> = env::args().collect();
    //let filename = &args[1];
    let filename = "./src/data/0000000000.bin";

    println!("file name : {}", filename);

    let mut f = File::open(filename).expect("file not found");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("read error");
    unsafe {
        let ptr = buffer.as_ptr();
        let arr = std::slice::from_raw_parts(ptr, 4);
        let data = arr as *const [u8] as *const f32;
        println!("{}", *data);
    }
    let buf : [u8;4] = [0x83,0xC0,0xB5,0x41];
    unsafe {
        let p = &buf as *const [u8] as *const f32;
        println!("{}", *p);
    }
}
