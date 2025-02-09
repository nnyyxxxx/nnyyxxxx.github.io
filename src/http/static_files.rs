use crate::utils::constants::*;
use actix_files as fs;
use actix_web::{dev::ServiceResponse, http::header};
use std::path::Path;

pub async fn create_static_file_handler(
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

pub fn configure_static_files() -> fs::Files {
    fs::Files::new("/static", STATIC_DIR)
        .prefer_utf8(true)
        .use_last_modified(true)
        .use_etag(false)
        .default_handler(create_static_file_handler)
}
