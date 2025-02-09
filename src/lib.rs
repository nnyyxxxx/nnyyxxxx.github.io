pub mod core {
    pub mod app;
    pub mod watcher;
}

pub mod http {
    pub mod handlers;
    pub mod routes;
    pub mod static_files;
}

pub mod utils {
    pub mod constants;
}

pub use core::app::Application;
pub use utils::constants::*;
