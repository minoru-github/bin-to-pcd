fn main() {
    let buf : [u8;4] = [0x83,0xC0,0xB5,0x41];
    unsafe {
        let p = &buf as *const [u8] as *const f32;
        println!("{}", *p);
    }
}
