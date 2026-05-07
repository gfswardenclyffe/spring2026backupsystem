# CSCI 3334-01 Final Project Report
Concurrent Task Dispatcher in Rust

# 1. Project Title
Concurrent Task Dispatcher in Rust

This project was developed for the CSCI 3334-01 Systems Programming course at the University of Texas Rio Grande Valley.

# 2. Project Summary
This project consists of the design and implementation of a concurrent task dispatching system developed in the Rust programming language. The principal objective of the dispatcher is to simulate a continuous stream of incoming tasks, where such tasks are placed into a centralized shared queue and later assigned to available workers according to the scheduling policy implemented within the system.

The system is structured under a centralized dispatcher architecture. In general terms, the program simulates task generation, queue insertion, task scheduling, worker execution, monitoring, metrics collection, and clean shutdown.

# 3. How to Build and Run

## Build Command
To build the project, run the following command from the project folder:

```bash
cargo build
```

## Run Command
To run the project, use:

```bash
cargo run
```

## Release Mode
To build and run the project in release mode, use:

```bash
cargo build --release
cargo run --release
```

# 4. Command Examples

The main command used to reproduce the experiment results is:

```bash
cargo run --release
```

The program automatically runs both scheduling policies:

- FIFO Scheduler
- Optimized CPU-Aware Scheduler

No additional command-line arguments are required.

# 5. Summary of Design

The project follows a centralized dispatcher architecture. The system is divided into four principal concurrent components:

- Task generator thread
- Dispatcher thread
- Worker pool
- Monitor thread

The task generator thread is responsible for creating tasks dynamically over time. This simulates a real system where tasks arrive gradually instead of appearing all at once.

Once generated, each task is inserted into a centralized shared queue implemented with:

```rust
VecDeque<Task>
```

The shared queue is protected using:

```rust
Arc<Mutex<VecDeque<Task>>>
```

The `Arc` structure allows shared ownership across multiple threads, while the `Mutex` protects the queue from race conditions during insertion and removal operations.

The dispatcher thread acts as the central scheduling layer of the system. It checks the shared queue, worker availability, and current simulated CPU utilization. Based on the scheduling policy, the dispatcher selects the next task and sends it to an available worker through channels.

The worker pool consists of eight worker threads. Each worker receives a task, simulates its execution, updates the metrics, and then becomes available again. A worker can only process one task at a time.

The monitor thread periodically records system behavior during execution, including CPU usage, queue length, active worker count, wait times, and general runtime statistics.

# 6. Task Model

Each task contains the following information:

- task identifier
- arrival time
- task type
- simulated CPU cost
- execution duration

The system recognizes two types of tasks:

- IO-bound tasks
- CPU-bound tasks

IO-bound tasks represent lighter operations and consume approximately 10% simulated CPU utilization. CPU-bound tasks represent heavier operations and consume approximately 35% simulated CPU utilization. Both task types execute for approximately 200 milliseconds.

# 7. Synchronization Strategy

The project combines mutex-protected shared structures, channels, and atomic synchronization primitives.

The shared queue and metrics structure are protected with `Arc<Mutex<T>>` because they require coordinated access from multiple threads.

Channels are used between the dispatcher and the worker threads through:

```rust
mpsc::channel()
```

The purpose of the channels is to safely transfer tasks from the dispatcher to the workers without manually synchronizing every individual task assignment.

The system also uses atomic primitives such as:

```rust
AtomicUsize
AtomicBool
```

These are used for simulated CPU utilization, active worker count, and shutdown/completion flags. Atomic counters were used because these values are updated frequently by multiple threads, and using a mutex for every small counter update would create unnecessary lock contention.

# 8. Scheduling Policies

The project implements two scheduling policies:

- FIFO Scheduler
- Optimized CPU-Aware Scheduler

## FIFO Scheduler

The FIFO Scheduler dispatches tasks in the same order in which they arrive into the queue. This scheduler does not consider task type or current CPU utilization. It was implemented as the baseline model because it is simple, predictable, and useful for comparison.

## Optimized CPU-Aware Scheduler

The Optimized CPU-Aware Scheduler evaluates current simulated CPU utilization before assigning tasks. Its purpose is to improve total runtime by keeping simulated CPU usage closer to the 100% cap.

Instead of blindly following arrival order, the optimized scheduler attempts to select CPU-bound tasks when they fit within the CPU limit. If a CPU-bound task does not fit, the dispatcher may select a lighter IO-bound task to use remaining CPU capacity more efficiently.

# 9. Metrics Collected

The program records several runtime and instrumentation metrics, including:

- total completed tasks
- completed IO tasks
- completed CPU tasks
- average wait time
- average turnaround time
- maximum wait time
- average CPU usage
- maximum CPU usage
- maximum active workers
- maximum queue length
- makespan
- total runtime

These metrics were collected to compare scheduler behavior, queue pressure, CPU utilization, worker activity, and overall system performance.

# 10. Experiment Results

## Experiment No. 1 — FIFO Scheduler

### Configuration

- 1000 total tasks
- 70% IO-bound tasks
- 30% CPU-bound tasks
- 8 worker threads
- CPU cap of 100%

### Results

