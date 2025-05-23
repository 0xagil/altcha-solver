use sha2::{Sha256, Digest};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::time::Instant;
use std::thread;
use crossbeam_channel::{bounded, Sender};

#[derive(Debug)]
struct Challenge {
    algorithm: String,
    challenge: String,
    max_number: u32,
    salt: String,
    signature: String,
}

#[derive(Debug)]
struct Result {
    number: u32,
    took: f64,
}

fn hash_check(salt: &str, num: u32, target: &str) -> bool {
    let test_string = format!("{}{}", salt, num);
    let mut hasher = Sha256::new();
    hasher.update(test_string.as_bytes());
    let result = hasher.finalize();
    let hash = hex::encode(result);
    hash == target
}

fn forward_worker(
    start: u32,
    end: u32,
    salt: String,
    target: String,
    sender: Sender<Result>,
    found: Arc<AtomicBool>,
    start_time: Instant
) {
    for num in start..end {
        if found.load(Ordering::Relaxed) {
            return;
        }

        if hash_check(&salt, num, &target) {
            found.store(true, Ordering::Relaxed);
            let took = start_time.elapsed().as_millis() as f64;
            let _ = sender.send(Result { number: num, took });
            return;
        }

        if num % 10_000 == 0 {
            println!("Forward worker tried: {}", num);
        }
    }
}

fn backward_worker(
    start: u32,
    end: u32,
    salt: String,
    target: String,
    sender: Sender<Result>,
    found: Arc<AtomicBool>,
    start_time: Instant
) {
    for num in (end..start).rev() {
        if found.load(Ordering::Relaxed) {
            return;
        }

        if hash_check(&salt, num, &target) {
            found.store(true, Ordering::Relaxed);
            let took = start_time.elapsed().as_millis() as f64;
            let _ = sender.send(Result { number: num, took });
            return;
        }

        if num % 10_000 == 0 {
            println!("Backward worker tried: {}", num);
        }
    }
}

fn solve_challenge(challenge: &Challenge) -> Option<Result> {
    let start_time = Instant::now();
    let found = Arc::new(AtomicBool::new(false));
    let (sender, receiver) = bounded(1);

    let midpoint = challenge.max_number / 2;
    let workers_per_direction = 10;
    let forward_batch_size = midpoint / workers_per_direction;
    let backward_batch_size = (challenge.max_number - midpoint) / workers_per_direction;

    let mut handles = vec![];

    // Forward workers
    for i in 0..workers_per_direction {
        let start = i * forward_batch_size;
        let end = if i == workers_per_direction - 1 {
            midpoint
        } else {
            start + forward_batch_size
        };

        let salt = challenge.salt.clone();
        let target = challenge.challenge.clone();
        let sender = sender.clone();
        let found = Arc::clone(&found);
        let start_time = start_time;

        handles.push(thread::spawn(move || {
            forward_worker(start, end, salt, target, sender, found, start_time);
        }));
    }

    // Backward workers
    for i in 0..workers_per_direction {
        let start = challenge.max_number - (i * backward_batch_size);
        let end = if i == workers_per_direction - 1 {
            midpoint
        } else {
            start - backward_batch_size
        };

        let salt = challenge.salt.clone();
        let target = challenge.challenge.clone();
        let sender = sender.clone();
        let found = Arc::clone(&found);
        let start_time = start_time;

        handles.push(thread::spawn(move || {
            backward_worker(start, end, salt, target, sender, found, start_time);
        }));
    }

    // Wait for result
    let result = receiver.recv().ok();

    // Set found flag to stop other workers
    found.store(true, Ordering::Relaxed);

    // Wait for all threads to finish
    for handle in handles {
        let _ = handle.join();
    }

    result
}

fn main() {
    let challenge = Challenge {
        algorithm: "SHA-256".to_string(),
        challenge: "11d22bf8463d767164170a40f8398e21ce61b14b68a0d9638690b712239f1b4b".to_string(),
        max_number: 150000,
        salt: "10c8977c6e2142024387a52c?expires=1746012392".to_string(),
        signature: "82f4a474856d66c3678550246e0612a17f8f5d47f8fe5b99a0821c2725a05119".to_string(),
    };

    println!("Starting bidirectional hash challenge solver...");

    if let Some(result) = solve_challenge(&challenge) {
        println!("\nFound solution: {}", result.number);
        println!("Time taken: {:.2}ms", result.took);

        // Verify solution
        if hash_check(&challenge.salt, result.number, &challenge.challenge) {
            println!("Hash verified successfully");

            // Show position relative to midpoint
            let midpoint = challenge.max_number / 2;
            if result.number < midpoint {
                println!("Found in lower half (0 -> {})", midpoint);
            } else {
                println!("Found in upper half ({} -> {})", midpoint, challenge.max_number);
            }
        } else {
            println!("Hash verification failed");
        }
    } else {
        println!("No solution found");
    }
}