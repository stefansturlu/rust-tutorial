use std::time::Duration;
use trpl::{self, Either};
use std::{
    pin::{pin,Pin},
    future::Future,
    thread,
    time::Instant
};

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

fn main_4() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        // Note: the move keyword moves tx into the async block, which is dropped at end, 
        //   allowing the other async block to finish
        let tx1_fut = pin!(async move {
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
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
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
        });
        // You can use a macro instead of join, join2, join3, ...
        // trpl::join!(tx1_fut, tx_fut, rx_fut);
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> =
            vec![tx1_fut, rx_fut, tx_fut];
        trpl::join_all(futures).await;
    });
}

fn main_5() {
    trpl::run(
        async {
            let a = async { 1u32 };
            let b = async { "Hello!" };
            let c = async { true };
            
            let (a_result, b_result, c_result) = trpl::join!(a, b, c);
            println!("{a_result}, {b_result}, {c_result}");

        }
    )
}

fn main_6() {
    trpl::run( async {
        let slow = async {
            println!("'slow' started.");
            trpl::sleep(Duration::from_millis(100)).await;
            println!("'slow' finished.");
        };

        let fast = async {
            println!("'fast' started.");
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'fast' finished.");
        };

        trpl::race(slow, fast).await;

    })
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms")
}
fn main_7() {
    // Yieldinng
    trpl::run(async {

        let a = async {
            println!("'a' started.");
            trpl::yield_now().await;
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            trpl::yield_now().await;
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::race(a, b).await;

    });
}

fn main_8() {
    trpl::run(async {
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );

    });
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

fn main_9() {
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "I finished!"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    })
}

fn main() {
    main_9()
}