import React from 'react';
import { Link, useLocation } from 'react-router-dom';

const Navbar: React.FC = () => {
  const location = useLocation();

  const isActive = (path: string) => {
    if (path === '/') return location.pathname === '/';
    return location.pathname.startsWith(path);
  };

  return (
    <nav className="navbar">
      <Link to="/" className="navbar-brand">
        <div className="logo"></div>
        Accessibility Audit Tool
      </Link>
      <ul className="navbar-nav">
        <li>
          <Link 
            to="/" 
            className={`nav-link ${isActive('/') ? 'active' : ''}`}
          >
            Dashboard
          </Link>
        </li>
        <li>
          <Link 
            to="/projects" 
            className={`nav-link ${isActive('/projects') && location.pathname !== '/audit' ? 'active' : ''}`}
          >
            Projects
          </Link>
        </li>
        <li>
          <Link 
            to="/audit" 
            className={`nav-link ${location.pathname === '/audit' ? 'active' : ''}`}
          >
            New Audit
          </Link>
        </li>
      </ul>
    </nav>
  );
};

export default Navbar;
