unsafe extern "C" {
    fn add(x: i32, y: i32) -> i32;
}

fn main() {
    let result = unsafe { add(1, 2) };
    println!("{result}");
}
