use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
//use rand::{Rng, SeedableRng};
use std::collections::VecDeque;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    mpsc, Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

// represents the two workload types used in the simulation
#[derive(Clone, Copy, Debug)]
enum TaskKind {
    IO,
    CPU,
}

// stores the information needed to schedule a task and later calculate its performance metrics
#[derive(Clone, Debug)]
struct Task {
    id: usize,
    kind: TaskKind,
    arrival_time: Instant,
    duration: Duration,
    cpu_cost: usize,
}

// stores the timing data after a worker completes a task
#[derive(Clone, Debug)]//derive(Clone, Debug
struct TaskResult {
    task_id: usize,
    kind: TaskKind,
    wait_time: Duration,
    turnaround_time: Duration,
}

// wraps the task so workers can also receive a shutdown message through none
struct WorkerMessage {
    task: Option<Task>,
}

fn main() {
    run_experiment("Experiment A: Balanced workload 70% IO / 30% CPU", 70, 30);
    println!("\n--------------------------------------------------\n");
    run_experiment("Experiment B: Stressed workload 20% IO / 80% CPU", 20, 80);
}

fn run_experiment(name: &str, io_percent: usize, cpu_percent: usize) {
    println!("{name}");

    let total_tasks = 1000;
    let worker_count = 8;
    let arrival_interval = Duration::from_millis(20);
    let task_duration = Duration::from_millis(200);

    // these atomic counters are shared across threads so the dispatcher monitor and workers can safely read and update the current system state
    let global_cpu = Arc::new(AtomicUsize::new(0));
    let active_workers = Arc::new(AtomicUsize::new(0));
    let queue_len = Arc::new(AtomicUsize::new(0));
    let running = Arc::new(AtomicBool::new(true));

    // the first channel moves tasks from the generator to the dispatcher and the second channel returns completed task results from the workers
    let (task_sender, task_receiver) = mpsc::channel::<Task>();
    let (result_sender, result_receiver) = mpsc::channel::<TaskResult>();

    let mut worker_senders = Vec::new();
    let mut worker_handles = Vec::new();

    let start_time = Instant::now();

    // each worker gets its own channel receiver which lets the dispatcher assign work to a specific available worker
    for worker_id in 0..worker_count {
        let (worker_sender, worker_receiver) = mpsc::channel::<WorkerMessage>();
        worker_senders.push(worker_sender);

        let result_sender_clone = result_sender.clone();
        let global_cpu_clone = Arc::clone(&global_cpu);
        let active_workers_clone = Arc::clone(&active_workers);

        let handle = thread::spawn(move || loop {
            let message = worker_receiver.recv().unwrap();

            match message.task {
                Some(task) => {
                    active_workers_clone.fetch_add(1, Ordering::SeqCst);

                    let start_execution = Instant::now();
                    let wait_time = start_execution.duration_since(task.arrival_time);

                    thread::sleep(task.duration);

                    let turnaround_time = Instant::now().duration_since(task.arrival_time);

                    // after the simulated work finishes the worker releases its cpu allocation and sends its timing results back for logging
                    global_cpu_clone.fetch_sub(task.cpu_cost, Ordering::SeqCst);
                    active_workers_clone.fetch_sub(1, Ordering::SeqCst);

                    result_sender_clone
                        .send(TaskResult {
                            task_id: task.id,
                            kind: task.kind,
                            wait_time,
                            turnaround_time,
                        })
                        .unwrap();
                }
                None => {
                    println!("Worker {worker_id} shutting down.");
                    break;
                }
            }
        });

        worker_handles.push(handle);
    }

    let monitor_global_cpu = Arc::clone(&global_cpu);
    let monitor_active_workers = Arc::clone(&active_workers);
    let monitor_queue_len = Arc::clone(&queue_len);
    let monitor_running = Arc::clone(&running);

    // the monitor samples the system during the experiment so the final output can report average utilization instead of relying on a single end value
    let monitor_handle = thread::spawn(move || {
        let mut samples = 0usize;
        let mut total_cpu = 0usize;
        let mut total_active_workers = 0usize;
        let mut max_queue_len = 0usize;

        while monitor_running.load(Ordering::SeqCst) {
            let cpu = monitor_global_cpu.load(Ordering::SeqCst);
            let workers = monitor_active_workers.load(Ordering::SeqCst);
            let q_len = monitor_queue_len.load(Ordering::SeqCst);

            total_cpu += cpu;
            total_active_workers += workers;
            max_queue_len = max_queue_len.max(q_len);
            samples += 1;

            thread::sleep(Duration::from_millis(10));
        }

        let avg_cpu = if samples > 0 {
            total_cpu as f64 / samples as f64
        } else {
            0.0
        };

        let avg_workers = if samples > 0 {
            total_active_workers as f64 / samples as f64
        } else {
            0.0
        };

        (avg_cpu, avg_workers, max_queue_len)
    });

    // the generator uses a fixed seed so the random task mix is reproducible every time the program runs
    let generator_handle = thread::spawn(move || {
        let mut rng = StdRng::seed_from_u64(42);

        for id in 0..total_tasks {
            let roll = rng.random_range(0..100);
            //let roll = rng.gen_range(0..100);

            let kind = if roll < io_percent {
                TaskKind::IO
            } else {
                TaskKind::CPU
            };

            let cpu_cost = match kind {
                TaskKind::IO => 10,
                TaskKind::CPU => 35,
            };

            let task = Task {
                id,
                kind,
                arrival_time: Instant::now(),
                duration: task_duration,
                cpu_cost,
            };

            task_sender.send(task).unwrap();
            thread::sleep(arrival_interval);
        }
    });

    let dispatcher_global_cpu = Arc::clone(&global_cpu);
    let dispatcher_queue_len = Arc::clone(&queue_len);

    // the dispatcher works like a basic scheduler by holding incoming tasks in a queue tracking free workers and enforcing the simulated cpu limit before sending tasks out
    let dispatcher_handle = thread::spawn(move || {
        let mut queue: VecDeque<Task> = VecDeque::new();
        let mut free_workers: VecDeque<usize> = (0..worker_count).collect();
        let mut completed_count = 0usize;

        loop {
            while let Ok(task) = task_receiver.try_recv() {
                queue.push_back(task);
                dispatcher_queue_len.store(queue.len(), Ordering::SeqCst);
            }

            // finished task results are used to free up workers and are also saved so the program can calculate final metrics
            while completed_count < total_tasks {
                match result_receiver.try_recv() {
                    Ok(result) => {
                        completed_count += 1;
                        free_workers.push_back(result.task_id % worker_count);

                        RESULT_LOG.lock().unwrap().push(result);
                    }
                    Err(_) => break,
                }
            }

            if completed_count == total_tasks {
                break;
            }

            let mut dispatched = false;

            if let Some(worker_index) = free_workers.pop_front() {
                if let Some(task) = queue.front() {
                    let current_cpu = dispatcher_global_cpu.load(Ordering::SeqCst);

                    // a task is only sent to a worker if its cpu cost keeps the simulated cpu usage within the 100 percent cap
                    if current_cpu + task.cpu_cost <= 100 {
                        let task = queue.pop_front().unwrap();
                        dispatcher_queue_len.store(queue.len(), Ordering::SeqCst);

                        dispatcher_global_cpu.fetch_add(task.cpu_cost, Ordering::SeqCst);

                        worker_senders[worker_index]
                            .send(WorkerMessage { task: Some(task) })
                            .unwrap();

                        dispatched = true;
                    } else {
                        free_workers.push_front(worker_index);
                    }
                } else {
                    free_workers.push_front(worker_index);
                }
            }

            if !dispatched {
                thread::sleep(Duration::from_millis(1));
            }
        }

        // once every task has completed the dispatcher sends none to each worker so their loops can exit normally
        for sender in worker_senders {
            sender.send(WorkerMessage { task: None }).unwrap();
        }
    });

    generator_handle.join().unwrap();
    dispatcher_handle.join().unwrap();

    for handle in worker_handles {
        handle.join().unwrap();
    }

    running.store(false, Ordering::SeqCst);
    let (avg_cpu, avg_workers, max_queue_len) = monitor_handle.join().unwrap();

    let makespan = start_time.elapsed();

    // the shared result log is copied and cleared between experiments so each run prints metrics from its own workload only
    let results = RESULT_LOG.lock().unwrap();
    let experiment_results = results.clone();
    drop(results);

    RESULT_LOG.lock().unwrap().clear();

    print_metrics(
        total_tasks,
        makespan,
        avg_cpu,
        avg_workers,
        max_queue_len,
        experiment_results,
        cpu_percent,
    );
}

