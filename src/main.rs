
// Celcius to farenheits and vice versa:


// c/5=(F-32)/9

use std::io;

fn main() {
    
    //calling functions
    let mut temp = String::new();
    println!("CELCIUS TO FARENHEIT CONVERTERRRRRRRRRR");
    println!("---------------------------------------");
    println!("Enter Temperature in celcius: ");
    println!("---------------------------------------");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    
    let temp:f32 = temp.trim().parse().expect("Enter a valid temperature");

    let farenh:f32 = c_to_f(temp);
    let cel:f32 = f_to_c(farenh);

    println!("IN FARENHEIT---> {}F",farenh);
    println!("IN CELCIUS---> {}C",cel);
    

}

fn c_to_f(temp:f32)-> f32{
    
    let res = 5.0*((temp-32.0)/9.0);
    
    res
}

fn f_to_c(temp:f32)->f32{

    let res = ((9.0/5.0)*temp)+32.0;
    
    res
}


