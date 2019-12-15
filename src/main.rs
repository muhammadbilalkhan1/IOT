use std::io;

fn main(){
    //user input data and print it on screen?
    println!("Enter your name.");
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .expect("only name enter.");

    println!("Enter your address.");
    let mut address = String::new();
    io::stdin().read_line(&mut address)
        .expect("only address enter.");

    println!("Enter your age.");
    let mut age = String::new();
    io::stdin().read_line(&mut age)
        .expect("only age enter.");

    println!("your name is: {}",name);   
    
    println!("your address is: {}",address);

    let age:u32 = age.trim().parse().unwrap();
    println!("your age is: {}",age);
    


    
}   
    