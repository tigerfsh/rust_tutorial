use core::num;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 0..10 {
    //         println!("Loop 1: {}", i);
    //         thread::sleep(Duration::from_millis(500));
    //     }
    // });
    

    // for i in 0..5 {
    //     println!("Loop 2: {}", i);
    //     thread::sleep(Duration::from_millis(500));
    // }

    // handle.join().unwrap();

    let a = 5;

    let handle = thread::spawn(move || {
        println!("a: {}", a);
    });

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1500));
        let a = 5;
        tx.send(a).unwrap();
    });

    let b = rx.recv().unwrap();
    println!("b: {}", b);

    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        logger(tx);
    });

    handle.join().unwrap();
    
    let b = rx.recv().unwrap();
    println!("b: {}", b);

    // how to send multiple values 
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        // let num1 = 1;
        // let num2 = 2;
        // let num3 = 3;
        
        // tx.send(num1).unwrap();
        // tx.send(num2).unwrap();
        // tx.send(num3).unwrap();
        let nums = vec![3, 5, 8];
        for n in nums {
            tx.send(n).unwrap();
        }
    });

    for i in rx {
        println!("i: {i}");
        
    }

    // how to create multiple producers
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move ||{tx.send("Hello").unwrap()});
    thread::spawn(move || {tx1.send("there").unwrap()});
    for v in rx {
        println!("v: {v}");
    }

    // how to handle data races with mutex and arc 
    // Mutex: lock 
    // Arc: thread-safe shared ownership of value 
    let safe = Arc::new(Mutex::new(5));
    let mut handles = vec![];
    for i in 0..2 {
        let safe = Arc::clone(&safe);
        let handle = thread::spawn(move || {
            let mut a = safe.lock().unwrap();
            *a += 3;

        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("a : {}", *safe.lock().unwrap());

    let a = 100;
    handles = vec![];
    let b = Arc::new(a);
    for i in 0..10 {
        let b = Arc::clone(&b);
        let handle = thread::spawn(move || {
            println!("b: {}", b);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
}

fn logger(a: mpsc::Sender<i32>) {
    a.send(500).unwrap();
    println!("send 5 in logger");
}