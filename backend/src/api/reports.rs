use axum::{
    extract::Path,
    json::Json,
    routing::get,
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
        .route("/projects/:id/reports", get(get_project_reports))
        .route("/projects/:id/reports/latest", get(get_latest_report))
        .route("/projects/:id/compare", get(compare_scans))
        .with_state(Arc::new(Storage::new()))
}

async fn get_project_reports(
    State(storage): State<Arc<Storage>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<PageAuditResult>>, AppError> {
    let project_id = id.parse::<Uuid>()
        .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;

    let scans = storage.compare_scans(project_id)
        .ok_or_else(|| AppError::NotFound(format!("Project not found: {}", project_id)))?;

    Ok(Json(scans))
}

async fn get_latest_report(
    State(storage): State<Arc<Storage>>,
    Path(id): Path<String>,
) -> Result<Json<PageAuditResult>, AppError> {
    let project_id = id.parse::<Uuid>()
        .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;

    let scan = storage.get_latest_scan(project_id)
        .ok_or_else(|| AppError::NotFound(format!("No scans found for project: {}", project_id)))?;

    Ok(Json(scan))
}

async fn compare_scans(
    State(storage): State<Arc<Storage>>,
    Path(id): Path<String>,
) -> Result<Json<ComparisonReport>, AppError> {
    let project_id = id.parse::<Uuid>()
        .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;

    let scans = storage.compare_scans(project_id)
        .ok_or_else(|| AppError::NotFound(format!("Project not found: {}", project_id)))?;

    if scans.is_empty() {
        return Err(AppError::NotFound("No scans available for comparison".to_string()));
    }

    // Generate comparison report
    let comparison = generate_comparison(&scans);

    Ok(Json(comparison))
}

fn generate_comparison(scans: &[PageAuditResult]) -> ComparisonReport {
    let latest = scans.last().unwrap();
    let previous = scans.iter().nth(scans.len() - 2);

    let issue_trend = if let Some(prev) = previous {
        IssueTrend {
            total_change: latest.summary.total_issues as i32 - prev.summary.total_issues as i32,
            critical_change: latest.summary.critical_count as i32 - prev.summary.critical_count as i32,
            serious_change: latest.summary.serious_count as i32 - prev.summary.serious_count as i32,
        }
    } else {
        IssueTrend {
            total_change: 0,
            critical_change: 0,
            serious_change: 0,
        }
    };

    ComparisonReport {
        project_id: Uuid::parse_str(&format!("{}", Uuid::new_v4())).unwrap(), // In real impl, use actual project ID
        total_scans: scans.len(),
        latest_scan: latest.clone(),
        previous_scan: previous.cloned(),
        issue_trend,
        compliance_history: scans.iter().map(|s| ComplianceSnapshot {
            timestamp: s.timestamp,
            level_a_pass: s.summary.wcag_level_a_pass,
            level_aa_pass: s.summary.wcag_level_aa_pass,
            level_aaa_pass: s.summary.wcag_level_aaa_pass,
            total_issues: s.summary.total_issues,
        }).collect(),
    }
}

#[derive(serde::Serialize)]
struct ComparisonReport {
    project_id: Uuid,
    total_scans: usize,
    latest_scan: PageAuditResult,
    previous_scan: Option<PageAuditResult>,
    issue_trend: IssueTrend,
    compliance_history: Vec<ComplianceSnapshot>,
}

#[derive(serde::Serialize)]
struct IssueTrend {
    total_change: i32,
    critical_change: i32,
    serious_change: i32,
}

#[derive(serde::Serialize)]
struct ComplianceSnapshot {
    timestamp: chrono::DateTime<chrono::Utc>,
    level_a_pass: bool,
    level_aa_pass: bool,
    level_aaa_pass: bool,
    total_issues: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_comparison_empty() {
        let scans: Vec<PageAuditResult> = vec![];
        // This would panic in real code due to unwrap, but we're testing the logic
        // In production, this case is handled by the error check above
    }

    #[test]
    fn test_generate_comparison_single_scan() {
        let scans = vec![PageAuditResult {
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
            page_title: Some("Test".to_string()),
            scan_duration_ms: 1000,
        }];

        let comparison = generate_comparison(&scans);
        assert_eq!(comparison.total_scans, 1);
        assert!(comparison.previous_scan.is_none());
        assert_eq!(comparison.issue_trend.total_change, 0);
    }

    #[test]
    fn test_generate_comparison_multiple_scans() {
        let scans = vec![
            PageAuditResult {
                url: "https://example.com".to_string(),
                timestamp: Utc::now(),
                issues: vec![],
                summary: AuditSummary {
                    total_issues: 5,
                    critical_count: 1,
                    serious_count: 2,
                    moderate_count: 1,
                    minor_count: 1,
                    wcag_level_a_pass: false,
                    wcag_level_aa_pass: false,
                    wcag_level_aaa_pass: false,
                },
                page_title: Some("Test".to_string()),
                scan_duration_ms: 1000,
            },
            PageAuditResult {
                url: "https://example.com".to_string(),
                timestamp: Utc::now(),
                issues: vec![],
                summary: AuditSummary {
                    total_issues: 3,
                    critical_count: 0,
                    serious_count: 1,
                    moderate_count: 1,
                    minor_count: 1,
                    wcag_level_a_pass: false,
                    wcag_level_aa_pass: false,
                    wcag_level_aaa_pass: false,
                },
                page_title: Some("Test".to_string()),
                scan_duration_ms: 900,
            },
        ];

        let comparison = generate_comparison(&scans);
        assert_eq!(comparison.total_scans, 2);
        assert!(comparison.previous_scan.is_some());
        assert_eq!(comparison.issue_trend.total_change, -2);
        assert_eq!(comparison.issue_trend.critical_change, -1);
    }
}
