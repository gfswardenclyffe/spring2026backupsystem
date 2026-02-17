// Assignment 2: Number Analyzer

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let nums: [i32; 10] = [12, 7, 9, 10, 15, 22, 3, 5, 30, 11];

    for n in nums {
        if n % 3 == 0 && n % 5 == 0 {
            println!("{} -> FizzBuzz", n);
        } else if n % 3 == 0 {
            println!("{} -> Fizz", n);
        } else if n % 5 == 0 {
            println!("{} -> Buzz", n);
        } else {
            if is_even(n) {
                println!("{} -> even", n);
            } else {
                println!("{} -> odd", n);
            }
        }
    }

    let mut i: usize = 0;
    let mut sum: i32 = 0;
    while i < nums.len() {
        sum = sum + nums[i];
        i = i + 1;
    }
    println!("Sum = {}", sum);

    let mut i: usize = 0;
    let mut largest: i32 = nums[0];
    while i < nums.len() {
        if nums[i] > largest {
            largest = nums[i];
        }
    i += 1;
    }
    println!("Largest = {}", largest);
}

