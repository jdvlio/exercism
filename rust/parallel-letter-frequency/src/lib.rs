use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    /* Divide input slice into (almost) equally sized chunks */
    let chunk_size = &input.len() / &worker_count;

    /*
     * Adjustment if the worker count
     * does not divide the input length
     */
    if &input.len() % &worker_count != 0 {
        chunk_size += 1;
    }

    /* An iterator over chunk slices */
    let chunk_iter = input.chunks(chunk_size);

    let (tx,rx) = channel();

    for i in 1..worker_count {

        thread::spawn(|| {

            let chunk = chunk_iter.next();
            let iter = chunk.iter();

            /*
             * Keys are 'letters' and values are
             * are their corresponding multiplicity
             */

            let mut map = HashMap::new();

            for letter in iter {
                let mult = map.entry(letter).or_insert(0);
                *mult += 1;
            }

            let drain = map.drain();
            let tx = &tx.clone();
        });
    }
}
