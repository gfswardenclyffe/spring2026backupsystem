use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const TOTAL_TASKS: usize = 1000;
const WORKER_COUNT: usize = 8;
const ARRIVAL_INTERVAL_MS: u64 = 20;
const TASK_DURATION_MS: u64 = 200;
const MONITOR_INTERVAL_MS: u64 = 10;
const CPU_LIMIT: usize = 100;
const RANDOM_SEED: u64 = 12345;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TaskType {
    IO,
    CPU,
}

#[derive(Clone, Debug)]
struct Task {
    id: usize,
    arrival_time: Instant,
    task_type: TaskType,
    cpu_cost: usize,
    duration_ms: u64,
}

#[derive(Clone, Copy, Debug)]
enum Policy {
    Fifo,
    Optimized,
}

#[derive(Default, Debug)]
struct Metrics {
    total_wait_time_ms: u128,
    total_turnaround_time_ms: u128,
    max_wait_time_ms: u128,
    completed_tasks: usize,
    completed_io: usize,
    completed_cpu: usize,
    total_cpu_samples: usize,
    sample_count: usize,
    max_cpu_seen: usize,
    max_active_workers: usize,
    max_queue_len: usize,
}

fn main() {
    println!("Experiment 1: Balanced workload 70% IO / 30% CPU");
    run_simulation("FIFO baseline", Policy::Fifo, 70, 30);
    run_simulation("Optimized scheduler", Policy::Optimized, 70, 30);
}

fn run_simulation(name: &str, policy: Policy, io_percent: usize, cpu_percent: usize) {
    println!("\n=== {} ===", name);

    let start = Instant::now();

    let queue = Arc::new(Mutex::new(VecDeque::<Task>::new()));
    let metrics = Arc::new(Mutex::new(Metrics::default()));

    let current_cpu = Arc::new(AtomicUsize::new(0));
    let active_workers = Arc::new(AtomicUsize::new(0));
    let generated_done = Arc::new(AtomicBool::new(false));
    let dispatcher_done = Arc::new(AtomicBool::new(false));

    let mut worker_senders = Vec::new();
    let mut worker_handles = Vec::new();

    for worker_id in 0..WORKER_COUNT {
        let (tx, rx) = mpsc::channel::<Option<Task>>();
        worker_senders.push(tx);

        let metrics_clone = Arc::clone(&metrics);
        let current_cpu_clone = Arc::clone(&current_cpu);
        let active_workers_clone = Arc::clone(&active_workers);

        let handle = thread::spawn(move || {
            while let Ok(message) = rx.recv() {
                match message {
                    Some(task) => {
                        active_workers_clone.fetch_add(1, Ordering::SeqCst);
                        current_cpu_clone.fetch_add(task.cpu_cost, Ordering::SeqCst);

                        let wait_time = task.arrival_time.elapsed().as_millis();

                        thread::sleep(Duration::from_millis(task.duration_ms));

                        let turnaround_time = task.arrival_time.elapsed().as_millis();

                        current_cpu_clone.fetch_sub(task.cpu_cost, Ordering::SeqCst);
                        active_workers_clone.fetch_sub(1, Ordering::SeqCst);

                        let mut m = metrics_clone.lock().unwrap();
                        m.completed_tasks += 1;
                        m.total_wait_time_ms += wait_time;
                        m.total_turnaround_time_ms += turnaround_time;
                        m.max_wait_time_ms = m.max_wait_time_ms.max(wait_time);
                        match task.task_type {
                            TaskType::IO => m.completed_io += 1,
                            TaskType::CPU => m.completed_cpu += 1,
                        }
                    }
                    None => break,
                }
            }
            println!("Worker {} shutting down.", worker_id);
        });

        worker_handles.push(handle);
    }

    let generator_queue = Arc::clone(&queue);
    let generator_done = Arc::clone(&generated_done);
    let generator_handle = thread::spawn(move || {
        let mut seed = RANDOM_SEED;

        for id in 0..TOTAL_TASKS {
            let task_type = choose_task_type(&mut seed, io_percent, cpu_percent);
            let task = match task_type {
                TaskType::IO => Task {
                    id,
                    arrival_time: Instant::now(),
                    task_type,
                    cpu_cost: 10,
                    duration_ms: TASK_DURATION_MS,
                },
                TaskType::CPU => Task {
                    id,
                    arrival_time: Instant::now(),
                    task_type,
                    cpu_cost: 35,
                    duration_ms: TASK_DURATION_MS,
                },
            };

            generator_queue.lock().unwrap().push_back(task);
            thread::sleep(Duration::from_millis(ARRIVAL_INTERVAL_MS));
        }

        generator_done.store(true, Ordering::SeqCst);
    });

    let monitor_metrics = Arc::clone(&metrics);
    let monitor_cpu = Arc::clone(&current_cpu);
    let monitor_workers = Arc::clone(&active_workers);
    let monitor_queue = Arc::clone(&queue);
    let monitor_done = Arc::clone(&dispatcher_done);

    let monitor_handle = thread::spawn(move || {
        while !monitor_done.load(Ordering::SeqCst) {
            let cpu_now = monitor_cpu.load(Ordering::SeqCst);
            let workers_now = monitor_workers.load(Ordering::SeqCst);
            let queue_len = monitor_queue.lock().unwrap().len();

            let mut m = monitor_metrics.lock().unwrap();
            m.total_cpu_samples += cpu_now;
            m.sample_count += 1;
            m.max_cpu_seen = m.max_cpu_seen.max(cpu_now);
            m.max_active_workers = m.max_active_workers.max(workers_now);
            m.max_queue_len = m.max_queue_len.max(queue_len);

            drop(m);
            thread::sleep(Duration::from_millis(MONITOR_INTERVAL_MS));
        }
    });

    let dispatcher_queue = Arc::clone(&queue);
    let dispatcher_generated_done = Arc::clone(&generated_done);
    let dispatcher_done_clone = Arc::clone(&dispatcher_done);
    let dispatcher_cpu = Arc::clone(&current_cpu);
    let dispatcher_workers = Arc::clone(&active_workers);

    let dispatcher_handle = thread::spawn(move || {
        let mut next_worker = 0;

        loop {
            let should_stop = dispatcher_generated_done.load(Ordering::SeqCst)
                && dispatcher_queue.lock().unwrap().is_empty()
                && dispatcher_workers.load(Ordering::SeqCst) == 0;

            if should_stop {
                break;
            }

            if dispatcher_workers.load(Ordering::SeqCst) >= WORKER_COUNT {
                thread::sleep(Duration::from_millis(1));
                continue;
            }

            let current_cpu_value = dispatcher_cpu.load(Ordering::SeqCst);

            let task_option = match policy {
                Policy::Fifo => pop_fifo_task(&dispatcher_queue, current_cpu_value),
                Policy::Optimized => pop_optimized_task(&dispatcher_queue, current_cpu_value),
            };

            if let Some(task) = task_option {
                let sender = &worker_senders[next_worker];
                sender.send(Some(task)).unwrap();
                next_worker = (next_worker + 1) % WORKER_COUNT;
            } else {
                thread::sleep(Duration::from_millis(1));
            }
        }

        for sender in worker_senders {
            sender.send(None).unwrap();
        }

        dispatcher_done_clone.store(true, Ordering::SeqCst);
    });

    generator_handle.join().unwrap();
    dispatcher_handle.join().unwrap();

    for handle in worker_handles {
        handle.join().unwrap();
    }

    monitor_handle.join().unwrap();

    let elapsed = start.elapsed();
    let m = metrics.lock().unwrap();
    let average_cpu = if m.sample_count == 0 {
        0.0
    } else {
        m.total_cpu_samples as f64 / m.sample_count as f64
    };

    println!("Completed tasks: {}", m.completed_tasks);
    println!("Completed IO tasks: {}", m.completed_io);
    println!("Completed CPU tasks: {}", m.completed_cpu);

    let average_wait_time = if m.completed_tasks == 0 {
        0.0
    } else {
        m.total_wait_time_ms as f64 / m.completed_tasks as f64
    };

    let average_turnaround_time = if m.completed_tasks == 0 {
        0.0
    } else {
        m.total_turnaround_time_ms as f64 / m.completed_tasks as f64
    };

    println!("Average wait time: {:.2} ms", average_wait_time);
    println!("Average turnaround time: {:.2} ms", average_turnaround_time);
    println!("Max wait time: {} ms", m.max_wait_time_ms);
    println!("Average CPU usage: {:.2}%", average_cpu);
    println!("Max CPU usage seen: {}%", m.max_cpu_seen);
    println!("Max active workers: {}", m.max_active_workers);
    println!("Max queue length: {}", m.max_queue_len);
    println!("Makespan: {:.2?}", elapsed);
    println!("Total runtime: {:.2?}", elapsed);
}

