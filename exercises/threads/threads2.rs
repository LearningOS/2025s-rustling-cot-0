// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

//

use std::sync::{Arc,Mutex,Condvar};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let cond_shared = Arc::clone(&pair);
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut status_locked = status_shared.lock().unwrap();
            status_locked.jobs_completed += 1;
            
            let (lock, cvar) = &*cond_shared;
            lock.lock().unwrap();
            cvar.notify_all();
        });
        handles.push(handle);
    }
    //等待条件变量唤醒打印
    let (lock, cvar) = &*pair;
    
    loop{
        let started = lock.lock().unwrap();
        cvar.wait(started).unwrap();
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
        if status.lock().unwrap().jobs_completed == 10 {
            break;
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("[final]jobs completed {}", status.lock().unwrap().jobs_completed);
}
