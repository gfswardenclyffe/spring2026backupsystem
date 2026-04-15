use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    // create sender (tx) and receiver (rx)
    //wrap receiver so multiple threads can safely share it
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    
    //store producer and consumer thread handles
    let mut producer_handles = vec![];
    let mut consumer_handles = vec![];

    // TODO: Create 2 producer threads
    //clone sender so each producer has its own handle and each producer generates half the items
    //save handle
    for id in 0..2 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(id, tx_clone, ITEM_COUNT / 2);
        });
        producer_handles.push(handle);
    }

    // TODO: Create 3 consumer threads
    //clone receiver reference
    //Start consumer thread
    //save hnadle to joint after
    for id in 0..3 {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(id, rx_clone);
        });
        consumer_handles.push(handle);
    }

    // TODO: Wait for all threads to finish
    //wait for all producer to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();//send signal per consumer
    }

    for handle in consumer_handles {
        handle.join().unwrap();//wait for consumers to finish
    }

    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    //Creation of random numbers
    let mut rng = rand::thread_rng();

    //generate number 1 to 100
    //send number to the chanel
    //simulate work delay
    for _ in 0..item_count {
        let num = rng.gen_range(1..101);
        println!("Producer {} produced {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    //lock te reveiver for safaty access
    //receive next value from channel
    loop {
        let value = {
            let receiver = rx.lock().unwrap();
            receiver.recv().unwrap()
        };

        //exit the loop when signal is received
        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal and is exiting", id);
            break;
        }

        //simulation of processing time
        println!("Consumer {} consumed {}", id, value);
        thread::sleep(Duration::from_millis(150));
    }
}