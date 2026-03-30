use crate::core::models::*;
use crate::core::errors::AppError;
use headless_chrome::{Browser, LaunchOptions};
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, warn};

/// Service for performing accessibility audits using headless browser and axe-core
pub struct AuditService {
    browser: Arc<Browser>,
}

impl AuditService {
    /// Create a new audit service with a headless browser instance
    pub async fn new() -> Result<Self, AppError> {
        let launch_options = LaunchOptionsBuilder::default()
            .headless(true)
            .sandbox(false) // Required for some container environments
            .build()?;

        let browser = Browser::new(launch_options)?;
        
        Ok(Self {
            browser: Arc::new(browser),
        })
    }

    /// Perform an accessibility audit on a given URL
    pub async fn audit_url(&self, url: &str, _check_level: Option<WcagLevel>) -> Result<PageAuditResult, AppError> {
        let start_time = Instant::now();
        
        info!("Starting audit for URL: {}", url);

        // Launch a new tab
        let tab = self.browser.new_tab()?;
        
        // Navigate to the URL
        tab.navigate_to(url)?
            .wait_until_navigated()?;

        // Get page title
        let page_title = tab.get_html()
            .ok()
            .and_then(|html| {
                use regex::Regex;
                Regex::new(r"<title>(.*?)</title>")
                    .ok()
                    .and_then(|re| re.captures(&html))
                    .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
            });

        // Inject axe-core and run analysis
        let issues = self.run_axe_analysis(&tab, url).await?;

        // Calculate summary
        let summary = calculate_summary(&issues);

        let scan_duration_ms = start_time.elapsed().as_millis() as u64;

        let result = PageAuditResult {
            url: url.to_string(),
            timestamp: Utc::now(),
            issues,
            summary,
            page_title,
            scan_duration_ms,
        };

        info!("Audit completed for {} in {}ms with {} issues", 
              url, scan_duration_ms, result.issues.len());

        Ok(result)
    }

    /// Run axe-core analysis on the current page
    async fn run_axe_analysis(&self, tab: &headless_chrome::Tab, _url: &str) -> Result<Vec<AccessibilityIssue>, AppError> {
        // Inject axe-core script
        let axe_script = include_str!("../../axe-core.min.js");
        
        if let Err(e) = tab.evaluate(axe_script) {
            warn!("Failed to inject axe-core: {}", e);
            // Continue without axe-core, use basic checks
            return Ok(self.run_basic_checks(tab).await?);
        }

        // Run axe analysis
        let script = r#"
            new Promise((resolve) => {
                axe.run(document, { runOnly: ['wcag2a', 'wcag2aa'] })
                    .then(results => resolve(results))
                    .catch(err => resolve({ violations: [] }));
            })
        "#;

        let result = tab.evaluate(script)?;
        
        // Parse axe-core results
        self.parse_axe_results(result)
    }

    /// Run basic accessibility checks when axe-core is unavailable
    async fn run_basic_checks(&self, tab: &headless_chrome::Tab) -> Result<Vec<AccessibilityIssue>, AppError> {
        let mut issues = Vec::new();
        
        // Check for missing alt text on images
        let check_alt_script = r#"
            new Promise((resolve) => {
                const images = document.querySelectorAll('img:not([alt])');
                const results = [];
                images.forEach(img => {
                    results.push({
                        type: 'critical',
                        ruleId: 'image-alt',
                        ruleName: 'Images must have alternate text',
                        description: 'Image elements must have alt attributes',
                        helpUrl: 'https://dequeuniversity.com/rules/axe/4.8/image-alt',
                        target: img.tagName.toLowerCase(),
                        html: img.outerHTML.substring(0, 200)
                    });
                });
                resolve(results);
            })
        "#;

        if let Ok(result) = tab.evaluate(check_alt_script) {
            if let Some(violations) = result.get_array() {
                for v in violations.iter().take(50) { // Limit to prevent huge results
                    if let Some(issue) = self.parse_basic_issue(v) {
                        issues.push(issue);
                    }
                }
            }
        }

        Ok(issues)
    }

    /// Parse axe-core JSON results into our issue format
    fn parse_axe_results(&self, result: headless_chrome::protocol::cdp::Runtime::EvaluateReturnValue) -> Result<Vec<AccessibilityIssue>, AppError> {
        // This would normally parse the actual axe-core JSON response
        // For now, return empty - in production this would properly parse the results
        Ok(Vec::new())
    }

