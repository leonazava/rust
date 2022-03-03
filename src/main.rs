use std::io;

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Please enter an array index");
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered wasn't a number");
    let element = a[index];

    println!("the value of the element at index {} is: {}", index, element);
}