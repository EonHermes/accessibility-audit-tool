.PHONY: help backend-frontend-backend-test frontend-test docker-build docker-up docker-down clean

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

backend-build: ## Build the Rust backend
	cd backend && cargo build

backend-run: ## Run the Rust backend
	cd backend && cargo run

backend-test: ## Run backend tests
	cd backend && cargo test

backend-fmt: ## Format backend code
	cd backend && cargo fmt

backend-clippy: ## Run clippy linter
	cd backend && cargo clippy -- -D warnings

frontend-install: ## Install frontend dependencies
	cd frontend && npm install

frontend-start: ## Start frontend development server
	cd frontend && npm start

frontend-build: ## Build frontend for production
	cd frontend && npm run build

frontend-test: ## Run frontend tests
	cd frontend && npm test

docker-build: ## Build Docker images
	docker-compose build

docker-up: ## Start all services in detached mode
	docker-compose up -d

docker-down: ## Stop all services
	docker-compose down

docker-logs: ## View logs from all services
	docker-compose logs -f

clean: ## Clean build artifacts
	cd backend && cargo clean
	rm -rf frontend/node_modules frontend/build frontend/dist

setup: ## Complete setup for development
	@echo "Setting up Accessibility Audit Tool..."
	@make backend-build || echo "Backend build may need Chrome installed"
	@make frontend-install
	@echo "Setup complete! Run 'make backend-run' and 'make frontend-start'"

download-axe-core: ## Download latest axe-core library
	curl -L https://raw.githubusercontent.com/dequelabs/axe-core/main/axe.min.js -o backend/axe-core.min.js
	@echo "axe-core downloaded to backend/axe-core.min.js"
