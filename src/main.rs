mod constants;
mod handlers;
mod routes;

use crate::constants::*;
use actix_files as fs;
use actix_web::{dev::ServiceResponse, http::header, App, HttpServer};
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

fn setup_file_watcher() -> (notify::RecommendedWatcher, std::sync::mpsc::Sender<()>) {
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

async fn create_static_file_handler(
    req: actix_web::dev::ServiceRequest,
) -> Result<ServiceResponse, actix_web::Error> {
    let (req, _) = req.into_parts();
    let file =
        fs::NamedFile::open_async(Path::new(&format!("./{}{}", STATIC_DIR, req.path()))).await?;
    let mut res = file.into_response(&req);
    res.headers_mut().insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static(CACHE_CONTROL_VALUE),
    );
    Ok(ServiceResponse::new(req, res))
}

fn configure_static_files() -> fs::Files {
    fs::Files::new("/static", STATIC_DIR)
        .prefer_utf8(true)
        .use_last_modified(true)
        .use_etag(false)
        .default_handler(create_static_file_handler)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}", SERVER_STARTING);

    let (_watcher, _tx) = setup_file_watcher();

    HttpServer::new(|| {
        App::new()
            .service(configure_static_files())
            .service(routes::config())
            .service(fs::Files::new("/", STATIC_DIR).index_file(INDEX_FILE))
    })
    .bind(format!("{}:{}", SERVER_HOST, SERVER_PORT))?
    .run()
    .await
}
