// #[derive(Debug)]
#[allow(unused_variables)]
#[allow(dead_code)]
use std::io;
struct Dataset {
    list: Vec<i32> // owns the list
}

fn main() {
    loop {
        println!("1. Mean");
        println!("2. Min");
        println!("3. Max");
        println!("4. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();

        match choice {
            "1" => {
                let dataset = Dataset::from_input();
                println!("Mean: {}", dataset.mean());
            }
            "2" => {
                let dataset = Dataset::from_input();
                println!("Min: {}", dataset.min());
            }
            "3" => {
                let dataset = Dataset::from_input();
                println!("Max: {}", dataset.max());
            }
            "4" => {
                println!("Exiting program.");
                break;
            }
            _ => {
                println!("Invalid option");
            }
        }
    }
}


impl Dataset {
    fn max(&self) -> i32 {
        if self.list.is_empty(){
            panic!("Empty list")
        }

        // reference to the struct's instance list
        let mut maximum = self.list[0]; // doesn't change the ownership of the dataset
        for i in &self.list{ // ownership changes that is why we are using borrowing &self.list
            if *i>maximum{ // accessing the exact value instead of the pointer
                maximum = *i; // updating the exact value instead of the pointer using the dereferencer
            }
        } maximum
    }
    fn min (&self) -> i32 {
        if self.list.is_empty(){
            panic!("Empty list")
        }
        let mut minimum = self.list[0];
        for i in &self.list{
            if *i < minimum{
                minimum = *i;
            }
        } minimum
    }
    fn mean (&self) -> f64 {
        if self.list.is_empty(){
            panic!("Empty list")
        }
        let mut sum = 0.0;
        let len = self.list.len() as f64; 
        for i in &self.list{
            sum += *i as f64;    
        } sum/len
    }
}

impl Dataset {
    fn from_input() -> Dataset {
        println!("Input your list of numbers (comma separated):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let mut numbers = Vec::new();

        for item in input.trim().split(',') {
            let num: i32 = item.trim().parse().expect("Invalid number in input");
            numbers.push(num);
        }

        Dataset { list: numbers }
    }
}
