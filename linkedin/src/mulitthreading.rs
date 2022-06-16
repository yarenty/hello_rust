use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub fn test_threads() {
    let mut handles = vec![];
    let counter = Arc::new(Mutex::new(0));

    for x in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Hello World: {} {}", num, x);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Finish mulitthreads :: {}", *counter.lock().unwrap());
}
