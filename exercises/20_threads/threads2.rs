// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{sync::{Arc, Mutex}, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 })); 
    // JobStatus хранит jobs_done;
    // Mutex разрешает изменять его только одному потоку одновременно;
    // Arc позволяет нескольким потокам владеть одним Mutex.

    let mut handles = Vec::new();
    for _ in 0..10 { 
        let status_shared = Arc::clone(&status); //  Для каждого потока создаётся ещё один Arc, указывающий на тот же Mutex<JobStatus>. Сам JobStatus не копируется

        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value. 
            let mut status = status_shared.lock().unwrap(); 
            // lock() пытается получить доступ к JobStatus 
            // если другой поток уже изменяет данные, текущий поток ждёт; 
            // status - это фактически MutexGuard<JobStatus> 
            status.jobs_done += 1;
        }); 
        // После выхода из замыкания MutexGuard уничтожается, и Mutex разблокируется 
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done); 

    // status                  // Arc<Mutex<JobStatus>>
    // status.lock()            // Result<MutexGuard<JobStatus>, ...>
    // status.lock().unwrap()   // MutexGuard<JobStatus>
    // status.lock().unwrap().jobs_done // u32
}
