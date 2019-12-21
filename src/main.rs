// fn main() {
//     let bmi = 18;
    
//     if bmi < 19{
//         println!("underweight");
//     }
//     if bmi > 19 && bmi < 25{
//         println! ("Normal Weight");
//     }
//     if bmi > 25 {
//         println! ("overweight");
//     }


// }

// use std::io;

// fn main(){

//     println!("guess the number");

//     println!("please input your guess");

// let mut guess = String :: new();

// io::stdin().read_line(&mut guess).expect ("failed to read line");

//     println!("you guessed{}",guess); 


// }

use std :: io;

fn main() {

    println!("Enter Height in cm :");
    let mut height = String :: new ();
    io::stdin().read_line(&mut height).expect ("");
    
    println!("Enter weight in kg :");
    let mut weight = String :: new ();
    io::stdin().read_line(&mut weight).expect ("");

    let height: f32 = height.trim().parse().expect("");
    let weight: f32 = weight.trim().parse().expect("");
    
    let mut value:f32 = 100.0;
    value = (height / 100.0) * (height / 100.0);

    println!("Your Height in cm : {}",height);
    println!("Your weight in kg : {}",weight);
    
    let mut bmi: f32 = 25.0;
    bmi = weight / value;
    println!("Your BMI is = {}", bmi);
    
    
    if bmi < 18.5{
        
        println!("underweight");
    }
    else if bmi > 18.5 && bmi < 25.0{
        println!("Normal Weight");
    }
    if bmi > 25.0 {
        println! ("overweight");
    }
    
}
