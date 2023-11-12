use std::fmt::Display;
pub fn print_slice_elements_using_display<T: Display>(items: &[T]) -> () {
    println!("[");
    for (ix, item) in items.iter().enumerate() {
        println!(" {ix}: {item}");
    }
    println!("]");
}