// shared result storage protected by a mutex because multiple parts of the program access it across threads
static RESULT_LOG: Mutex<Vec<TaskResult>> = Mutex::new(Vec::new());

fn print_metrics(
    total_tasks: usize,
    makespan: Duration,
    avg_cpu: f64,
    avg_workers: f64,
    max_queue_len: usize,
    results: Vec<TaskResult>,
    expected_cpu_percent: usize,
) {
    let completed = results.len();

    let total_wait: Duration = results.iter().map(|r| r.wait_time).sum();
    let total_turnaround: Duration = results.iter().map(|r| r.turnaround_time).sum();

    let avg_wait = total_wait.as_secs_f64() / completed as f64;
    let avg_turnaround = total_turnaround.as_secs_f64() / completed as f64;

    let cpu_tasks = results
        .iter()
        .filter(|r| matches!(r.kind, TaskKind::CPU))
        .count();

    let io_tasks = results
        .iter()
        .filter(|r| matches!(r.kind, TaskKind::IO))
        .count();

    let max_wait = results
        .iter()
        .map(|r| r.wait_time.as_secs_f64())
        .fold(0.0, f64::max);

    // these metrics give a quick summary of completion count workload mix runtime wait time cpu pressure queue size and worker utilization
    println!("Total tasks expected: {total_tasks}");
    println!("Total tasks completed: {completed}");
    println!("CPU task target percentage: {expected_cpu_percent}%");
    println!("CPU tasks completed: {cpu_tasks}");
    println!("IO tasks completed: {io_tasks}");
    println!("Makespan / total runtime: {:.3} seconds", makespan.as_secs_f64());
    println!("Average wait time: {:.3} seconds", avg_wait);
    println!("Average turnaround time: {:.3} seconds", avg_turnaround);
    println!("Max wait time: {:.3} seconds", max_wait);
    println!("Average CPU usage: {:.2}%", avg_cpu);
    println!("Average active workers: {:.2}", avg_workers);
    println!("Max queue length: {max_queue_len}");
}