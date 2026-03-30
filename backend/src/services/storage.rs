use crate::core::models::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// In-memory storage for audit projects
/// In production, this would be replaced with a database
pub struct Storage {
    projects: Arc<RwLock<HashMap<Uuid, AuditProject>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            projects: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new project
    pub fn create_project(&self, request: CreateProjectRequest) -> Result<AuditProject, String> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let project = AuditProject {
            id,
            name: request.name,
            description: request.description,
            created_at: now,
            updated_at: now,
            target_url: request.target_url,
            scans: Vec::new(),
        };

        self.projects.write().unwrap().insert(id, project.clone());
        
        Ok(project)
    }

    /// Get a project by ID
    pub fn get_project(&self, id: Uuid) -> Option<AuditProject> {
        self.projects.read().unwrap().get(&id).cloned()
    }

    /// List all projects
    pub fn list_projects(&self) -> Vec<AuditProject> {
        self.projects.read().unwrap().values().cloned().collect()
    }

    /// Update a project
    pub fn update_project(&self, id: Uuid, updates: ProjectUpdates) -> Result<AuditProject, String> {
        let mut projects = self.projects.write().unwrap();
        
        if let Some(project) = projects.get_mut(&id) {
            if let Some(name) = updates.name {
                project.name = name;
            }
            if let Some(description) = updates.description {
                project.description = description;
            }
            if let Some(target_url) = updates.target_url {
                project.target_url = target_url;
            }
            project.updated_at = Utc::now();
            
            Ok(project.clone())
        } else {
            Err(format!("Project not found: {}", id))
        }
    }

    /// Delete a project
    pub fn delete_project(&self, id: Uuid) -> Result<(), String> {
        let mut projects = self.projects.write().unwrap();
        
        if projects.remove(&id).is_some() {
            Ok(())
        } else {
            Err(format!("Project not found: {}", id))
        }
    }

    /// Add a scan result to a project
    pub fn add_scan(&self, project_id: Uuid, scan: PageAuditResult) -> Result<AuditProject, String> {
        let mut projects = self.projects.write().unwrap();
        
        if let Some(project) = projects.get_mut(&project_id) {
            project.scans.push(scan);
            project.updated_at = Utc::now();
            
            Ok(project.clone())
        } else {
            Err(format!("Project not found: {}", project_id))
        }
    }

    /// Get the latest scan for a project
    pub fn get_latest_scan(&self, project_id: Uuid) -> Option<PageAuditResult> {
        self.projects.read().unwrap()
            .get(&project_id)
            .and_then(|p| p.scans.last().cloned())
    }

    /// Compare scans over time for a project
    pub fn compare_scans(&self, project_id: Uuid) -> Option<Vec<PageAuditResult>> {
        self.projects.read().unwrap()
            .get(&project_id)
            .map(|p| p.scans.clone())
    }
}

/// Updates that can be applied to a project
pub struct ProjectUpdates {
    pub name: Option<String>,
    pub description: Option<Option<String>>,
    pub target_url: Option<String>,
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_get_project() {
        let storage = Storage::new();
        
        let request = CreateProjectRequest {
            name: "Test Project".to_string(),
            description: Some("A test project".to_string()),
            target_url: "https://example.com".to_string(),
        };

        let project = storage.create_project(request).unwrap();
        assert_eq!(project.name, "Test Project");
        
        let retrieved = storage.get_project(project.id).unwrap();
        assert_eq!(retrieved.name, "Test Project");
    }

    #[test]
    fn test_list_projects() {
        let storage = Storage::new();
        
        storage.create_project(CreateProjectRequest {
            name: "Project 1".to_string(),
            description: None,
            target_url: "https://example.com".to_string(),
        }).unwrap();

        storage.create_project(CreateProjectRequest {
            name: "Project 2".to_string(),
            description: None,
            target_url: "https://test.com".to_string(),
        }).unwrap();

        let projects = storage.list_projects();
        assert_eq!(projects.len(), 2);
    }

    #[test]
    fn test_add_scan() {
        let storage = Storage::new();
        
        let project = storage.create_project(CreateProjectRequest {
            name: "Test Project".to_string(),
            description: None,
            target_url: "https://example.com".to_string(),
        }).unwrap();

        let scan = PageAuditResult {
            url: "https://example.com".to_string(),
            timestamp: Utc::now(),
            issues: vec![],
            summary: AuditSummary {
                total_issues: 0,
                critical_count: 0,
                serious_count: 0,
                moderate_count: 0,
                minor_count: 0,
                wcag_level_a_pass: true,
                wcag_level_aa_pass: true,
                wcag_level_aaa_pass: true,
            },
            page_title: Some("Test Page".to_string()),
            scan_duration_ms: 1000,
        };

        let updated = storage.add_scan(project.id, scan).unwrap();
        assert_eq!(updated.scans.len(), 1);
    }

    #[test]
    fn test_delete_project() {
        let storage = Storage::new();
        
        let project = storage.create_project(CreateProjectRequest {
            name: "Test Project".to_string(),
            description: None,
            target_url: "https://example.com".to_string(),
        }).unwrap();

        assert!(storage.get_project(project.id).is_some());
        
        storage.delete_project(project.id).unwrap();
        
        assert!(storage.get_project(project.id).is_none());
    }

    #[test]
    fn test_nonexistent_project() {
        let storage = Storage::new();
        let fake_id = Uuid::new_v4();
        
        assert!(storage.get_project(fake_id).is_none());
        assert!(storage.delete_project(fake_id).is_err());
    }
}
