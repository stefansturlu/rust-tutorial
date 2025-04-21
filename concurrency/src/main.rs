use std::time::Duration;
use trpl;

fn main_1() {
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // Without this await, the spawned task won't finish
        handle.await.unwrap();
    });
}

fn main_2() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        // Here we get the same order every time!

        // Without this, nothing will print
        trpl::join(fut1, fut2).await;
    });
}

fn main_3() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Got: {received}");
    });
}

fn question_3() -> i32 {
    /*
        Say you are given a message-passing channel channel() which is non-blocking and bounded,
        meaning send returns a future that completes once there is capacity in the channel. 
        Assume you also have a function join(a, b) which fairly waits on both its arguments to 
        completion. Then given this async code:
        
        let (tx, mut rx) = channel(16);
        let recv_fut = rx.recv();
        let send_fut = tx.send(0);
        let (n, _) = join(recv_fut, send_fut).await;
        println!("{}", n.unwrap());
        
        What will happen as a result of executing this code?
        
        
        Answer:
        Context: Because the channel is non-blocking, we can create a receive future before sending without looping forever.
    */
    0
}

fn main() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        // Note: the move keyword moves tx into the async block, which is dropped at end, 
        //   allowing the other async block to finish
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    });
}
