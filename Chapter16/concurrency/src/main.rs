use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

fn main() {
    handle_demo();
    move_demo();
    mutex_demo();
    arc_mutex_demo();
    mpsc_demo();

    println!("Chapter16: complete");
}

fn mpsc_demo(){
    let (tx, rx) = mpsc::channel();

    {
        let tx1 = tx.clone();

        thread::spawn(move || {
            tx1.send(String::from("hello world from thread 1")).unwrap();
        });

        thread::spawn(move || {
            tx.send(String::from("hello world from thread 2")).unwrap();
        });
    }


    for received in rx {
        println!("Got: {}", received);
    }

}

fn arc_mutex_demo(){
    // tworzenie licznika wewnątrz Mutexa, Mutex wewnątrz Arc.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // klonowanie Arc (kazdy kolejny wątek)
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);

    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn mutex_demo(){
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
fn move_demo(){
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();

    //println!("Here's a vector: {:?}", v); - to nie zadziała.
}

fn handle_demo() {


    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }

    handle.join().unwrap();
}