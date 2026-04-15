use std::thread;

fn main() {
    let mut threads = vec![];

    for i in 1..=5 {
        let t = thread::spawn(move || {
            println!("Thread {}", i);
        });

        threads.push(t);
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("All threads completed.");
}




