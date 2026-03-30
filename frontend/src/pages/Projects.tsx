import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { projectsApi, CreateProjectRequest } from '../utils/api';

const Projects: React.FC = () => {
  const [projects, setProjects] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [newProject, setNewProject] = useState<CreateProjectRequest>({
    name: '',
    description: '',
    target_url: '',
  });

  useEffect(() => {
    loadProjects();
  }, []);

  const loadProjects = async () => {
    try {
      const response = await projectsApi.getAll();
      setProjects(response.data);
    } catch (error) {
      console.error('Failed to load projects:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateProject = async (e: React.FormEvent) => {
    e.preventDefault();
    
    try {
      await projectsApi.create(newProject);
      setShowCreateModal(false);
      setNewProject({ name: '', description: '', target_url: '' });
      loadProjects();
    } catch (error) {
      console.error('Failed to create project:', error);
      alert('Failed to create project. Please check the URL and try again.');
    }
  };

  const handleDeleteProject = async (id: string) => {
    if (!confirm('Are you sure you want to delete this project?')) return;
    
    try {
      await projectsApi.delete(id);
      loadProjects();
    } catch (error) {
      console.error('Failed to delete project:', error);
    }
  };

  if (loading) {
    return (
      <div className="loading">
        <div className="spinner"></div>
      </div>
    );
  }

  return (
    <div className="projects-page">
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '2rem' }}>
        <h1>Projects</h1>
        <button 
          className="btn btn-primary" 
          onClick={() => setShowCreateModal(true)}
        >
          + New Project
        </button>
      </div>

      {projects.length === 0 ? (
        <div className="empty-state">
          <div className="empty-state-icon">📁</div>
          <div className="empty-state-title">No projects yet</div>
          <p>Create your first accessibility audit project to get started</p>
          <button 
            className="btn btn-primary" 
            onClick={() => setShowCreateModal(true)}
            style={{ marginTop: '1rem' }}
          >
            Create Project
          </button>
        </div>
      ) : (
        <div className="grid grid-3">
          {projects.map((project) => (
            <div key={project.id} className="card">
              <div className="card-header">
                <h3 className="card-title">{project.name}</h3>
              </div>
              <div className="card-body">
                <p style={{ marginBottom: '1rem', color: 'var(--text-muted)' }}>
                  {project.description || 'No description'}
                </p>
                <a 
                  href={project.target_url} 
                  target="_blank" 
                  rel="noopener noreferrer"
                  style={{ color: 'var(--accent-primary)', display: 'block', marginBottom: '1rem' }}
                >
                  {new URL(project.target_url).hostname} ↗
                </a>
                
                <div style={{ marginBottom: '1rem' }}>
                  <small style={{ color: 'var(--text-muted)' }}>
                    Scans: {project.scans?.length || 0} | 
                    Created: {new Date(project.created_at).toLocaleDateString()}
                  </small>
                </div>

                {project.scans && project.scans.length > 0 && (
                  <div style={{ marginBottom: '1rem' }}>
                    {project.scans[project.scans.length - 1].summary.wcag_level_aa_pass ? (
                      <span className="badge badge-minor">✓ AA Compliant</span>
                    ) : (
                      <span className="badge badge-serious">
                        ⚠ {project.scans[project.scans.length - 1].summary.total_issues} issues
                      </span>
                    )}
                  </div>
                )}

                <div style={{ display: 'flex', gap: '0.5rem' }}>
                  <Link 
                    to={`/projects/${project.id}`} 
                    className="btn btn-secondary"
                    style={{ flex: 1, textAlign: 'center' }}
                  >
                    View Details
                  </Link>
                  <button 
                    className="btn btn-danger"
                    onClick={() => handleDeleteProject(project.id)}
                  >
                    Delete
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Create Project Modal */}
      {showCreateModal && (
        <div style={{
          position: 'fixed',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          backgroundColor: 'rgba(0, 0, 0, 0.7)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          zIndex: 1000,
        }}>
          <div className="card" style={{ width: '500px', maxWidth: '90%' }}>
            <div className="card-header">
              <h2 className="card-title">Create New Project</h2>
            </div>
            <form onSubmit={handleCreateProject}>
              <div className="form-group">
                <label className="form-label">Project Name *</label>
                <input
                  type="text"
                  className="form-input"
                  value={newProject.name}
                  onChange={(e) => setNewProject({ ...newProject, name: e.target.value })}
                  required
                  placeholder="My Website Audit"
                />
              </div>

              <div className="form-group">
                <label className="form-label">Target URL *</label>
                <input
                  type="url"
                  className="form-input"
                  value={newProject.target_url}
                  onChange={(e) => setNewProject({ ...newProject, target_url: e.target.value })}
                  required
                  placeholder="https://example.com"
                />
              </div>

              <div className="form-group">
                <label className="form-label">Description</label>
                <textarea
                  className="form-textarea"
                  value={newProject.description || ''}
                  onChange={(e) => setNewProject({ ...newProject, description: e.target.value })}
                  rows={3}
                  placeholder="Optional description..."
                />
              </div>

              <div style={{ display: 'flex', gap: '1rem', justifyContent: 'flex-end' }}>
                <button 
                  type="button" 
                  className="btn btn-secondary"
                  onClick={() => setShowCreateModal(false)}
                >
                  Cancel
                </button>
                <button type="submit" className="btn btn-primary">
                  Create Project
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  );
};

export default Projects;
