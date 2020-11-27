use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::RayonWorkers;
use rayon::{prelude::*, ThreadPool};

pub async fn start() -> Result<(), JsValue> {
    // Generate rayon thread-pool with the number of threads
    // given by window.navigator.hardwareConcurency
    let workers = RayonWorkers::new(None);
    // Spawn another web worker to run the multithreaded
    // part because blocking is not allowed in the main thread
    let num = workers.concurrency();
    workers.run(move || {
        (1..=num as i32).into_par_iter().for_each(|e| {
            crate::logv(&JsValue::from(format!("Hey from thread number {}!", e)));
        });
    });
    crate::alert("Hey from main thread!\nCheck the dev console for messages from other threads.");
    Ok(())
}
