# Accessibility Audit Tool

A comprehensive CLI + web dashboard for scanning websites and applications for WCAG (Web Content Accessibility Guidelines) compliance issues. Built with Rust backend and React frontend.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)
![React](https://img.shields.io/badge/React-18.2+-61dafb.svg)

## 🌟 Features

- **Headless Browser Automation**: Uses headless Chrome for real-page rendering and analysis
- **axe-core Integration**: Industry-standard accessibility testing engine
- **WCAG Compliance Reporting**: Detailed reports for WCAG 2.1 Levels A, AA, and AAA
- **Beautiful Dark-Themed Dashboard**: Modern React UI with visual issue overlays
- **Project Management**: Save scans, compare over time, track progress
- **RESTful API**: Clean Axum-based backend with comprehensive endpoints
- **Comprehensive Test Suite**: Unit tests for both backend and frontend

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ (with `cargo`)
- Node.js 18+ and npm/yarn
- Google Chrome or Chromium browser
- Docker (optional, for containerized deployment)

### Backend Setup

```bash
# Navigate to backend directory
cd backend

# Install dependencies
cargo build

# Download axe-core (required for full functionality)
curl -L https://raw.githubusercontent.com/dequelabs/axe-core/main/axe.min.js -o axe-core.min.js

# Run the server
cargo run
```

The backend will start on `http://localhost:3000` by default.

### Frontend Setup

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Start development server
npm start
```

The frontend will open at `http://localhost:3001`.

## 📖 API Documentation

### Endpoints

#### Projects

- `POST /projects` - Create a new audit project
- `GET /projects` - List all projects
- `GET /projects/:id` - Get project details
- `PUT /projects/:id` - Update project
- `DELETE /projects/:id` - Delete project

#### Audits

- `POST /audit` - Start a new accessibility audit
  ```json
  {
    "url": "https://example.com",
    "project_id": "uuid-here",
    "check_level": "AA"
  }
  ```

#### Reports

- `GET /projects/:id/reports` - Get all scans for a project
- `GET /projects/:id/reports/latest` - Get latest scan result
- `GET /projects/:id/compare` - Compare scans over time

### Example Usage

```bash
# Create a project
curl -X POST http://localhost:3000/projects \
  -H "Content-Type: application/json" \
  -d '{
    "name": "My Website",
    "target_url": "https://example.com"
  }'

# Start an audit
curl -X POST http://localhost:3000/audit \
  -H "Content-Type: application/json" \
  -d '{
    "url": "https://example.com",
    "project_id": "<project-uuid>"
  }'
```

## 🏗️ Architecture

### Backend (Rust)

```
backend/
├── src/
│   ├── main.rs          # Application entry point
│   ├── api/             # REST API routes
│   │   ├── audit.rs     # Audit endpoints
│   │   ├── projects.rs  # Project management
│   │   └── reports.rs   # Report generation
│   ├── core/            # Core domain models
│   │   ├── models.rs    # Data structures
│   │   └── errors.rs    # Error handling
│   ├── services/        # Business logic
│   │   ├── audit_service.rs  # Browser automation & axe-core
│   │   └── storage.rs        # Project persistence
│   └── utils/           # Utilities and validators
├── tests/               # Integration tests
└── Cargo.toml
```

### Frontend (React)

```
frontend/
├── src/
│   ├── components/      # Reusable UI components
│   │   └── Navbar.tsx
│   ├── pages/           # Page components
│   │   ├── Dashboard.tsx
│   │   ├── Projects.tsx
│   │   ├── ProjectDetail.tsx
│   │   └── AuditPage.tsx
│   ├── utils/           # Utilities
│   │   └── api.ts       # API client
│   ├── styles/          # CSS styles
│   └── App.tsx          # Main app component
├── public/
└── package.json
```

## 🧪 Testing

### Backend Tests

```bash
cd backend
cargo test
```

### Frontend Tests

```bash
cd frontend
npm test
```

## 🔧 Configuration

### Environment Variables

**Backend:**
- `PORT` - Server port (default: 3000)
- `RUST_LOG` - Logging level (default: info)

**Frontend:**
- `REACT_APP_API_URL` - Backend API URL (default: http://localhost:3000)

## 📊 WCAG Compliance Levels

The tool checks for compliance with three levels:

- **Level A**: Minimum accessibility requirements
- **Level AA**: Standard accessibility (recommended target)
- **Level AAA**: Enhanced accessibility (highest level)

### Common Issues Detected

1. **Critical**
   - Missing alt text on images
   - Form fields without labels
   - Color contrast failures

2. **Serious**
   - Keyboard navigation issues
   - Missing ARIA attributes
   - Improper heading hierarchy

3. **Moderate**
   - Link purpose unclear
   - Insufficient error identification
   - Timing issues

4. **Minor**
   - Presentational hints in markup
   - Redundant links
   - Minor semantic issues

## 🐳 Docker Deployment

### Build and Run

```bash
# Build backend image
docker build -t accessibility-audit-backend ./backend

# Build frontend image
docker build -t accessibility-audit-frontend ./frontend

# Run with docker-compose
docker-compose up -d
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- [axe-core](https://github.com/dequelabs/axe-core) - Accessibility engine
- [headless_chrome](https://crates.io/crates/headless_chrome) - Rust Chrome automation
- [Axum](https://github.com/tokio-rs/axum) - Ergonomic Rust web framework
- [React](https://react.dev/) - UI library

## 📞 Support

For issues, questions, or contributions, please open a GitHub issue.

---

Built with ❤️ for a more accessible web
