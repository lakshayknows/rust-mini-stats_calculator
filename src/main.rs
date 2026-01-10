#[allow(unused_variables)]
use std::io;

fn main() {
    loop {
        println!("1.Mean");
        println!("2.Min");
        println!("3.Max");
        println!("4.Quit");
    let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();
        match choice {
            "1" => {
                let numbers = read_numbers();
                let result = mean(&numbers);
                println!("Mean: {}", result);
        },
        "2" => {
                let numbers = read_numbers();
                let result = min(&numbers);
                println!("Min: {}", result);
        },
        "3" => {    
                let numbers = read_numbers();
                let result = max(&numbers);
                println!("Max: {}", result);
        },
        "4" => {
            println!("Exiting program.");
                break;
        },
        _ => {
            panic!("Invalid Option")
        }

        }
    }
}


fn mean(list:&Vec<i32>)-> f64{
    if list.is_empty() {
        panic!("Cannot find any vector");
    }

    let mut sum = 0.0;
    let len = list.len() as f64;
    println!("size of the list: {}",len);
    for i in list{
        sum += *i as f64; 

    } sum/len
}

fn max(list:&Vec<i32>) -> i32{
    if list.is_empty() {
        panic!("Cannot find any vector");
    }
    let mut maximum = list[0];
    for i in list{
        if i > &maximum{
            maximum = *i;
        }        
    } maximum
}

fn min(list:&Vec<i32>) -> i32{
    if list.is_empty() {
        panic!("Cannot find any vector");
    }
    let mut minimum = list[0];
    for i in list{
        if i < &minimum{
            minimum = *i;
        }        
    } minimum
}

fn read_numbers() -> Vec<i32> {
    println!("Input your list of numbers (comma separated):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let mut numbers = Vec::new();

    for item in input.trim().split(',') {
        let num: i32 = item.trim().parse().expect("Invalid number in input");
        numbers.push(num);
    }

    numbers
}
