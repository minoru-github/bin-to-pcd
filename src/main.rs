use std::fs::File;
use std::io::{prelude::*, Write};
use std::{env, mem};

fn main() {
    //let args: Vec<String> = env::args().collect();
    //let filename = &args[1];
    let filename = "./src/data/0000000000.bin";
    let mut buffer = Vec::new();
    read_binary_file(filename, &mut buffer);

    let mut pcd_lines = Vec::<Vec<f32>>::new();
    to_pcd(&buffer, &mut pcd_lines);

    let output_filename = "./src/data/0000000000.pcd";
    output_pcd_file(output_filename, &pcd_lines);
}

const PCD_ELEMS: usize = 4; // [x, y, z, i]

fn read_binary_file(filename: &str, mut buffer: &mut Vec<u8>) {
    let mut f = File::open(filename).expect("file not found");
    f.read_to_end(&mut buffer).expect("read error");
}

fn to_pcd(input_buffer: &Vec<u8>, output_pcd_lines: &mut Vec<Vec<f32>>) {
    let bytes_of_f32 = mem::size_of::<f32>();
    let bytes_per_line = bytes_of_f32 * PCD_ELEMS;
    let lines = input_buffer.len() / bytes_per_line;

    let ptr = input_buffer.as_ptr();
    let mut ptr_offset = 0;
    for _ in 0..lines {
        // xyzi
        let mut pcd = Vec::new();
        for _ in 0..PCD_ELEMS {
            unsafe {
                let array_u8 = std::slice::from_raw_parts(ptr.add(ptr_offset), bytes_of_f32);
                let data = (array_u8 as *const [u8]) as *const f32;
                pcd.push(*data);
                ptr_offset += bytes_of_f32;
            }
        }
        output_pcd_lines.push(pcd);
    }
}

fn output_pcd_file(output_filename: &str, pcd_lines: &Vec<Vec<f32>>) {
    let mut file = File::create(output_filename).expect("new file not created");
    for pcd in pcd_lines {
        let mut line_str = "".to_string();
        for (count, &elem) in pcd.iter().enumerate() {
            line_str.push_str(elem.to_string().as_str());
            if count != PCD_ELEMS - 1 {
                line_str.push(' ');
            } else {
                line_str.push('\n');
            }
        }
        file.write_all(line_str.as_bytes())
            .expect("pcd data not write");
    }
}

#[allow(dead_code)]
fn debug_pcd(pcd_lines: &Vec<Vec<f32>>) {
    let mut cnt = 0;
    for pcd in pcd_lines {
        println!("{:?}", pcd);
        cnt += 1;
        if cnt >= 10 {
            break;
        }
    }
}
