
//Functions for assignment 1.
fn fahrenheit_to_celsius(fah:f64) -> f64{
    return (fah-32.0)/1.8;
}
fn celsius_to_fahernheit(cel: f64) ->f64{
    return (cel*1.8)/32.0;
}
fn ass1(){

    let mut temp = 33;  //33 Fahernheit
    let mut set = 0;    //To limit loop

    loop{   //Should go from 33F to 38F.
        println!("Fah:{} Cel:{}",temp, fahrenheit_to_celsius(temp as f64));
        set += 1;
        temp += 1;

        if set == 6{
            break;
        }
    }
}

//Functions for assignment 2.
fn is_even(n: i32) -> bool{
    
    if n%2==0 {
        return true;
        }
    else {
        return false;
        }
}
fn ass2(){
    
    let arr: [i32;10] = [15, 21, 69, 88, 55, 34, 41, 44, 67, 30];

    for num in arr.iter(){
        
        println!("");
        println!("{} is {}", num, is_even(*num));

        if num%3==0 && num%5==0{
            println!("{} is FizzBuzz", num);
        }
        else if num%3==0{
            println!("{} is Fizz", num);
        }
        else if num%5==0{
            println!("{} is Buzz", num);
        }
        else{
            println!("{} is not divisible by 3 or 5.", num);
        }
    }

    println!("");

    let mut i = 0;
    let mut sum = 0;
    while i != arr.len(){
        
        sum = sum + arr[i];
        i+=1;
    }

    println!("Sum is {}", sum);

    let mut arr_counter = 0;
    let mut max = arr[0];
    loop{
        
        if arr[arr_counter] > max{
            max = arr[arr_counter];
        }
        arr_counter +=1;

        if arr_counter == arr.len(){
            break;
        }
    }
    println!("Max is {}", max);
    println!("");
}

//Functions for assignment 3.
use std::io;
fn guess_number(gu: i32, se: i32) -> i32{

    if gu < se{
        return -1;
    }
    else if gu > se{
        return 1;
    }
    else{
        return 0;
    }
}
fn ass3(){
    
    let secert_num = 67;
    let mut guess_count = 1;

    loop{
        println!("Input guess.");
        let mut guess = String::new();
        
        let _ = io::stdin()
            .read_line(&mut guess);

        let guess_input: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => todo!()
        };
        
        if guess_number(guess_input, secert_num) == -1{
            println!("Guess is too low");
            println!("");
            guess_count += 1;
        }
        else if guess_number(guess_input, secert_num) == 1{
            println!("Guess is too high");
            guess_count += 1;
            println!("");
        }
        else{
            println!("Guess is correct.");
            println!("");
            break;
        }
    }

    println!("Amount of guesses: {}", guess_count);
}

fn main(){

    
    ass1(); //Assignment 1
    ass2(); //Assignment 2
    ass3(); //Assignment 3
}