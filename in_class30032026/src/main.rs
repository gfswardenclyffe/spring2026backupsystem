//Task 1
//fn main() {
//    let operation = |a: i32, b: i32| {
//        a * b
//    };

//    println!("Result: {}", operation(10, 5));
//}

//Task 2
//fn track_changes() {
//    let mut tracker = 0;
//    let mut update = || {
//        tracker += 1;//implementation
//        println!("Tracker: {}", tracker);
//    };

//    update();
//    update();
//}

//fn main() {
//    track_changes();
//}

//Task 3
//fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
//where
//    F: Fn(i32) -> i32,
//{
//    vec.into_iter().map(f).collect()//implementation
//}

//fn main() {
//    let numbers = vec![1, 2, 3];

//    let doubled = process_vector(numbers.clone(), |x| {
//        x * 2//implementation
//    });

//    let replaced = process_vector(numbers, |x| {
//        if x > 2 {
//            0
//        } else {
//            x
//        }
//    });

//    println!("Doubled: {:?}", doubled);
//    println!("Replaced: {:?}", replaced);
//}

//Task 4
//use std::{thread, time::Duration};

//struct ComputeCache<T>
//where
//    T: Fn() -> String,
//{
//    cpt: T,//implementation
//    x: Option<String>,//implementation
//}

//impl<T> ComputeCache<T>
//where
//    T: Fn() -> String,
//{
//    fn new(computation: T) -> Self {
//        ComputeCache {
//            cpt: computation,
//            x: None,
//        }
//    }

//    fn get_result(&mut self) -> String {
//        match &self.x {
//            Some(x) => {
//                println!("Retrieved from cache instantly!");
//                x.clone()
//            }
//            None => {
//                let result = (self.cpt)();
//                self.x = Some(result.clone());
//                result
//            }
//        }
//    }
//}

//fn main() {
//    let mut cache = ComputeCache::new(|| {
//        println!("Computing (this will take 2 seconds)...");
//        thread::sleep(Duration::from_secs(2));
//        "Hello, world!".to_string()
//    });

//    println!("First call:");
//    println!("Result: {}", cache.get_result());
    
//    println!("\nSecond call:");
//    println!("Result (cached): {}", cache.get_result());
//}

//task 3.1 
fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();

    for value in vec {
        result.push(f(value));
    }

    result
}
fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        x * 2
    });

    let replaced = process_vector(numbers, |x| {
        if x > 2 {
            0
        } else {
            x
        }
    });
    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}