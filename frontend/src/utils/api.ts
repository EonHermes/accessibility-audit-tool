import axios from 'axios';

const API_BASE = process.env.REACT_APP_API_URL || 'http://localhost:3000';

const api = axios.create({
  baseURL: API_BASE,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Types
export interface AccessibilityIssue {
  id: string;
  type_: 'Critical' | 'Serious' | 'Moderate' | 'Minor';
  rule_id: string;
  rule_name: string;
  description: string;
  help_url: string;
  target?: string;
  html_snippet?: string;
}

export interface AuditSummary {
  total_issues: number;
  critical_count: number;
  serious_count: number;
  moderate_count: number;
  minor_count: number;
  wcag_level_a_pass: boolean;
  wcag_level_aa_pass: boolean;
  wcag_level_aaa_pass: boolean;
}

export interface PageAuditResult {
  url: string;
  timestamp: string;
  issues: AccessibilityIssue[];
  summary: AuditSummary;
  page_title?: string;
  scan_duration_ms: number;
}

export interface AuditProject {
  id: string;
  name: string;
  description?: string;
  created_at: string;
  updated_at: string;
  target_url: string;
  scans: PageAuditResult[];
}

export interface CreateProjectRequest {
  name: string;
  description?: string;
  target_url: string;
}

export interface AuditRequest {
  url: string;
  project_id?: string;
  check_level?: 'A' | 'AA' | 'AAA';
}

// Project API
export const projectsApi = {
  getAll: () => api.get<AuditProject[]>('/projects'),
  
  getById: (id: string) => api.get<AuditProject>(`/projects/${id}`),
  
  create: (data: CreateProjectRequest) => api.post<AuditProject>('/projects', data),
  
  update: (id: string, data: Partial<CreateProjectRequest>) => 
    api.put<AuditProject>(`/projects/${id}`, data),
  
  delete: (id: string) => api.delete(`/projects/${id}`),
};

// Audit API
export const auditApi = {
  start: (data: AuditRequest) => api.post<{ success: boolean; result?: PageAuditResult; error?: string }>('/audit', data),
};

// Reports API
export const reportsApi = {
  getProjectReports: (id: string) => api.get<PageAuditResult[]>(`/projects/${id}/reports`),
  
  getLatestReport: (id: string) => api.get<PageAuditResult>(`/projects/${id}/reports/latest`),
  
  compareScans: (id: string) => api.get(`/projects/${id}/compare`),
};

export default api;