fn choose_task_type(seed: &mut u64, io_percent: usize, cpu_percent: usize) -> TaskType {
    let cycle = io_percent + cpu_percent;
    let random_number = next_random(seed) as usize;
    let position = random_number % cycle;

    if position < io_percent {
        TaskType::IO
    } else {
        TaskType::CPU
    }
}

fn next_random(seed: &mut u64) -> u64 {
    // Simple linear congruential generator.
    // This keeps the workload random but reproducible.
    *seed = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1);
    *seed
}

fn pop_fifo_task(queue: &Arc<Mutex<VecDeque<Task>>>, current_cpu: usize) -> Option<Task> {
    let mut q = queue.lock().unwrap();

    if let Some(task) = q.front() {
        if current_cpu + task.cpu_cost <= CPU_LIMIT {
            return q.pop_front();
        }
    }

    None
}

fn pop_optimized_task(queue: &Arc<Mutex<VecDeque<Task>>>, current_cpu: usize) -> Option<Task> {
    let mut q = queue.lock().unwrap();

    if q.is_empty() {
        return None;
    }

    // If CPU usage is already high, prefer IO tasks.
    // If CPU usage is low, allow CPU tasks so they do not starve.
    let prefer_io = current_cpu >= 65;

    let selected_index = q.iter().position(|task| {
        current_cpu + task.cpu_cost <= CPU_LIMIT
            && if prefer_io {
                task.task_type == TaskType::IO
            } else {
                true
            }
    });

    if let Some(index) = selected_index {
        q.remove(index)
    } else {
        None
    }
}
