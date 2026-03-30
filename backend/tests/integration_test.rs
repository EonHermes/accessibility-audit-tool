use accessibility_audit_backend::core::models::*;
use serde_json::json;

#[test]
fn test_issue_type_serialization() {
    let critical = IssueType::Critical;
    let serialized = serde_json::to_string(&critical).unwrap();
    assert_eq!(serialized, "\"Critical\"");

    let deserialized: IssueType = serde_json::from_str("\"Serious\"").unwrap();
    assert!(matches!(deserialized, IssueType::Serious));
}

#[test]
fn test_wcag_level_serialization() {
    let level_a = WcagLevel::A;
    let serialized = serde_json::to_string(&level_a).unwrap();
    assert_eq!(serialized, "\"A\"");

    let level_aa: WcagLevel = serde_json::from_str("\"AA\"").unwrap();
    assert!(matches!(level_aa, WcagLevel::AA));
}

#[test]
fn test_audit_summary_creation() {
    use accessibility_audit_backend::services::audit_service::calculate_summary;
    
    let issues: Vec<AccessibilityIssue> = vec![];
    let summary = calculate_summary(&issues);
    
    assert_eq!(summary.total_issues, 0);
    assert!(summary.wcag_level_a_pass);
    assert!(summary.wcag_level_aa_pass);
}

#[test]
fn test_audit_request_validation() {
    use validator::Validate;
    
    let valid_request = AuditRequest {
        url: "https://example.com".to_string(),
        project_id: None,
        check_level: Some(WcagLevel::AA),
    };
    
    assert!(valid_request.validate().is_ok());

    let invalid_request = AuditRequest {
        url: "not-a-url".to_string(),
        project_id: None,
        check_level: None,
    };
    
    assert!(invalid_request.validate().is_err());
}

#[test]
fn test_create_project_request_validation() {
    use validator::Validate;
    
    let valid_request = CreateProjectRequest {
        name: "Test Project".to_string(),
        description: Some("A test project".to_string()),
        target_url: "https://example.com".to_string(),
    };
    
    assert!(valid_request.validate().is_ok());

    let invalid_name = CreateProjectRequest {
        name: "".to_string(),
        description: None,
        target_url: "https://example.com".to_string(),
    };
    
    assert!(invalid_name.validate().is_err());

    let invalid_url = CreateProjectRequest {
        name: "Test".to_string(),
        description: None,
        target_url: "not-a-url".to_string(),
    };
    
    assert!(invalid_url.validate().is_err());
}

#[test]
fn test_page_audit_result_structure() {
    let result = PageAuditResult {
        url: "https://example.com".to_string(),
        timestamp: chrono::Utc::now(),
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
        scan_duration_ms: 1500,
    };

    assert_eq!(result.url, "https://example.com");
    assert_eq!(result.issues.len(), 0);
    assert_eq!(result.scan_duration_ms, 1500);
}

#[test]
fn test_uuid_serialization() {
    let uuid = uuid::Uuid::new_v4();
    let serialized = serde_json::to_string(&uuid).unwrap();
    
    // UUID should serialize as a quoted string
    assert!(serialized.starts_with('"'));
    assert!(serialized.ends_with('"'));
    
    // Should be able to deserialize back
    let deserialized: uuid::Uuid = serde_json::from_str(&serialized).unwrap();
    assert_eq!(uuid, deserialized);
}

#[test]
fn test_datetime_serialization() {
    let now = chrono::Utc::now();
    let serialized = serde_json::to_string(&now).unwrap();
    
    // DateTime should serialize as a quoted string (ISO 8601)
    assert!(serialized.starts_with('"'));
    assert!(serialized.ends_with('"'));
}

#[test]
fn test_issue_creation() {
    let issue = AccessibilityIssue {
        id: uuid::Uuid::new_v4(),
        type_: IssueType::Critical,
        rule_id: "image-alt".to_string(),
        rule_name: "Images must have alternate text".to_string(),
        description: "Image elements must have alt attributes".to_string(),
        help_url: "https://dequeuniversity.com/rules/axe/4.8/image-alt".to_string(),
        target: Some("img.hero-image".to_string()),
        html_snippet: Some("<img src=\"hero.jpg\">".to_string()),
        wcag_criterion: None,
        location: None,
    };

    assert_eq!(issue.rule_id, "image-alt");
    assert_eq!(issue.type_, IssueType::Critical);
    assert!(issue.target.is_some());
}
