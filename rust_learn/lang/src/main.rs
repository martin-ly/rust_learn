
fn used_function() {}

//cargo  `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}


fn main() {
    used_function();
    println!("Hello, world!");
}
