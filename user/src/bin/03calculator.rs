#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("=== Simple Calculator App ===");
    
    let numbers = [10, 25, 7, 42, 16];
    let mut sum = 0;
    let mut max = numbers[0];
    let mut min = numbers[0];
    
    println!("Numbers: ");
    for i in 0..numbers.len() {
        println!("[{}]: {}", i, numbers[i]);
        sum += numbers[i];
        if numbers[i] > max { max = numbers[i]; }
        if numbers[i] < min { min = numbers[i]; }
    }
    
    println!("Statistics:");
    println!("Sum: {}", sum);
    println!("Average: {}", sum as f32 / numbers.len() as f32);
    println!("Maximum: {}", max);
    println!("Minimum: {}", min);
    
    println!("Calculator completed successfully!");
    0
}