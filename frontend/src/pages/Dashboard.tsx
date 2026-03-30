import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { projectsApi, auditApi, PageAuditResult } from '../utils/api';

const Dashboard: React.FC = () => {
  const [projects, setProjects] = useState<any[]>([]);
  const [latestScans, setLatestScans] = useState<PageAuditResult[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadDashboardData();
  }, []);

  const loadDashboardData = async () => {
    try {
      const projectsResponse = await projectsApi.getAll();
      setProjects(projectsResponse.data);

      // Load latest scans for each project
      const scanPromises = projectsResponse.data.map((project: any) => 
        auditApi.start({ url: project.target_url, project_id: project.id })
          .then(res => res.data.result)
          .catch(() => null)
      );

      const scans = await Promise.all(scanPromises);
      setLatestScans(scans.filter((s): s is PageAuditResult => s !== null));
    } catch (error) {
      console.error('Failed to load dashboard data:', error);
    } finally {
      setLoading(false);
    }
  };

  if (loading) {
    return (
      <div className="loading">
        <div className="spinner"></div>
      </div>
    );
  }

  const totalIssues = latestScans.reduce((sum, scan) => sum + scan.summary.total_issues, 0);
  const criticalIssues = latestScans.reduce((sum, scan) => sum + scan.summary.critical_count, 0);
  const projectsCount = projects.length;
  const passingProjects = projects.filter(p => 
    p.scans.length > 0 && p.scans[p.scans.length - 1].summary.wcag_level_aa_pass
  ).length;

  return (
    <div className="dashboard">
      <h1 style={{ marginBottom: '2rem' }}>Dashboard</h1>

      <div className="stats-grid">
        <div className="stat-card">
          <div className="stat-value">{projectsCount}</div>
          <div className="stat-label">Total Projects</div>
        </div>
        <div className="stat-card">
          <div className="stat-value" style={{ color: criticalIssues > 0 ? 'var(--error)' : 'var(--success)' }}>
            {criticalIssues}
          </div>
          <div className="stat-label">Critical Issues</div>
        </div>
        <div className="stat-card">
          <div className="stat-value">{totalIssues}</div>
          <div className="stat-label">Total Issues</div>
        </div>
        <div className="stat-card">
          <div className="stat-value" style={{ color: 'var(--success)' }}>{passingProjects}</div>
          <div className="stat-label">AA Compliant</div>
        </div>
      </div>

      <div className="grid grid-2">
        <div className="card">
          <div className="card-header">
            <h2 className="card-title">Recent Projects</h2>
          </div>
          <div className="card-body">
            {projects.length === 0 ? (
              <div className="empty-state">
                <div className="empty-state-icon">📁</div>
                <div className="empty-state-title">No projects yet</div>
                <p>Create your first accessibility audit project</p>
                <Link to="/projects" className="btn btn-primary" style={{ marginTop: '1rem', display: 'inline-block' }}>
                  Create Project
                </Link>
              </div>
            ) : (
              <div className="table-container">
                <table>
                  <thead>
                    <tr>
                      <th>Name</th>
                      <th>URL</th>
                      <th>Status</th>
                      <th>Action</th>
                    </tr>
                  </thead>
                  <tbody>
                    {projects.slice(0, 5).map((project: any) => (
                      <tr key={project.id}>
                        <td>{project.name}</td>
                        <td>
                          <a href={project.target_url} target="_blank" rel="noopener noreferrer" 
                             style={{ color: 'var(--accent-primary)' }}>
                            {new URL(project.target_url).hostname}
                          </a>
                        </td>
                        <td>
                          {project.scans.length > 0 ? (
                            project.scans[project.scans.length - 1].summary.wcag_level_aa_pass ? (
                              <span className="badge badge-minor">AA Compliant</span>
                            ) : (
                              <span className="badge badge-serious">Issues Found</span>
                            )
                          ) : (
                            <span className="badge badge-moderate">No Scans</span>
                          )}
                        </td>
                        <td>
                          <Link to={`/projects/${project.id}`} className="btn btn-secondary" style={{ padding: '0.5rem 1rem' }}>
                            View
                          </Link>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>
        </div>

        <div className="card">
          <div className="card-header">
            <h2 className="card-title">Quick Actions</h2>
          </div>
          <div className="card-body" style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
            <Link to="/audit" className="btn btn-primary">
              🚀 Start New Audit
            </Link>
            <Link to="/projects" className="btn btn-secondary">
              📁 Manage Projects
            </Link>
            <a href="https://www.w3.org/WAI/WCAG21/quickref/" target="_blank" rel="noopener noreferrer" 
               className="btn btn-secondary">
              📖 WCAG Guidelines
            </a>
          </div>
        </div>
      </div>

      {latestScans.length > 0 && (
        <div className="card" style={{ marginTop: '2rem' }}>
          <div className="card-header">
            <h2 className="card-title">Latest Scan Results</h2>
          </div>
          <div className="card-body">
            <div className="table-container">
              <table>
                <thead>
                  <tr>
                    <th>URL</th>
                    <th>Issues</th>
                    <th>Critical</th>
                    <th>AA Level</th>
                    <th>Duration</th>
                  </tr>
                </thead>
                <tbody>
                  {latestScans.slice(0, 5).map((scan, index) => (
                    <tr key={index}>
                      <td>{new URL(scan.url).hostname}</td>
                      <td>{scan.summary.total_issues}</td>
                      <td>
                        <span className={scan.summary.critical_count > 0 ? 'compliance-fail' : 'compliance-pass'}>
                          {scan.summary.critical_count}
                        </span>
                      </td>
                      <td>
                        {scan.summary.wcag_level_aa_pass ? (
                          <span className="badge badge-minor">✓ Pass</span>
                        ) : (
                          <span className="badge badge-serious">✗ Fail</span>
                        )}
                      </td>
                      <td>{(scan.scan_duration_ms / 1000).toFixed(2)}s</td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Dashboard;
