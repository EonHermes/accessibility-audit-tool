use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// WCAG Level of compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WcagLevel {
    A,
    AA,
    AAA,
}

/// Type of accessibility issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    Critical,
    Serious,
    Moderate,
    Minor,
}

/// WCAG criterion reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WcagCriterion {
    pub id: String,
    pub name: String,
    pub level: WcagLevel,
    pub description: String,
}

/// Accessibility issue found during audit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityIssue {
    pub id: Uuid,
    pub type_: IssueType,
    pub rule_id: String,
    pub rule_name: String,
    pub description: String,
    pub help_url: String,
    pub target: Option<String>,
    pub html_snippet: Option<String>,
    pub wcag_criterion: Option<WcagCriterion>,
    pub location: Option<PageLocation>,
}

/// Location on a page where an issue was found
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageLocation {
    pub selector: String,
    pub xpath: Option<String>,
    pub line_number: Option<u32>,
    pub column_number: Option<u32>,
}

/// Summary of audit results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSummary {
    pub total_issues: usize,
    pub critical_count: usize,
    pub serious_count: usize,
    pub moderate_count: usize,
    pub minor_count: usize,
    pub wcag_level_a_pass: bool,
    pub wcag_level_aa_pass: bool,
    pub wcag_level_aaa_pass: bool,
}

/// Result of a single page audit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageAuditResult {
    pub url: String,
    pub timestamp: DateTime<Utc>,
    pub issues: Vec<AccessibilityIssue>,
    pub summary: AuditSummary,
    pub page_title: Option<String>,
    pub scan_duration_ms: u64,
}

/// Complete project containing multiple scans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditProject {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub target_url: String,
    pub scans: Vec<PageAuditResult>,
}

/// Request to start a new audit
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct AuditRequest {
    #[validate(url(message = "Invalid URL format"))]
    pub url: String,
    pub project_id: Option<Uuid>,
    pub check_level: Option<WcagLevel>,
}

/// Response containing audit results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditResponse {
    pub success: bool,
    pub result: Option<PageAuditResult>,
    pub error: Option<String>,
}

/// Project creation request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateProjectRequest {
    #[validate(length(min = 1, max = 200, message = "Name must be between 1 and 200 characters"))]
    pub name: String,
    pub description: Option<String>,
    #[validate(url(message = "Invalid URL format"))]
    pub target_url: String,
}
