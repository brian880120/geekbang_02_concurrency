use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::{thread, time::Duration};

#[allow(unused)]
const N: usize = 2;
#[allow(unused)]
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();
    // for i in 0..100 {
    //     metrics.inc("req.page.1");
    //     metrics.inc("req.page.2");
    //     if i % 2 == 0 {
    //         metrics.inc("req.page.3");
    //     }
    // }

    // for _ in 0..27 {
    //     metrics.inc("call.thread.worker.1");
    // }

    println!("{:?}", metrics.snapshot());

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
fn task_worker(idx: usize, mut metrics: Metrics) -> Result<()> {
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
fn requests_worker(mut metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..256);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
