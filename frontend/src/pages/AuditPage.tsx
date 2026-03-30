import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { auditApi, projectsApi, AuditRequest } from '../utils/api';

const AuditPage: React.FC = () => {
  const navigate = useNavigate();
  const [url, setUrl] = useState('');
  const [selectedProject, setSelectedProject] = useState<string>('');
  const [projects, setProjects] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);

  React.useEffect(() => {
    loadProjects();
  }, []);

  const loadProjects = async () => {
    try {
      const response = await projectsApi.getAll();
      setProjects(response.data);
    } catch (err) {
      console.error('Failed to load projects:', err);
    }
  };

  const handleAudit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!url) {
      setError('Please enter a URL');
      return;
    }

    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const auditRequest: AuditRequest = {
        url,
        project_id: selectedProject || undefined,
      };

      const response = await auditApi.start(auditRequest);
      
      if (response.data.success && response.data.result) {
        setResult(response.data.result);
        
        // If we created a new scan for a project, navigate to it
        if (selectedProject) {
          setTimeout(() => {
            navigate(`/projects/${selectedProject}`);
          }, 2000);
        }
      } else {
        setError(response.data.error || 'Audit failed');
      }
    } catch (err: any) {
      setError(err.response?.data?.error || 'Failed to start audit. Make sure the backend is running.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="audit-page">
      <h1 style={{ marginBottom: '2rem' }}>New Accessibility Audit</h1>

      <div className="grid grid-2">
        {/* Audit Form */}
        <div className="card">
          <div className="card-header">
            <h2 className="card-title">Start Audit</h2>
          </div>
          <form onSubmit={handleAudit}>
            <div className="form-group">
              <label className="form-label">Website URL *</label>
              <input
                type="url"
                className="form-input"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                placeholder="https://example.com"
                required
              />
            </div>

            <div className="form-group">
              <label className="form-label">Save to Project</label>
              <select
                className="form-input"
                value={selectedProject}
                onChange={(e) => setSelectedProject(e.target.value)}
              >
                <option value="">-- Create New Project --</option>
                {projects.map((project) => (
                  <option key={project.id} value={project.id}>
                    {project.name} ({new URL(project.target_url).hostname})
                  </option>
                ))}
              </select>
            </div>

            {error && (
              <div className="alert alert-error" style={{ marginBottom: '1rem' }}>
                {error}
              </div>
            )}

            <button 
              type="submit" 
              className="btn btn-primary"
              disabled={loading}
              style={{ width: '100%' }}
            >
              {loading ? (
                <>
                  <span className="spinner" style={{ width: '20px', height: '20px', marginRight: '0.5rem' }}></span>
                  Running Audit...
                </>
              ) : (
                '🚀 Start Accessibility Audit'
              )}
            </button>
          </form>

          <div style={{ marginTop: '1.5rem', padding: '1rem', backgroundColor: 'var(--bg-primary)', borderRadius: '8px' }}>
            <h4 style={{ marginBottom: '0.5rem' }}>What we check:</h4>
            <ul style={{ paddingLeft: '1.5rem', color: 'var(--text-secondary)' }}>
              <li>Image alt text</li>
              <li>Color contrast ratios</li>
              <li>Keyboard navigation</li>
              <li>ARIA labels and roles</li>
              <li>Form accessibility</li>
              <li>Heading structure</li>
              <li>WCAG 2.1 Level A & AA compliance</li>
            </ul>
          </div>
        </div>

        {/* Results */}
        {result && (
          <div className="card">
            <div className="card-header">
              <h2 className="card-title">Audit Results</h2>
            </div>
            <div className="card-body">
              <div style={{ marginBottom: '1.5rem' }}>
                <a 
                  href={result.url} 
                  target="_blank" 
                  rel="noopener noreferrer"
                  style={{ color: 'var(--accent-primary)' }}
                >
                  {new URL(result.url).hostname} ↗
                </a>
              </div>

              <div className="stats-grid" style={{ gridTemplateColumns: 'repeat(2, 1fr)', marginBottom: '1.5rem' }}>
                <div className="stat-card">
                  <div className="stat-value">{result.summary.total_issues}</div>
                  <div className="stat-label">Total Issues</div>
                </div>
                <div className="stat-card">
                  <div className="stat-value" style={{ color: result.summary.critical_count > 0 ? 'var(--error)' : 'var(--success)' }}>
                    {result.summary.critical_count}
                  </div>
                  <div className="stat-label">Critical</div>
                </div>
              </div>

              <div style={{ marginBottom: '1.5rem' }}>
                <h4 style={{ marginBottom: '0.75rem' }}>WCAG Compliance Status</h4>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '0.5rem' }}>
                  <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                    <span>Level A:</span>
                    <span className={result.summary.wcag_level_a_pass ? 'compliance-pass' : 'compliance-fail'}>
                      {result.summary.wcag_level_a_pass ? '✓ Pass' : '✗ Fail'}
                    </span>
                  </div>
                  <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                    <span>Level AA:</span>
                    <span className={result.summary.wcag_level_aa_pass ? 'compliance-pass' : 'compliance-fail'}>
                      {result.summary.wcag_level_aa_pass ? '✓ Pass' : '✗ Fail'}
                    </span>
                  </div>
                  <div style={{ display: 'flex', justifyContent: 'space-between' }}>
                    <span>Level AAA:</span>
                    <span className={result.summary.wcag_level_aaa_pass ? 'compliance-pass' : 'compliance-fail'}>
                      {result.summary.wcag_level_aaa_pass ? '✓ Pass' : '✗ Fail'}
                    </span>
                  </div>
                </div>
              </div>

              <div style={{ display: 'flex', gap: '0.5rem', flexWrap: 'wrap' }}>
                {result.summary.critical_count > 0 && (
                  <span className="badge badge-critical">{result.summary.critical_count} Critical</span>
                )}
                {result.summary.serious_count > 0 && (
                  <span className="badge badge-serious">{result.summary.serious_count} Serious</span>
                )}
                {result.summary.moderate_count > 0 && (
                  <span className="badge badge-moderate">{result.summary.moderate_count} Moderate</span>
                )}
                {result.summary.minor_count > 0 && (
                  <span className="badge badge-minor">{result.summary.minor_count} Minor</span>
                )}
              </div>

              <div style={{ marginTop: '1.5rem', padding: '0.75rem', backgroundColor: 'var(--bg-primary)', borderRadius: '8px' }}>
                <small style={{ color: 'var(--text-muted)' }}>
                  Scan completed in {(result.scan_duration_ms / 1000).toFixed(2)} seconds
                </small>
              </div>

              {result.issues.length > 0 && (
                <button 
                  className="btn btn-secondary" 
                  style={{ width: '100%', marginTop: '1rem' }}
                  onClick={() => navigate(`/projects/${selectedProject}`)}
                >
                  View Full Report →
                </button>
              )}
            </div>
          </div>
        )}

        {!result && !loading && (
          <div className="card">
            <div className="empty-state">
              <div className="empty-state-icon">🔍</div>
              <div className="empty-state-title">Ready to Audit</div>
              <p>Enter a URL above to start an accessibility audit</p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default AuditPage;
