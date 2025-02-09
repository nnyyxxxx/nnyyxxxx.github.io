use site::core::{app::Application, watcher::setup_file_watcher};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (_watcher, _tx) = setup_file_watcher();
    let app = Application::new();
    app.run().await
}
