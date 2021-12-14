
use std::cmp;
use std::collections::HashMap;
use std::sync::{mpsc, Arc};
use std::thread;

/* a reasonable threshold */
const MAX_THREADS: usize = 16;

/*
 * A function that instantiates hash maps for the
 * counting of characters occurences, given a string.
 */
fn counter(string: &str, mut hash: HashMap<char,usize>) -> HashMap<char,usize> {
    for c in string.chars() {
        if c.is_alphabetic() {
            let mult = hash.entry(c).or_insert(0);
            *mult += 1
        }
    }

    return hash
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    /* No need to be wasteful */
    let worker_count = cmp::min(MAX_THREADS, worker_count);

    let mut strings: Vec<String> = Vec::with_capacity(worker_count);
    let mut handles = Vec::with_capacity(worker_count);

    /*
     * Move from a slice to a vector.  Note
     * that, since new threads can outlive the
     * references in the signature, we clone the
     * relevant strings outright.
     */
    for line in input.iter() {
        let string = line.to_string().to_lowercase();
        strings.push(string);
    }


    /* output channels */
    let (tx, rx) = mpsc::channel();


    let mut job: Vec<Vec<String>> = Vec::with_capacity(worker_count);
    for _ in 0..worker_count {
        job.push(Vec::new());
    }

    let mut count = 0;

    /* Assign jobs in a round-robin fashion */
    for item in strings.iter() {

        job[count].push(item.to_string());
        count = (count + 1) % worker_count;
    }

    /*
     * I want to share this vector
     * across threads
     */
    let job_arc = Arc::new(job);

    for count in 0..worker_count {

        let tx = mpsc::Sender::clone(&tx);
        let job = Arc::clone(&job_arc);

        handles.push(thread::spawn( move || {
            let mut hash = HashMap::new();

            for string in job[count].iter() {
                hash = counter(string, hash)
            }

            tx.send(hash).unwrap();
        }));
    }

    drop(tx);

    /* Join all thread handles */
    for _ in 1..worker_count {
        let handle = handles.pop().unwrap();
        handle.join().unwrap();
    }

    let mut result = HashMap::new();

    while let Ok(mut hash) = rx.recv() {

        /*
         * Aggregate all the key-value pairs into
         * one hash map.
         */
       for (k,v) in hash.drain() {
           let mult = result.entry(k).or_insert(0);
           *mult += v;
       }
    }

    return result
}
