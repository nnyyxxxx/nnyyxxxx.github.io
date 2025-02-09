use crate::utils::constants::*;
use notify::{Event, RecursiveMode, Result as NotifyResult, Watcher};
use once_cell::sync::Lazy;
use std::{
    path::Path,
    sync::{mpsc::channel, Mutex},
    time::Instant,
};

static LAST_UPDATE: Lazy<Mutex<Instant>> = Lazy::new(|| Mutex::new(Instant::now()));

fn handle_file_event(res: NotifyResult<Event>, tx: &std::sync::mpsc::Sender<()>) {
    let event = match res {
        Ok(e) => e,
        Err(_) => return,
    };

    let notify::EventKind::Modify(notify::event::ModifyKind::Data(_)) = event.kind else {
        return;
    };

    let mut last_update = LAST_UPDATE.lock().unwrap();
    let now = Instant::now();

    if now.duration_since(*last_update) < DEBOUNCE_DURATION {
        return;
    }

    println!("{}", CHANGES_APPLIED);
    let _ = tx.send(());
    *last_update = now;
}

pub fn setup_file_watcher() -> (notify::RecommendedWatcher, std::sync::mpsc::Sender<()>) {
    let (tx, _rx) = channel();
    let tx_clone = tx.clone();

    let mut watcher = notify::recommended_watcher(move |res| {
        handle_file_event(res, &tx_clone);
    })
    .unwrap();

    watcher
        .watch(Path::new(STATIC_DIR), RecursiveMode::Recursive)
        .unwrap();

    (watcher, tx)
}
