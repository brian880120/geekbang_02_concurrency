use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread};

const NUM_PRODUCER: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // producer
    for i in 0..NUM_PRODUCER {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    drop(tx);

    // consumer
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("consumer thread stopped");
    });

    consumer
        .join()
        .map_err(|e| anyhow!("thread join error: {:?}", e))?;

    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(std::time::Duration::from_millis(sleep_time));

        // randomly stop the producer
        if rand::random::<u8>() % 10 == 0 {
            println!("producer {} stopped", idx);
            break;
        }
    }
    Ok(())
}
