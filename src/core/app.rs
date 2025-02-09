use crate::{
    http::{routes, static_files},
    utils::constants::*,
};
use actix_files as fs;
use actix_web::{App as ActixApp, HttpServer};

pub struct Application {
    host: String,
    port: u16,
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl Application {
    pub fn new() -> Self {
        Self {
            host: SERVER_HOST.to_string(),
            port: SERVER_PORT,
        }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        println!("{}", SERVER_STARTING);

        HttpServer::new(|| {
            ActixApp::new()
                .service(static_files::configure_static_files())
                .service(routes::config())
                .service(fs::Files::new("/", STATIC_DIR).index_file(INDEX_FILE))
        })
        .bind(format!("{}:{}", self.host, self.port))?
        .run()
        .await
    }
}
