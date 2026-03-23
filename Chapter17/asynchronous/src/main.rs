use std::sync::mpsc;
use std::time::Duration;
use trpl::StreamExt;

fn main() {
    merge_demo();
}

fn merge_demo(){
    trpl::run(async {
        let (tx1, rx1) = trpl::channel();
        let (tx2, rx2) = trpl::channel();

        trpl::spawn_task(async move {
            for i in 1..=5 {
                trpl::sleep(Duration::from_millis(300)).await;
                tx1.send(format!("Source A: {}", i)).unwrap();
            }
        });

        trpl::spawn_task(async move {
            for i in 1..=5 {
                trpl::sleep(Duration::from_millis(500)).await;
                tx2.send(format!("Source B: {}", i)).unwrap();
            }
        });

        let mut s1 = trpl::ReceiverStream::new(rx1);
        let mut s2 = trpl::ReceiverStream::new(rx2);

        //to be done

    });
}

fn stream_demo(){
    trpl::run(async {
       let (tx, rx) = trpl::channel();

        let tx_task = async move {
            let values = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("asynchronous"),
                String::from("world!"),
            ];

            for val in values {
                trpl::sleep(Duration::from_millis(500)).await;
                tx.send(val).unwrap();
            }
        };

        trpl::spawn_task(tx_task);

        let mut stream = trpl::ReceiverStream::new(rx);

        while let Some(val) = stream.next().await{
            println!("{}", val);
        }

    });
}


fn race_demo(){
    trpl::run(async {
        let slow_query = async {
            println!("Started slow query");
            trpl::sleep(Duration::from_secs(2)).await;
            "Some data from server"
        };

        let timeout = async {
            trpl::sleep(Duration::from_secs(1)).await;
            "Error: Time limit reached!"
        };

        let result = trpl::race(slow_query, timeout).await;
        match result {
            trpl::Either::Left(data) => {println!("data from fake server: {}",data)},
            trpl::Either::Right(msg) => {println!("timeout: {}",msg)},
        }

    })
}

async fn task_one(){
    println!("Chapter17: task one started");
    trpl::sleep(Duration::from_millis(500)).await;
    println!("Chapter17: task one ended");
}

async fn task_two(){
    println!("Chapter17: task two started");
    trpl::sleep(Duration::from_millis(500)).await;
    println!("Chapter17: task two ended");
}

fn tasks_demo(){
    trpl::run(async {
        trpl::join(task_two(), task_one()).await;
    })
}