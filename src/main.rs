use tokio::sync::mpsc;

#[derive(Debug)]
struct Job {
    id: u64,
}

#[tokio::main]
async fn main() {
    //Create tx, create rx
    let (tx, mut rx) = mpsc::channel::<Job>(3);
    let handle = tokio::spawn(async move {
        while let Some(received_job) = rx.recv().await {
            println!("Job ID {:?} received", received_job.id);
        }
    });
    for i in 0..3 {
        let job = Job { id: i };

        // Send the job to the channel
        if let Err(e) = tx.send(job).await {
            println!("Failed to send job: {:?}", e);
        }
    }
    // Close the channel by dropping the sender so receiver can exit the loop
    drop(tx);

    if let Err(e) = handle.await {
        println!("Error: {:?}", e);
    }
}

// Job

// Channel

// Queue
// Producer
// Queue
