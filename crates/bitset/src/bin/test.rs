fn main() {
    let p = dbg!(dbg!(&mut 1usize as *mut usize) as *mut u8);
}
