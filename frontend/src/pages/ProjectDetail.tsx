import React, { useState, useEffect } from 'react';
import { useParams, Link } from 'react-router-dom';
import { projectsApi, reportsApi, PageAuditResult } from '../utils/api';

const ProjectDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const [project, setProject] = useState<any>(null);
  const [scans, setScans] = useState<PageAuditResult[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedScan, setSelectedScan] = useState<PageAuditResult | null>(null);

  useEffect(() => {
    loadProjectData();
  }, [id]);

  const loadProjectData = async () => {
    if (!id) return;
    
    try {
      const projectResponse = await projectsApi.getById(id);
      setProject(projectResponse.data);
      
      const scansResponse = await reportsApi.getProjectReports(id);
      setScans(scansResponse.data);
      
      if (scansResponse.data.length > 0) {
        setSelectedScan(scansResponse.data[scansResponse.data.length - 1]);
      }
    } catch (error) {
      console.error('Failed to load project:', error);
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

  if (!project) {
    return (
      <div className="empty-state">
        <div className="empty-state-title">Project not found</div>
        <Link to="/projects" className="btn btn-primary" style={{ marginTop: '1rem' }}>
          Back to Projects
        </Link>
      </div>
    );
  }

  return (
    <div className="project-detail">
      <Link to="/projects" className="btn btn-secondary" style={{ marginBottom: '1rem' }}>
        ← Back to Projects
      </Link>

      <div className="card" style={{ marginBottom: '2rem' }}>
        <div className="card-header">
          <h1 className="card-title">{project.name}</h1>
        </div>
        <div className="card-body">
          <p style={{ marginBottom: '1rem' }}>{project.description || 'No description'}</p>
          <a 
            href={project.target_url} 
            target="_blank" 
            rel="noopener noreferrer"
            style={{ color: 'var(--accent-primary)' }}
          >
            {project.target_url} ↗
          </a>
        </div>
      </div>

      <div className="grid grid-2">
        {/* Scan History */}
        <div className="card">
          <div className="card-header">
            <h2 className="card-title">Scan History ({scans.length})</h2>
          </div>
          <div className="card-body" style={{ maxHeight: '400px', overflowY: 'auto' }}>
            {scans.length === 0 ? (
              <p style={{ color: 'var(--text-muted)' }}>No scans yet. Start an audit to see results.</p>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
                {scans.map((scan, index) => (
                  <button
                    key={index}
                    className={`btn ${selectedScan === scan ? 'btn-primary' : 'btn-secondary'}`}
                    onClick={() => setSelectedScan(scan)}
                    style={{ justifyContent: 'flex-start', textAlign: 'left' }}
                  >
                    <div>
                      <div>{new Date(scan.timestamp).toLocaleString()}</div>
                      <small style={{ opacity: 0.7 }}>
                        {scan.summary.total_issues} issues • 
                        {(scan.scan_duration_ms / 1000).toFixed(2)}s
                      </small>
                    </div>
                  </button>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* Latest Results */}
        <div className="card">
          <div className="card-header">
            <h2 className="card-title">Latest Results</h2>
          </div>
          {selectedScan ? (
            <div className="card-body">
              <div className="stats-grid" style={{ gridTemplateColumns: 'repeat(2, 1fr)', marginBottom: '1rem' }}>
                <div className="stat-card">
                  <div className="stat-value">{selectedScan.summary.total_issues}</div>
                  <div className="stat-label">Total Issues</div>
                </div>
                <div className="stat-card">
                  <div className="stat-value" style={{ color: selectedScan.summary.critical_count > 0 ? 'var(--error)' : 'var(--success)' }}>
                    {selectedScan.summary.critical_count}
                  </div>
                  <div className="stat-label">Critical</div>
                </div>
              </div>

              <div style={{ marginBottom: '1rem' }}>
                <h4 style={{ marginBottom: '0.5rem' }}>WCAG Compliance</h4>
                <div style={{ display: 'flex', gap: '1rem' }}>
                  <span className={selectedScan.summary.wcag_level_a_pass ? 'compliance-pass' : 'compliance-fail'}>
                    Level A: {selectedScan.summary.wcag_level_a_pass ? '✓ Pass' : '✗ Fail'}
                  </span>
                  <span className={selectedScan.summary.wcag_level_aa_pass ? 'compliance-pass' : 'compliance-fail'}>
                    Level AA: {selectedScan.summary.wcag_level_aa_pass ? '✓ Pass' : '✗ Fail'}
                  </span>
                  <span className={selectedScan.summary.wcag_level_aaa_pass ? 'compliance-pass' : 'compliance-fail'}>
                    Level AAA: {selectedScan.summary.wcag_level_aaa_pass ? '✓ Pass' : '✗ Fail'}
                  </span>
                </div>
              </div>

              <button className="btn btn-primary" onClick={() => window.open(selectedScan.url, '_blank')}>
                View Scanned Page ↗
              </button>
            </div>
          ) : (
            <div className="card-body">
              <p style={{ color: 'var(--text-muted)' }}>Select a scan to view details</p>
            </div>
          )}
        </div>
      </div>

      {/* Issues List */}
      {selectedScan && selectedScan.issues.length > 0 && (
        <div className="card" style={{ marginTop: '2rem' }}>
          <div className="card-header">
            <h2 className="card-title">Accessibility Issues ({selectedScan.issues.length})</h2>
          </div>
          <div className="card-body">
            <div className="table-container">
              <table>
                <thead>
                  <tr>
                    <th>Severity</th>
                    <th>Rule</th>
                    <th>Description</th>
                    <th>Action</th>
                  </tr>
                </thead>
                <tbody>
                  {selectedScan.issues.slice(0, 20).map((issue) => (
                    <tr key={issue.id}>
                      <td>
                        <span className={`badge badge-${issue.type_.toLowerCase()}`}>
                          {issue.type_}
                        </span>
                      </td>
                      <td>{issue.rule_name}</td>
                      <td>{issue.description}</td>
                      <td>
                        <a 
                          href={issue.help_url} 
                          target="_blank" 
                          rel="noopener noreferrer"
                          style={{ color: 'var(--accent-primary)' }}
                        >
                          Learn more ↗
                        </a>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
            {selectedScan.issues.length > 20 && (
              <p style={{ marginTop: '1rem', color: 'var(--text-muted)' }}>
                Showing first 20 of {selectedScan.issues.length} issues
              </p>
            )}
          </div>
        </div>
      )}
    </div>
  );
};

export default ProjectDetail;
