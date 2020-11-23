include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// mod bindings;

pub fn rust_add(x: i32, y: i32) -> i32 {
    unsafe { add(x, y) as i32 }
}

fn main() {
    println!("Hello, world! {}", rust_add(2, 3));
    println!("hello");
}
