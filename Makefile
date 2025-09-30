# Makefile for REChain SDK - REChain Network Solutions LLC
# Complete build automation and development workflow

.PHONY: help build test clean check format clippy security benchmark deploy docs install dev-env backup restore

# Default target
help:
	@echo "🚀 REChain SDK - Development Makefile"
	@echo ""
	@echo "Available targets:"
	@echo "  build          Build the entire workspace"
	@echo "  test           Run all tests"
	@echo "  clean          Clean build artifacts"
	@echo "  check          Check code without building"
	@echo "  format         Format code with rustfmt"
	@echo "  clippy         Run clippy lints"
	@echo "  security       Run security checks"
	@echo "  benchmark      Run benchmarks"
	@echo "  deploy         Deploy to testnet"
	@echo "  docs           Generate documentation"
	@echo "  install        Install development tools"
	@echo "  dev-env        Set up development environment"
	@echo "  backup         Create system backup"
	@echo "  restore        Restore from backup"
	@echo "  sdk            Generate SDKs"
	@echo "  monitor        Start monitoring stack"
	@echo "  validate       Validate entire setup"
	@echo "  release        Create release build"
	@echo "  audit          Run comprehensive audit"
	@echo ""
	@echo "Examples:"
	@echo "  make build     # Build everything"
	@echo "  make test      # Run all tests"
	@echo "  make deploy    # Deploy to testnet"
	@echo "  make dev-env   # Set up development environment"

# Variables
CARGO := cargo
RELEASE_FLAG := --release
FEATURES :=

# Build targets
build:
	@echo "🔨 Building REChain SDK..."
	@$(CARGO) build $(RELEASE_FLAG) --workspace $(FEATURES)
	@echo "✅ Build completed successfully"

test:
	@echo "🧪 Running tests..."
	@$(CARGO) test --workspace $(RELEASE_FLAG) $(FEATURES)
	@echo "✅ Tests completed successfully"

clean:
	@echo "🧹 Cleaning build artifacts..."
	@$(CARGO) clean
	@rm -rf node_modules dist build target
	@echo "✅ Clean completed"

check:
	@echo "🔍 Checking code..."
	@$(CARGO) check --workspace --all-targets $(FEATURES)
	@echo "✅ Check completed"

format:
	@echo "📝 Formatting code..."
	@$(CARGO) fmt --all
	@echo "✅ Format completed"

clippy:
	@echo "🔍 Running clippy..."
	@$(CARGO) clippy --workspace --all-targets -- -D warnings $(FEATURES)
	@echo "✅ Clippy completed"

security: clippy
	@echo "🔒 Running security checks..."
	@$(CARGO) audit || echo "⚠️  Cargo audit not available"
	@echo "✅ Security checks completed"

benchmark:
	@echo "⚡ Running benchmarks..."
	@$(CARGO) test --workspace $(RELEASE_FLAG) --features runtime-benchmarks
	@echo "✅ Benchmarks completed"

# Deployment targets
deploy:
	@echo "🚀 Deploying to testnet..."
	@chmod +x scripts/deploy.sh
	@./scripts/deploy.sh rechain-testnet
	@echo "✅ Deployment completed"

# Documentation targets
docs:
	@echo "📚 Generating documentation..."
	@$(CARGO) doc --workspace --all-features --no-deps --open
	@echo "✅ Documentation generated"

# Development environment
install:
	@echo "📦 Installing development tools..."
	@$(CARGO) install cargo-expand cargo-fuzz cargo-audit cargo-deny
	@npm install -g typescript prettier eslint
	@echo "✅ Installation completed"

dev-env:
	@echo "🛠️  Setting up development environment..."
	@chmod +x scripts/setup-dev-env.sh
	@./scripts/setup-dev-env.sh
	@echo "✅ Development environment setup completed"

# Data management
backup:
	@echo "💾 Creating backup..."
	@chmod +x scripts/backup.sh
	@./scripts/backup.sh ./backups 30 rechain
	@echo "✅ Backup completed"

restore:
	@echo "🔄 Restoring from backup..."
	@echo "Please specify backup file to restore from"
	@echo "Usage: make restore BACKUP_FILE=/path/to/backup.tar.gz"

# SDK generation
sdk:
	@echo "🔧 Generating SDKs..."
	@chmod +x scripts/generate-sdk.sh
	@./scripts/generate-sdk.sh rechain-dapp all ./sdk
	@echo "✅ SDK generation completed"

# Monitoring
monitor:
	@echo "📊 Starting monitoring stack..."
	@docker-compose up -d prometheus grafana
	@echo "✅ Monitoring started"
	@echo "📈 Prometheus: http://localhost:9090"
	@echo "📊 Grafana: http://localhost:3000"

# Validation
validate: check test clippy security
	@echo "🔍 Running comprehensive validation..."
	@$(CARGO) doc --workspace --all-features --no-deps
	@echo "✅ Validation completed successfully"

# Release build
release: validate
	@echo "📦 Creating release build..."
	@$(CARGO) build --workspace $(RELEASE_FLAG)
	@mkdir -p release
	@cp target/release/rechain release/
	@cp target/release/rechain-execute-worker release/ 2>/dev/null || true
	@cp target/release/rechain-prepare-worker release/ 2>/dev/null || true
	@echo "✅ Release build completed"
	@echo "📁 Release artifacts: ./release/"

