# Contributing to Accessibility Audit Tool

Thank you for your interest in contributing! This document provides guidelines and instructions.

## 🌟 How to Contribute

### Reporting Bugs

Before creating bug reports, please check existing issues. When creating a bug report:

- Use a clear and descriptive title
- Describe the exact steps to reproduce the problem
- Provide specific examples if possible
- Describe your environment (OS, Rust/Node versions, browser)
- Include screenshots if applicable

### Suggesting Features

Feature suggestions are welcome! Please provide:

- A clear description of the feature
- Use cases and benefits
- Any relevant examples or mockups

### Pull Requests

1. **Fork** the repository
2. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes** with clear, descriptive commits
4. **Add tests** for new functionality
5. **Update documentation** as needed
6. **Ensure all tests pass**:
   ```bash
   # Backend
   cargo test
   
   # Frontend
   npm test
   ```
7. **Push and create a Pull Request**

## 📋 Code Style Guidelines

### Rust (Backend)

- Follow [Rust API guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Write meaningful doc comments
- Handle errors gracefully with proper error types

```rust
// Good example
/// Performs an accessibility audit on the specified URL.
/// 
/// # Arguments
/// * `url` - The URL to audit
/// * `check_level` - Optional WCAG level to check
///
/// # Returns
/// Result containing PageAuditResult or AppError
pub async fn audit_url(&self, url: &str) -> Result<PageAuditResult, AppError> {
    // Implementation
}
```

### TypeScript/React (Frontend)

- Use TypeScript for type safety
- Follow React best practices
- Use functional components with hooks
- Keep components small and focused
- Write meaningful prop types

```typescript
// Good example
interface ButtonProps {
  onClick: () => void;
  children: React.ReactNode;
  variant?: 'primary' | 'secondary';
}

const Button: React.FC<ButtonProps> = ({ 
  onClick, 
  children, 
  variant = 'primary' 
}) => {
  return (
    <button className={`btn btn-${variant}`} onClick={onClick}>
      {children}
    </button>
  );
};
```

## 🧪 Testing Requirements

### Backend Tests

- Unit tests for all services
- Integration tests for API endpoints
- Test error handling scenarios
- Aim for >80% code coverage

```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html
```

### Frontend Tests

- Component unit tests
- Integration tests for user flows
- Mock API calls in tests

```bash
# Run tests
npm test

# Run with coverage
npm run test:coverage
```

## 📝 Documentation

- Update README.md for significant changes
- Add inline comments for complex logic
- Document new API endpoints
- Include examples where helpful

## 🔀 Branch Strategy

- `main` - Production-ready code
- `feature/*` - New features
- `bugfix/*` - Bug fixes
- `hotfix/*` - Critical production fixes

## 🚀 Release Process

1. Ensure all tests pass
2. Update version in Cargo.toml and package.json
3. Create release notes
4. Tag the release
5. Publish to crates.io (backend) if applicable

## 💬 Questions?

Open an issue for questions or join discussions in GitHub Discussions.

---

Thank you for making this project better! 🙏
