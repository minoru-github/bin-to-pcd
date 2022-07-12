use std::fs::File;
use std::io::{prelude::*, Write};
use std::{env, mem};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let offset;
    if args.len() >= 3 {
        offset = Offset::new(&args[2], &args[3], &args[4]);
    } else {
        let init_value = &"0.0".to_string();
        offset = Offset::new(init_value, init_value, init_value);
    }

    let mut buffer = Vec::new();
    println!("{}", path);
    read_binary_file(&path, &mut buffer);

    let mut pcd_lines = Vec::<Vec<f32>>::new();
    to_pcd(&buffer, &offset, &mut pcd_lines);

    let mut output_path = path[0..path.len() - 4].to_string();
    output_path.push_str(".pcd");
    println!("out {}", output_path);

    output_pcd_file(&output_path, &pcd_lines);
}

const PCD_ELEMS: usize = 4; // [x, y, z, i]

fn read_binary_file(path: &str, mut buffer: &mut Vec<u8>) {
    let mut f = File::open(path).expect("file not found");
    f.read_to_end(&mut buffer).expect("read error");
}

struct Offset {
    x_m: f32,
    y_m: f32,
    z_m: f32,
}

impl Offset {
    pub fn new(x_m: &String, y_m: &String, z_m: &String) -> Self {
        Offset {
            x_m: x_m.parse::<f32>().unwrap(),
            y_m: y_m.parse::<f32>().unwrap(),
            z_m: z_m.parse::<f32>().unwrap(),
        }
    }
}

fn to_pcd(input_buffer: &Vec<u8>, offset: &Offset, output_pcd_lines: &mut Vec<Vec<f32>>) {
    let bytes_of_f32 = mem::size_of::<f32>();
    let bytes_per_line = bytes_of_f32 * PCD_ELEMS;
    let lines = input_buffer.len() / bytes_per_line;

    let offset_vec = [offset.x_m, offset.y_m, offset.z_m, 0.0];

    let ptr = input_buffer.as_ptr();
    let mut ptr_offset = 0;
    for _ in 0..lines {
        // xyzi
        let mut pcd = Vec::new();
        for index in 0..PCD_ELEMS {
            unsafe {
                let array_u8 = std::slice::from_raw_parts(ptr.add(ptr_offset), bytes_of_f32);
                let data_ptr = (array_u8 as *const [u8]) as *const f32;
                let data = *data_ptr + offset_vec[index];
                pcd.push(data);
                ptr_offset += bytes_of_f32;
            }
        }
        output_pcd_lines.push(pcd);
    }
}

fn output_pcd_file(output_filename: &str, pcd_lines: &Vec<Vec<f32>>) {
    let mut file = File::create(output_filename).expect("new file not created");

    write_pcd_header(pcd_lines.len(), &mut file);

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
            .expect("pcd data not written");
    }
}

fn write_pcd_header(lines: usize, file: &mut File) {
    let mut write_line = |header: &str| {
        let header = header.to_string() + "\n";
        file.write_all(header.as_bytes())
            .expect("header not written")
    };

    let width = "WIDTH ".to_string() + lines.to_string().as_str();
    let points = "POINTS ".to_string() + lines.to_string().as_str();

    let str_vec = [
        "VERSION 0.7",
        "FIELDS x y z i",
        "SIZE 4 4 4 4",
        "TYPE F F F F",
        "COUNT 1 1 1 1",
        width.as_str(),
        "HEIGHT 1",
        "VIEWPOINT 0.0 0.0 0.0 1.0 0.0 0.0 0.0",
        points.as_str(),
        "DATA ascii",
    ];

    for str in str_vec {
        write_line(str);
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

#[cfg(test)]
mod test {
    use super::Offset;
    #[test]
    fn test_offset() {
        let x_m = 1.234 as f32;
        let y_m = 0.0 as f32;
        let z_m = -999.999 as f32;
        let offset = Offset::new(
            &"1.234".to_string(),
            &"0.0".to_string(),
            &"-999.999".to_string(),
        );
        assert_eq!(x_m, offset.x_m);
        assert_eq!(y_m, offset.y_m);
        assert_eq!(z_m, offset.z_m);
    }
}
