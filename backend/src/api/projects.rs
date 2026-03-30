use axum::{
    extract::{Path, State},
    json::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::core::models::*;
use crate::core::errors::AppError;
use crate::services::storage::Storage;

pub fn routes() -> Router<Arc<Storage>> {
    Router::new()
        .route("/projects", post(create_project))
        .route("/projects", get(list_projects))
        .route("/projects/:id", get(get_project))
        .route("/projects/:id", put(update_project))
        .route("/projects/:id", delete(delete_project))
        .with_state(Arc::new(Storage::new()))
}

async fn create_project(
    State(storage): State<Arc<Storage>>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<Json<AuditProject>, AppError> {
    request.validate()?;
    
    let project = storage.create_project(request)?;
    Ok(Json(project))
}

async fn list_projects(State(storage): State<Arc<Storage>>) -> Json<Vec<AuditProject>> {
    let projects = storage.list_projects();
    Json(projects)
}

async fn get_project(
    State(storage): State<Arc<Storage>>,
    Path(id): Path<String>,
) -> Result<Json<AuditProject>, AppError> {
    let project_id = id.parse::<Uuid>()
        .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;

    storage.get_project(project_id)
        .ok_or_else(|| AppError::NotFound(format!("Project not found: {}", project_id)))
        .map(Json)
}

async fn update_project(
    State(storage): State<Arc<Storage>>,
    Path(id): Path<String>,
    Json(updates): Json<ProjectUpdatesRequest>,
) -> Result<Json<AuditProject>, AppError> {
    let project_id = id.parse::<Uuid>()
        .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;

    let project_updates = crate::services::storage::ProjectUpdates {
        name: updates.name,
        description: updates.description,
        target_url: updates.target_url,
    };

    let project = storage.update_project(project_id, project_updates)?;
    Ok(Json(project))
}

async fn delete_project(
    State(storage): State<Arc<Storage>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, AppError> {
    let project_id = id.parse::<Uuid>()
        .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;

    storage.delete_project(project_id)?;
    
    Ok(Json(json!({
        "success": true,
        "message": "Project deleted successfully"
    })))
}

#[derive(Debug, serde::Deserialize)]
struct ProjectUpdatesRequest {
    name: Option<String>,
    description: Option<Option<String>>,
    target_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_create_project() {
        let app = routes();
        
        let request = Request::builder()
            .method("POST")
            .uri("/projects")
            .body(Body::from(r#"{"name": "Test", "target_url": "https://example.com"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_project_invalid() {
        let app = routes();
        
        let request = Request::builder()
            .method("POST")
            .uri("/projects")
            .body(Body::from(r#"{"name": "", "target_url": "not-a-url"}"#))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_list_projects_empty() {
        let app = routes();
        
        let request = Request::builder()
            .method("GET")
            .uri("/projects")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_nonexistent_project() {
        let app = routes();
        let fake_id = Uuid::new_v4().to_string();
        
        let request = Request::builder()
            .method("GET")
            .uri(format!("/projects/{}", fake_id))
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
