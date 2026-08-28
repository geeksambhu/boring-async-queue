use tokio::sync::mpsc;

#[derive(Debug)]
struct Job {
    id: u64,
}

#[tokio::main]
async fn main() {
    //Create tx, create rx
    let job = Job { id: 1 };
    let (tx, mut rx) = mpsc::channel::<Job>(3);

    // Send the job to the channel
    if let Err(_) = tx.send(job).await {
        println!("Failed to send job");
    }
    //receive the job from the channel
    tokio::spawn(async move {
        if let Some(received_job) = rx.recv().await {
            println!("Job ID {:?} received", received_job.id);
            return;
        }
    });

    // Wait for a second to allow the spawned task to complete
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
}

// Job

// Channel

// Queue
// Producer
// Queue
