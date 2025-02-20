use anyhow::Result;
use concurrency::AmapMetrics;
use rand::Rng;
use std::{thread, time::Duration};

#[allow(unused)]
const N: usize = 2;
#[allow(unused)]
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = AmapMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "req.page.1",
        "req.page.2",
        "req.page.3",
        "req.page.4",
    ]);

    println!("{}", metrics);

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        requests_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }
}

#[allow(unused)]
fn task_worker(idx: usize, mut metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

#[allow(unused)]
fn requests_worker(mut metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
