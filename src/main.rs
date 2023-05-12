
// Celcius to farenheit and vice versa:


// c/5=(F-32)/9


fn main() {
    
    //calling functions
    
    let farenh:f32 = c_to_f(100.00);
    let cel:f32 = f_to_c(farenh);

    println!("{}F",farenh);
    println!("{}C",cel);
    

}

fn c_to_f(temp:f32)-> f32{
    
    let res = 5.0*((temp-32.0)/9.0);
    
    res
}

fn f_to_c(temp:f32)->f32{

    let res = ((9.0/5.0)*temp)+32.0;
    
    res
}


