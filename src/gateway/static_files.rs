//! Static file serving for web dashboard.
//! 
//! First tries to serve from an external web directory (e.g., ../web/dist),
//! then falls back to embedded files if the external directory doesn't exist.

use axum::{
    body::Body,
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use std::path::PathBuf;
use std::sync::Arc;

/// Web directory configuration
#[derive(Clone)]
pub struct WebDirConfig {
    pub web_dir: Arc<Option<PathBuf>>,
}

impl WebDirConfig {
    pub fn new(web_dir: Option<PathBuf>) -> Self {
        Self {
            web_dir: Arc::new(web_dir),
        }
    }

    pub fn default() -> Self {
        let external_path = std::env::current_dir()
            .ok()
            .and_then(|p| p.parent().map(|parent| parent.join("web").join("dist")))
            .and_then(|p| p.canonicalize().ok())
            .filter(|p| p.exists() && p.is_dir());
        Self::new(external_path)
    }
}

/// Serve static files from `/_app/*` path
pub async fn handle_static(uri: Uri) -> impl IntoResponse {
    let path = uri.path().strip_prefix("/_app/").unwrap_or(uri.path());
    let web_dir = WebDirConfig::default().web_dir;
    serve_file(&web_dir, path).await
}

/// SPA fallback: serve index.html for any non-API, non-static GET request
pub async fn handle_spa_fallback() -> impl IntoResponse {
    let web_dir = WebDirConfig::default().web_dir;
    serve_file(&web_dir, "index.html").await
}

async fn serve_file(web_dir: &Option<PathBuf>, path: &str) -> Response {
    // Try external directory first
    if let Some(web_dir) = web_dir {
        let file_path = web_dir.join(if path.starts_with('/') {
            &path[1..]
        } else {
            path
        });

        tracing::info!("Trying to serve file: {:?} from external web_dir: {:?}", file_path, web_dir);

        if file_path.exists() {
            match tokio::fs::read(&file_path).await {
                Ok(contents) => {
                    let mime = mime_guess::from_path(&file_path)
                        .first_or_octet_stream()
                        .to_string();

                    let cache_control = if path.contains("assets/") {
                        "public, max-age=31536000, immutable".to_string()
                    } else {
                        "no-cache".to_string()
                    };

                    return (
                        StatusCode::OK,
                        [
                            (header::CONTENT_TYPE, mime),
                            (header::CACHE_CONTROL, cache_control),
                        ],
                        Body::from(contents),
                    )
                        .into_response();
                }
                Err(e) => {
                    tracing::error!("Failed to read file {:?}: {}", file_path, e);
                    // Fall through to next fallback
                }
            }
        } else {
            tracing::warn!("File not found in external directory: {:?}", file_path);
            // Fall through to next fallback
        }
    }

    // Try parent directory's web/dist
    let parent_web_dist = std::env::current_dir()
        .ok()
        .and_then(|p| p.parent().map(|parent| parent.join("web").join("dist")))
        .filter(|p| p.exists() && p.is_dir());

    if let Some(web_dir) = parent_web_dist {
        let file_path = web_dir.join(if path.starts_with('/') {
            &path[1..]
        } else {
            path
        });

        tracing::info!("Trying to serve file: {:?} from parent web_dir: {:?}", file_path, web_dir);

        if file_path.exists() {
            match tokio::fs::read(&file_path).await {
                Ok(contents) => {
                    let mime = mime_guess::from_path(&file_path)
                        .first_or_octet_stream()
                        .to_string();

                    let cache_control = if path.contains("assets/") {
                        "public, max-age=31536000, immutable".to_string()
                    } else {
                        "no-cache".to_string()
                    };

                    return (
                        StatusCode::OK,
                        [
                            (header::CONTENT_TYPE, mime),
                            (header::CACHE_CONTROL, cache_control),
                        ],
                        Body::from(contents),
                    )
                        .into_response();
                }
                Err(e) => {
                    tracing::error!("Failed to read file {:?}: {}", file_path, e);
                    // Fall through to next fallback
                }
            }
        } else {
            tracing::warn!("File not found in parent web directory: {:?}", file_path);
            // Fall through to next fallback
        }
    }

    // Try current directory's web/dist
    let current_web_dist = std::env::current_dir()
        .ok()
        .map(|p| p.join("web").join("dist"))
        .filter(|p| p.exists() && p.is_dir());

    if let Some(web_dir) = current_web_dist {
        let file_path = web_dir.join(if path.starts_with('/') {
            &path[1..]
        } else {
            path
        });

        tracing::info!("Trying to serve file: {:?} from current web_dir: {:?}", file_path, web_dir);

        if file_path.exists() {
            match tokio::fs::read(&file_path).await {
                Ok(contents) => {
                    let mime = mime_guess::from_path(&file_path)
                        .first_or_octet_stream()
                        .to_string();

                    let cache_control = if path.contains("assets/") {
                        "public, max-age=31536000, immutable".to_string()
                    } else {
                        "no-cache".to_string()
                    };

                    return (
                        StatusCode::OK,
                        [
                            (header::CONTENT_TYPE, mime),
                            (header::CACHE_CONTROL, cache_control),
                        ],
                        Body::from(contents),
                    )
                        .into_response();
                }
                Err(e) => {
                    tracing::error!("Failed to read file {:?}: {}", file_path, e);
                    // Fall through to next fallback
                }
            }
        } else {
            tracing::warn!("File not found in current web directory: {:?}", file_path);
            // Fall through to next fallback
        }
    }

    // Try global web/dist directory
    let global_web_dist = PathBuf::from("/Users/haha/.zeroclaw/web/dist")
        .canonicalize()
        .ok()
        .filter(|p| p.exists() && p.is_dir());

    if let Some(web_dir) = global_web_dist {
        let file_path = web_dir.join(if path.starts_with('/') {
            &path[1..]
        } else {
            path
        });

        tracing::info!("Trying to serve file: {:?} from global web_dir: {:?}", file_path, web_dir);

        if file_path.exists() {
            match tokio::fs::read(&file_path).await {
                Ok(contents) => {
                    let mime = mime_guess::from_path(&file_path)
                        .first_or_octet_stream()
                        .to_string();

                    let cache_control = if path.contains("assets/") {
                        "public, max-age=31536000, immutable".to_string()
                    } else {
                        "no-cache".to_string()
                    };

                    return (
                        StatusCode::OK,
                        [
                            (header::CONTENT_TYPE, mime),
                            (header::CACHE_CONTROL, cache_control),
                        ],
                        Body::from(contents),
                    )
                        .into_response();
                }
                Err(e) => {
                    tracing::error!("Failed to read file {:?}: {}", file_path, e);
                    // Fall through to next fallback
                }
            }
        } else {
            tracing::warn!("File not found in global web directory: {:?}", file_path);
            // Fall through to next fallback
        }
    }

    // If all fallbacks fail
    tracing::warn!("File not found: {}", path);
    (StatusCode::NOT_FOUND, "Not found").into_response()
}
