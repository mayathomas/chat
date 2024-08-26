use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel(16);

    // Will not be seen
    tx.send(10).unwrap();

    let mut rx = tx.subscribe();

    tx.send(20).unwrap();

    let value = rx.recv().await.unwrap();
    println!("Received {}", value);

    let mut rx1 = tx.subscribe();
    tx.send(30).unwrap();

    let value = rx.recv().await.unwrap();
    println!("Received {}", value);
    let value = rx1.recv().await.unwrap();
    println!("Received {}", value);
}
