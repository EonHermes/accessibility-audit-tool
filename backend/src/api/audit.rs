use axum::{
    extract::State,
    json::Json,
    routing::{get, post},
    Router,
};
use serde_json::json;
use std::sync::Arc;

use crate::core::models::*;
use crate::core::errors::AppError;
use crate::services::{audit_service::AuditService, storage::Storage};

pub struct AppState {
    pub audit_service: Arc<AuditService>,
    pub storage: Arc<Storage>,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/audit", post(start_audit))
        .route("/audit/status/:job_id", get(get_audit_status))
        .with_state(Arc::new(AppState {
            audit_service: Arc::new(AuditService::new().expect("Failed to create audit service")),
            storage: Arc::new(Storage::new()),
        }))
}

async fn start_audit(Json(request): Json<AuditRequest>) -> Result<Json<AuditResponse>, AppError> {
    // Validate the request
    request.validate()?;

    let state = axum::extract::State::<Arc<AppState>>::from_request(
        axum::http::Request::builder().body(()).unwrap(),
    ).await.unwrap();

    // Perform the audit
    match state.audit_service.audit_url(&request.url, request.check_level).await {
        Ok(result) => {
            // If project_id is provided, save to project
            if let Some(project_id) = request.project_id {
                if let Err(e) = state.storage.add_scan(project_id, result.clone()) {
                    tracing::warn!("Failed to save scan to project: {}", e);
                }
            }

            Ok(Json(AuditResponse {
                success: true,
                result: Some(result),
                error: None,
            }))
        }
        Err(e) => Ok(Json(AuditResponse {
            success: false,
            result: None,
            error: Some(format!("Audit failed: {}", e)),
        })),
    }
}

async fn get_audit_status(_job_id: String) -> Json<serde_json::Value> {
    // In a production system, this would check the status of an async job
    // For now, return a simple response
    Json(json!({
        "status": "not_implemented",
        "message": "Async audit jobs are not yet supported"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_audit_invalid_url() {
        let app = routes();
        
        let request = Request::builder()
            .method("POST")
            .uri("/audit")
            .body(Body::from(r#"{"url": "not-a-url"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_audit_valid_url() {
        let app = routes();
        
        let request = Request::builder()
            .method("POST")
            .uri("/audit")
            .body(Body::from(r#"{"url": "https://example.com"}"#))
            .unwrap();

        // This will actually try to run the audit, which may fail due to browser setup
        let response = app.oneshot(request).await.unwrap();
        
        // We expect either success or a browser error, not a 404/500
        assert!(response.status() == StatusCode::OK || response.status() == StatusCode::BAD_GATEWAY);
    }
}