# Comprehensive audit
audit: validate security
	@echo "🔍 Running comprehensive audit..."
	@echo "📊 System Information:"
	@echo "  Rust version: $$(rustc --version)"
	@echo "  Cargo version: $$(cargo --version)"
	@echo "  OS: $$(uname -a)"
	@echo ""
	@echo "📈 Performance Metrics:"
	@$(CARGO) build --workspace --release --timings 2>/dev/null || echo "  Build timing: Available in target/debug/timings"
	@echo ""
	@echo "🔒 Security Status:"
	@$(CARGO) audit 2>/dev/null || echo "  Security audit: Install cargo-audit for security scanning"
	@echo ""
	@echo "📋 Workspace Status:"
	@$(CARGO) tree --workspace
	@echo ""
	@echo "✅ Comprehensive audit completed"

# Development workflow
dev: format clippy check test
	@echo "🔄 Development workflow completed"

# Quick check for CI
ci: check test
	@echo "✅ CI checks completed"

# Full validation for production
prod: audit benchmark
	@echo "🏭 Production validation completed"

# Update dependencies
update-deps:
	@echo "📦 Updating dependencies..."
	@$(CARGO) update
	@npm update 2>/dev/null || echo "NPM not available"

# Install system dependencies
install-system-deps:
	@echo "📦 Installing system dependencies..."
	@echo "Please install manually based on your OS:"
	@echo "  Ubuntu/Debian: sudo apt install build-essential clang cmake libssl-dev pkg-config protobuf-compiler"
	@echo "  macOS: brew install cmake openssl protobuf"
	@echo "  Arch: sudo pacman -S base-devel clang cmake openssl protobuf"

# Development shortcuts
quick-build:
	@$(CARGO) build --workspace

quick-test:
	@$(CARGO) test --workspace

# Maintenance
outdated:
	@echo "📦 Checking for outdated dependencies..."
	@$(CARGO) outdated || echo "Install cargo-outdated for dependency checking"

unused-deps:
	@echo "🔍 Checking for unused dependencies..."
	@$(CARGO) unused || echo "Install cargo-unused for dependency analysis"

# Environment specific builds
minimal:
	@$(CARGO) build --workspace --no-default-features

full:
	@$(CARGO) build --workspace --all-features

# Docker operations
docker-build:
	@echo "🐳 Building Docker images..."
	@docker-compose build

docker-up:
	@echo "🐳 Starting Docker containers..."
	@docker-compose up -d

docker-down:
	@echo "🐳 Stopping Docker containers..."
	@docker-compose down

# Node operations
start-node:
	@echo "🌐 Starting REChain node..."
	@$(CARGO) run --bin rechain -- --dev --ws-external --rpc-external

# Validator operations
start-validator:
	@echo "✅ Starting validator node..."
	@$(CARGO) run --bin rechain -- --validator --ws-external --rpc-external

# Monitoring
logs:
	@echo "📋 Showing recent logs..."
	@docker-compose logs -f

status:
	@echo "📊 System status..."
	@docker-compose ps

# Cleanup operations
clean-docker:
	@echo "🧹 Cleaning Docker artifacts..."
	@docker system prune -f

clean-all: clean clean-docker
	@echo "🧹 Complete cleanup finished"

# Help target with detailed information
help-detailed:
	@echo "🚀 REChain SDK - Detailed Development Makefile"
	@echo ""
	@echo "=== Development Workflow ==="
	@echo "  make dev           # Complete development cycle (format, clippy, check, test)"
	@echo "  make build         # Build workspace in release mode"
	@echo "  make test          # Run all tests"
	@echo "  make check         # Check code without building"
	@echo ""
	@echo "=== Code Quality ==="
	@echo "  make format        # Format code with rustfmt"
	@echo "  make clippy        # Run clippy lints"
	@echo "  make security      # Run security checks"
	@echo "  make audit         # Comprehensive audit (quality + security)"
	@echo ""
	@echo "=== Deployment ==="
	@echo "  make deploy        # Deploy to testnet"
	@echo "  make release       # Create production release"
	@echo "  make docker-up     # Start with Docker"
	@echo ""
	@echo "=== Development Environment ==="
	@echo "  make install       # Install development tools"
	@echo "  make dev-env       # Set up complete development environment"
	@echo "  make sdk           # Generate SDKs for all languages"
	@echo ""
	@echo "=== Operations ==="
	@echo "  make backup        # Create system backup"
	@echo "  make monitor       # Start monitoring stack"
	@echo "  make validate      # Validate entire setup"
	@echo ""
	@echo "=== Quick Commands ==="
	@echo "  make quick-build   # Fast build"
	@echo "  make quick-test    # Fast test"
	@echo "  make start-node    # Start development node"
	@echo "  make logs          # View system logs"
	@echo ""
	@echo "=== Examples ==="
	@echo "  make build && make test"
	@echo "  make dev-env && make sdk"
	@echo "  make deploy && make monitor"