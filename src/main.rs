use futures::channel::mpsc::{unbounded, UnboundedSender};
use futures::{SinkExt, StreamExt};

use async_std::task;

async fn case1(mut channel: UnboundedSender<String>) -> bool {
    let s = format!("{}", "snot");
    channel.send(s).await.is_ok()
}

async fn case2(mut channel: UnboundedSender<String>) -> bool {
    let f = channel.send(format!("{}", "snot"));
    f.await.is_ok()
}

async fn case3(mut channel: UnboundedSender<String>) -> bool {
    channel.send(format!("{}", "snot")).await.is_ok()
}

fn main() {
    let (tx, rx) = unbounded();
    let tx1 = tx.clone();
    task::spawn(case1(tx1));
    let tx2 = tx.clone();
    task::spawn(case2(tx2));
    let tx3 = tx.clone();
    task::spawn(case3(tx3));
    println!("Hello, world!");
}
