use tokio::sync::mpsc;
fn main() {
    //Create tx, create rx
    let (_tx, _rx) = mpsc::channel::<Job>(3);
}
// Job
struct Job {
    id: u64,
}
// Channel

// Queue
// Producer
// Queue
