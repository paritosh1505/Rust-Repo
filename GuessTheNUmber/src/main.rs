use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let input_handle = io::stdin();

loop{
    let mut rand_num = rand::thread_rng();
    let ran_num:u32 = rand_num.gen_range(10..=99);
    println!("random number is {}",ran_num);
    
    let mut inputnum = String::new();
    
    input_handle.read_line(&mut inputnum).expect("Failed to read data");
    let parse_num:u32 = inputnum.trim().parse().expect("Please enter vaid number");
    
    
    println!("input num is {}",parse_num);
    
    match parse_num.cmp(&ran_num){
        Ordering::Less=>println!("Number is less"),
        Ordering::Equal => {
            println!("Number is equal");
            break;
        },
        Ordering :: Greater =>println!("Number is greater"),
    
    }
}


   
}