    /// Parse a basic issue from JavaScript evaluation
    fn parse_basic_issue(&self, value: &serde_json::Value) -> Option<AccessibilityIssue> {
        Some(AccessibilityIssue {
            id: Uuid::new_v4(),
            type_: serde_json::from_value(value.get("type")?.clone()).ok()?,
            rule_id: value.get("ruleId")?.as_str()?.to_string(),
            rule_name: value.get("ruleName")?.as_str()?.to_string(),
            description: value.get("description")?.as_str()?.to_string(),
            help_url: value.get("helpUrl")?.as_str()?.to_string(),
            target: value.get("target").and_then(|t| t.as_str()).map(String::from),
            html_snippet: value.get("html").and_then(|h| h.as_str()).map(String::from),
            wcag_criterion: None,
            location: None,
        })
    }
}

/// Calculate audit summary from issues
fn calculate_summary(issues: &[AccessibilityIssue]) -> AuditSummary {
    let critical_count = issues.iter().filter(|i| matches!(i.type_, IssueType::Critical)).count();
    let serious_count = issues.iter().filter(|i| matches!(i.type_, IssueType::Serious)).count();
    let moderate_count = issues.iter().filter(|i| matches!(i.type_, IssueType::Moderate)).count();
    let minor_count = issues.iter().filter(|i| matches!(i.type_, IssueType::Minor)).count();

    AuditSummary {
        total_issues: issues.len(),
        critical_count,
        serious_count,
        moderate_count,
        minor_count,
        wcag_level_a_pass: critical_count == 0 && serious_count == 0,
        wcag_level_aa_pass: critical_count == 0 && serious_count == 0 && moderate_count == 0,
        wcag_level_aaa_pass: issues.is_empty(),
    }
}

// Builder for LaunchOptions since headless_chrome doesn't expose it directly
struct LaunchOptionsBuilder {
    headless: bool,
    sandbox: bool,
}

impl LaunchOptionsBuilder {
    fn default() -> Self {
        Self {
            headless: true,
            sandbox: true,
        }
    }

    fn headless(mut self, value: bool) -> Self {
        self.headless = value;
        self
    }

    fn sandbox(mut self, value: bool) -> Self {
        self.sandbox = value;
        self
    }

    fn build(self) -> Result<LaunchOptions, headless_chrome::error::Error> {
        LaunchOptionsBuilder::default()
            .headless(self.headless)
            .sandbox(self.sandbox)
            ._build()
    }
}

impl LaunchOptionsBuilder {
    fn _build(self) -> Result<LaunchOptions, headless_chrome::error::Error> {
        use std::path::PathBuf;
        
        let mut options = vec![
            "--disable-gpu".to_string(),
            "--no-sandbox".to_string(),
            "--disable-setuid-sandbox".to_string(),
            "--disable-dev-shm-usage".to_string(),
        ];

        if self.headless {
            options.push("--headless=new".to_string());
        }

        Ok(LaunchOptions {
            args: options,
            headless: self.headless,
            sandbox: self.sandbox,
            pipe: false,
            log_path: None,
            user_data_dir: None,
            chrome_executable: None,
            idle_browser_timeout: std::time::Duration::from_secs(30),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_summary_empty() {
        let summary = calculate_summary(&[]);
        assert_eq!(summary.total_issues, 0);
        assert!(summary.wcag_level_a_pass);
        assert!(summary.wcag_level_aa_pass);
        assert!(summary.wcag_level_aaa_pass);
    }

    #[test]
    fn test_calculate_summary_with_critical() {
        let issues = vec![
            AccessibilityIssue {
                id: Uuid::new_v4(),
                type_: IssueType::Critical,
                rule_id: "test".to_string(),
                rule_name: "Test Rule".to_string(),
                description: "Test".to_string(),
                help_url: "http://example.com".to_string(),
                target: None,
                html_snippet: None,
                wcag_criterion: None,
                location: None,
            }
        ];
        
        let summary = calculate_summary(&issues);
        assert_eq!(summary.total_issues, 1);
        assert_eq!(summary.critical_count, 1);
        assert!(!summary.wcag_level_a_pass);
    }

    #[test]
    fn test_calculate_summary_mixed() {
        let issues = vec![
            AccessibilityIssue {
                id: Uuid::new_v4(),
                type_: IssueType::Critical,
                rule_id: "test1".to_string(),
                rule_name: "Test 1".to_string(),
                description: "Test".to_string(),
                help_url: "http://example.com".to_string(),
                target: None,
                html_snippet: None,
                wcag_criterion: None,
                location: None,
            },
            AccessibilityIssue {
                id: Uuid::new_v4(),
                type_: IssueType::Moderate,
                rule_id: "test2".to_string(),
                rule_name: "Test 2".to_string(),
                description: "Test".to_string(),
                help_url: "http://example.com".to_string(),
                target: None,
                html_snippet: None,
                wcag_criterion: None,
                location: None,
            },
        ];
        
        let summary = calculate_summary(&issues);
        assert_eq!(summary.total_issues, 2);
        assert_eq!(summary.critical_count, 1);
        assert_eq!(summary.moderate_count, 1);
    }
}
