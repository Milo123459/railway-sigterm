#[tokio::main]
async fn main() {
    let mut signal =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();
    tokio::spawn(async move {
        loop {
            println!("loop loop");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    });
    tokio::select! {
        _ = signal.recv() => {
            println!("received sigterm");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            std::process::exit(0);
        }
    }
}
