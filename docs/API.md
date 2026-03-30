# Accessibility Audit Tool - API Documentation

Base URL: `http://localhost:3000` (or your configured backend URL)

## Table of Contents

- [Projects](#projects)
- [Audits](#audits)
- [Reports](#reports)
- [Error Handling](#error-handling)

---

## Projects

### Create Project

**POST** `/projects`

Create a new accessibility audit project.

**Request Body:**
```json
{
  "name": "My Website Audit",
  "description": "Testing example.com for WCAG compliance",
  "target_url": "https://example.com"
}
```

**Response:** `201 Created`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "My Website Audit",
  "description": "Testing example.com for WCAG compliance",
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z",
  "target_url": "https://example.com",
  "scans": []
}
```

### List Projects

**GET** `/projects`

Get all projects.

**Response:** `200 OK`
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "My Website Audit",
    "description": "Testing example.com for WCAG compliance",
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-15T10:30:00Z",
    "target_url": "https://example.com",
    "scans": []
  }
]
```

### Get Project

**GET** `/projects/:id`

Get a specific project by ID.

**Response:** `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "My Website Audit",
  "description": "Testing example.com for WCAG compliance",
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z",
  "target_url": "https://example.com",
  "scans": [...]
}
```

### Update Project

**PUT** `/projects/:id`

Update a project.

**Request Body:**
```json
{
  "name": "Updated Name",
  "description": "New description"
}
```

**Response:** `200 OK`

### Delete Project

**DELETE** `/projects/:id`

Delete a project and all its scans.

**Response:** `200 OK`
```json
{
  "success": true,
  "message": "Project deleted successfully"
}
```

---

## Audits

### Start Audit

**POST** `/audit`

Start a new accessibility audit on a URL.

**Request Body:**
```json
{
  "url": "https://example.com",
  "project_id": "550e8400-e29b-41d4-a716-446655440000",
  "check_level": "AA"
}
```

**Fields:**
- `url` (required): The URL to audit
- `project_id` (optional): Save results to this project
- `check_level` (optional): WCAG level to check (A, AA, or AAA)

**Response:** `200 OK`
```json
{
  "success": true,
  "result": {
    "url": "https://example.com",
    "timestamp": "2024-01-15T10:35:00Z",
    "issues": [
      {
        "id": "660e8400-e29b-41d4-a716-446655440001",
        "type_": "Critical",
        "rule_id": "image-alt",
        "rule_name": "Images must have alternate text",
        "description": "Image elements must have alt attributes",
        "help_url": "https://dequeuniversity.com/rules/axe/4.8/image-alt",
        "target": "img.hero-image",
        "html_snippet": "<img src=\"hero.jpg\">"
      }
    ],
    "summary": {
      "total_issues": 5,
      "critical_count": 1,
      "serious_count": 2,
      "moderate_count": 1,
      "minor_count": 1,
      "wcag_level_a_pass": false,
      "wcag_level_aa_pass": false,
      "wcag_level_aaa_pass": false
    },
    "page_title": "Example Domain",
    "scan_duration_ms": 3500
  }
}
```

**Error Response:** `400 Bad Request`
```json
{
  "success": false,
  "result": null,
  "error": "Invalid URL format"
}
```

---

## Reports

### Get Project Reports

**GET** `/projects/:id/reports`

Get all scan results for a project.

**Response:** `200 OK`
```json
[
  {
    "url": "https://example.com",
    "timestamp": "2024-01-15T10:35:00Z",
    "issues": [...],
    "summary": {...},
    "page_title": "Example Domain",
    "scan_duration_ms": 3500
  }
]
```

### Get Latest Report

**GET** `/projects/:id/reports/latest`

Get the most recent scan result for a project.

**Response:** `200 OK`
```json
{
  "url": "https://example.com",
  "timestamp": "2024-01-15T10:35:00Z",
  "issues": [...],
  "summary": {...},
  "page_title": "Example Domain",
  "scan_duration_ms": 3500
}
```

### Compare Scans

**GET** `/projects/:id/compare`

Compare all scans for a project to track progress over time.

**Response:** `200 OK`
```json
{
  "project_id": "550e8400-e29b-41d4-a716-446655440000",
  "total_scans": 3,
  "latest_scan": {...},
  "previous_scan": {...},
  "issue_trend": {
    "total_change": -2,
    "critical_change": -1,
    "serious_change": 0
  },
  "compliance_history": [
    {
      "timestamp": "2024-01-15T10:30:00Z",
      "level_a_pass": false,
      "level_aa_pass": false,
      "level_aaa_pass": false,
      "total_issues": 7
    },
    {
      "timestamp": "2024-01-15T11:00:00Z",
      "level_a_pass": true,
      "level_aa_pass": false,
      "level_aaa_pass": false,
      "total_issues": 5
    }
  ]
}
```

---

## Error Handling

All errors follow this format:

**Error Response:**
```json
{
  "error": "Description of the error",
  "type": "error_type"
}
```

### Error Types

- `validation_error`: Invalid request parameters (400)
- `browser_error`: Browser automation failed (400)
- `network_error`: Network/request failure (502)
- `not_found`: Resource not found (404)
- `internal_error`: Server error (500)

### HTTP Status Codes

| Code | Meaning |
|------|---------|
| 200 | Success |
| 201 | Created |
| 400 | Bad Request - Invalid input |
| 404 | Not Found - Resource doesn't exist |
| 500 | Internal Server Error |
| 502 | Bad Gateway - Backend service error |

---

## Rate Limiting

Currently, there are no rate limits. In production, consider implementing:
- Per-user rate limiting
- Request throttling for expensive operations
- Queue system for concurrent audits

---

## Authentication

This is a local development tool and does not include authentication by default. For production deployment, implement:
- API key authentication
- OAuth 2.0 integration
- JWT tokens

---

## Examples

### cURL Examples

```bash
# Create project
curl -X POST http://localhost:3000/projects \
  -H "Content-Type: application/json" \
  -d '{"name":"Test","target_url":"https://example.com"}'

# Start audit
curl -X POST http://localhost:3000/audit \
  -H "Content-Type: application/json" \
  -d '{"url":"https://example.com"}'

# Get all projects
curl http://localhost:3000/projects
```

### JavaScript Example

```javascript
const API_URL = 'http://localhost:3000';

// Create project
async function createProject(name, url) {
  const response = await fetch(`${API_URL}/projects`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ name, target_url: url })
  });
  return response.json();
}

// Start audit
async function startAudit(url, projectId) {
  const response = await fetch(`${API_URL}/audit`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ url, project_id: projectId })
  });
  return response.json();
}

// Usage
const project = await createProject('My Site', 'https://example.com');
const result = await startAudit('https://example.com', project.id);
console.log(`Found ${result.result.summary.total_issues} issues`);
```