| Metric | Result |
|---|---:|
| Completed Tasks | 1000 |
| Completed IO Tasks | 729 |
| Completed CPU Tasks | 271 |
| Average Wait Time | 8632.12 ms |
| Average Turnaround Time | 8832.21 ms |
| Max Wait Time | 17483 ms |
| Average CPU Usage | 89.39% |
| Max CPU Usage Seen | 100% |
| Max Active Workers | 8 |
| Max Queue Length | 456 |
| Makespan | 37.73s |
| Total Runtime | 37.73s |

### Observation

The FIFO Scheduler provided predictable execution order and stable baseline behavior. However, because it does not consider task type or CPU pressure, CPU-heavy bursts produced larger queue growth and longer waiting times.

## Experiment No. 2 — Optimized CPU-Aware Scheduler

### Configuration

- 1000 total tasks
- 70% IO-bound tasks
- 30% CPU-bound tasks
- 8 worker threads
- CPU-aware scheduling policy enabled
- CPU cap of 100%

### Results

| Metric | Result |
|---|---:|
| Completed Tasks | 1000 |
| Completed IO Tasks | 729 |
| Completed CPU Tasks | 271 |
| Average Wait Time | 9186.22 ms |
| Average Turnaround Time | 9386.32 ms |
| Max Wait Time | 16156 ms |
| Average CPU Usage | 97.57% |
| Max CPU Usage Seen | 100% |
| Max Active Workers | 8 |
| Max Queue Length | 500 |
| Makespan | 35.37s |
| Total Runtime | 35.37s |

### Observation

The Optimized CPU-Aware Scheduler improved total runtime by increasing simulated CPU saturation and keeping the system closer to the CPU cap. Although the average wait time increased, the makespan improved because the scheduler used available CPU capacity more aggressively and reduced idle capacity during execution.

# 11. Experiment Comparison

| Metric | FIFO Scheduler | Optimized Scheduler |
|---|---:|---:|
| Completed Tasks | 1000 | 1000 |
| Completed IO Tasks | 729 | 729 |
| Completed CPU Tasks | 271 | 271 |
| Average Wait Time | 8632.12 ms | 9186.22 ms |
| Average Turnaround Time | 8832.21 ms | 9386.32 ms |
| Max Wait Time | 17483 ms | 16156 ms |
| Average CPU Usage | 89.39% | 97.57% |
| Max CPU Usage Seen | 100% | 100% |
| Max Active Workers | 8 | 8 |
| Max Queue Length | 456 | 500 |
| Makespan | 37.73s | 35.37s |
| Total Runtime | 37.73s | 35.37s |

The FIFO Scheduler performed better in average wait time, but the Optimized CPU-Aware Scheduler performed better in total runtime. The main reason is that the optimized scheduler kept simulated CPU utilization higher throughout execution. This demonstrates that scheduler performance depends on the metric being optimized. FIFO is simpler and more predictable, while the optimized scheduler improves makespan by prioritizing resource utilization.

# 12. Tool Use Disclosure

This project utilized AI-assisted references and external technical documentation for purposes of conceptual clarification, debugging support, concurrency design analysis, and refinement of the scheduling architecture implemented throughout the system.

Among the principal resources consulted during development were:

- Rust documentation
- Cargo documentation
- GitHub community references
- online concurrency and synchronization materials
- ChatGPT
- Claude

Regarding the recommendations accepted during development, one of the principal suggestions involved replacing mutex-protected counters with lightweight atomic synchronization primitives for handling simulated CPU utilization and active worker counters.

Initially, portions of the system utilized mutex-based synchronization through structures similar to:

```rust
Arc<Mutex<usize>>
```

Such recommendation was accepted because the counters were updated continuously by multiple threads simultaneously, which generated unnecessary lock contention during execution. Consequently, the implementation was modified into atomic-based synchronization through:

```rust
AtomicUsize
```

Such modification improved concurrency efficiency by reducing excessive locking for simple counter operations.

Moving on to the recommendations rejected during development, one preliminary design proposal suggested protecting most shared structures through a single centralized mutex. Although functional, such approach was ultimately rejected because it generated excessive contention between worker threads, dispatcher operations, and monitoring activities throughout execution of the simulation.

Therefore, the synchronization strategy was redesigned by separating responsibilities between queue protection, metrics synchronization, and atomic counters, thereby improving scalability and overall thread coordination within the system.

# 13. Conclusion

The development of this project demonstrated that concurrent systems are more complex than traditional sequential programs. One of the principal observations was the importance of separating responsibilities between threads for purposes of organization, synchronization, and debugging.

The project also demonstrated that scheduling policies and synchronization strategies directly affect system behavior and performance. The FIFO Scheduler provided predictable baseline behavior, while the Optimized CPU-Aware Scheduler improved total runtime by increasing CPU saturation. However, the optimized strategy also introduced additional scheduling complexity and possible fairness trade-offs.

Overall, the project demonstrated that improving performance in concurrent systems requires balancing efficiency, fairness, synchronization cost, and scheduling complexity.

# 14. Author

Gustavo Franco-Sanchez  
CSCI 3334-01 Systems Programming  
University of Texas Rio Grande Valley  
2026