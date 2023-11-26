use std::io;

fn main() {
    let arr = [1, 2, 3, 4, 5];

    let mut index = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];
    println!("The element at index {index} is: {element}");


}