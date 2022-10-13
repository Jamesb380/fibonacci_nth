//Finds the nth element of Fibonacci's sequence

use std::io;
fn main() {
let n: u8;

n = ini();

println!("The fibonacci {}th element is {}", n, nth_value(n));

}

fn ini() -> u8 {

    let mut n = String::new();
    println!("Please enter nth Fibonacci");
    io::stdin().read_line(&mut n).expect("Failed to readline");
    let n: u8 = match n.trim().parse::<_>(){
        Ok(num) => num,
        Err(_) => 255,
};
n
}

fn nth_value(i:u8) -> u64{
    let mut f:u64 = 0;
    let mut l:u64 =1;
    let mut temp:u64 = 0;
    
    for number in 2..i{
        
        temp = l;
        l = l + f;
        f = temp;
        

    };
    l

}

