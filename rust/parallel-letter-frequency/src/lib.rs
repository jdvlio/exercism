
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;


pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    /* 'n Dom vraag kry 'n dom antwoord ^^ */
    if input.is_empty() || worker_count <= 0 {
        panic!("bsdchad's disappointment");
    }

    /* Spawn a channel */
    let (tx,rx) = mpsc::channel();

    let slice = input.concat();

    /* Divide input slice into (almost) equally sized chunks */
    let mut chunk_size = slice.len() / &worker_count;
    let rem = slice.len() % &worker_count;

    /*
     * Adjustment if the worker count
     * does not divide the slice length
     */
    if 0 < rem {
        chunk_size +=1
    }

    for i in 1..worker_count {


        thread::spawn( move || {

            let mut slice_mut = match i {
                1 => slice.clone(),
                _  => right.to_string(),
            };

            /* Split string slices in two */
            let (left,right) = slice.split_at(chunk_size.min(slice.len()));

            /*
             * Keys are 'letters' and values are
             * are their corresponding multiplicity
             */
            let mut map = HashMap::new();

            /* construct the entries of the hash map */
            for character in left.chars() {
                let mult = map.entry(character).or_insert(0);
                *mult += 1;
            }

            /* clone the channel transmitter */
            // let tx = mpsc::Sender::clone(&tx);
            let worker_tx = &tx.clone();

            let drain = map.drain();

            for msg in drain {
                worker_tx.send(msg);
            }

        });
    }

    drop(tx);

    let mut result = HashMap::new();

    for (k,v) in rx {
        let mult = result.entry(k).or_insert(0);
        *mult += v;
    }

    return result;
}
