#!/bin/bash
# Development Environment Setup Script - REChain Network Solutions LLC
# Complete setup for development environment

set -e

echo "🚀 Setting up REChain development environment..."

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_section() {
    echo -e "${BLUE}[SETUP]${NC} $1"
}

# Check OS
detect_os() {
    print_section "Detecting operating system..."

    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        OS="linux"
        print_status "Detected Linux system"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        OS="macos"
        print_status "Detected macOS system"
    elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
        OS="windows"
        print_status "Detected Windows system"
    else
        print_status "Unknown OS: $OSTYPE"
        OS="unknown"
    fi
}

# Install system dependencies
install_dependencies() {
    print_section "Installing system dependencies..."

    case $OS in
        "linux")
            # Ubuntu/Debian
            if command -v apt-get &> /dev/null; then
                print_status "Installing dependencies via apt..."
                sudo apt-get update
                sudo apt-get install -y \
                    build-essential \
                    clang \
                    cmake \
                    libssl-dev \
                    pkg-config \
                    protobuf-compiler \
                    git \
                    curl \
                    wget \
                    docker.io \
                    docker-compose \
                    nodejs \
                    npm
            fi
            ;;
        "macos")
            # macOS with Homebrew
            if command -v brew &> /dev/null; then
                print_status "Installing dependencies via Homebrew..."
                brew install \
                    cmake \
                    openssl \
                    protobuf \
                    git \
                    docker \
                    docker-compose \
                    node \
                    npm
            else
                print_status "Please install Homebrew first: https://brew.sh/"
                exit 1
            fi
            ;;
        "windows")
            print_status "Please install dependencies manually on Windows:"
            print_status "1. Install Rust: https://rustup.rs/"
            print_status "2. Install Docker Desktop: https://docker.com/desktop"
            print_status "3. Install Node.js: https://nodejs.org/"
            print_status "4. Install Git: https://git-scm.com/"
            ;;
    esac
}

# Install Rust
install_rust() {
    print_section "Installing Rust toolchain..."

    if ! command -v rustc &> /dev/null; then
        print_status "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source ~/.cargo/env
    else
        print_status "Rust already installed"
    fi

    # Install nightly toolchain for fuzzing
    print_status "Installing nightly toolchain..."
    rustup install nightly
    rustup component add rustfmt clippy --toolchain nightly

    # Install development tools
    print_status "Installing development tools..."
    cargo install cargo-expand
    cargo install cargo-fuzz
    cargo install cargo-audit
    cargo install cargo-deny
    cargo install sqlx-cli
}

# Install Node.js dependencies
install_nodejs_deps() {
    print_section "Installing Node.js dependencies..."

    if command -v npm &> /dev/null; then
        print_status "Installing development tools..."
        npm install -g \
            typescript \
            ts-node \
            prettier \
            eslint \
            yarn \
            lerna
    fi
}

# Set up Git hooks
setup_git_hooks() {
    print_section "Setting up Git hooks..."

    mkdir -p .git/hooks

    # Pre-commit hook
    cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# Pre-commit hook for REChain SDK

set -e

echo "🔍 Running pre-commit checks..."

# Check formatting
echo "📝 Checking code formatting..."
cargo fmt --all -- --check

# Run Clippy
echo "🔍 Running Clippy..."
cargo clippy --workspace --all-targets -- -D warnings

# Run tests
echo "🧪 Running tests..."
cargo test --workspace

echo "✅ Pre-commit checks passed!"
EOF

    chmod +x .git/hooks/pre-commit

    print_status "Git hooks configured"
}

# Set up development database
setup_database() {
    print_section "Setting up development database..."

    if command -v docker &> /dev/null; then
        print_status "Starting PostgreSQL database..."
        docker run -d \
            --name rechain-dev-db \
            -e POSTGRES_DB=rechain_dev \
            -e POSTGRES_USER=rechain \
            -e POSTGRES_PASSWORD=rechain_password \
            -p 5432:5432 \
            postgres:15

        print_status "PostgreSQL database started on localhost:5432"
    fi
}

# Set up monitoring
setup_monitoring() {
    print_section "Setting up monitoring stack..."

    if command -v docker &> /dev/null; then
        print_status "Starting monitoring stack..."
        docker-compose up -d prometheus grafana

        print_status "Monitoring available at:"
        print_status "  Prometheus: http://localhost:9090"
        print_status "  Grafana: http://localhost:3000 (admin/admin)"
    fi
}

# Set up environment files
setup_env_files() {
    print_section "Setting up environment files..."

    # Create development environment file
    cat > .env.development << EOF
# REChain Development Environment
NODE_ENV=development
DATABASE_URL=postgresql://rechain:rechain_password@localhost:5432/rechain_dev
REDIS_URL=redis://localhost:6379
RUST_LOG=debug
WS_ENDPOINT=ws://localhost:9944
RPC_ENDPOINT=http://localhost:9933

# Oracle API Keys (add your own)
COINGECKO_API_KEY=your_coingecko_api_key
ALCHEMY_API_KEY=your_alchemy_api_key

# Monitoring
PROMETHEUS_ENDPOINT=http://localhost:9090
GRAFANA_ENDPOINT=http://localhost:3000

# Security
JWT_SECRET=your_jwt_secret_key_here
ENCRYPTION_KEY=your_encryption_key_here
EOF

    print_status "Environment files created"
}

# Build the project
build_project() {
    print_section "Building project..."

    print_status "Building workspace..."
    cargo build --workspace

    print_status "Running tests..."
    cargo test --workspace

    print_status "Build completed successfully"
}

# Set up IDE configuration
setup_ide() {
    print_section "Setting up IDE configuration..."

    # VS Code configuration
    mkdir -p .vscode

    cat > .vscode/settings.json << EOF
{
    "rust-analyzer.enable": true,
    "rust-analyzer.cargo.loadOutDirsFromCheck": true,
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.checkOnSave": true,
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.lens.debug": true,
    "rust-analyzer.lens.implementations": true,
    "rust-analyzer.lens.references": true,
    "rust-analyzer.server.extraEnv": {
        "RUST_BACKTRACE": "1"
    },
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    },
    "[toml]": {
        "editor.defaultFormatter": "tamasfe.even-better-toml"
    },
    "[json]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode"
    },
    "[yaml]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode"
    },
    "[markdown]": {
        "editor.defaultFormatter": "esbenp.prettier-vscode"
    }
}
EOF

    cat > .vscode/launch.json << EOF
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug REChain Node",
            "type": "cppdbg",
            "request": "launch",
            "program": "\${workspaceFolder}/target/debug/rechain",
            "args": ["--dev", "--ws-external"],
            "cwd": "\${workspaceFolder}",
            "externalConsole": false,
            "MIMode": "gdb"
        },
        {
            "name": "Debug Tests",
            "type": "cppdbg",
            "request": "launch",
            "program": "\${workspaceFolder}/target/debug/deps/pallet_*",
            "args": [],
            "cwd": "\${workspaceFolder}",
            "MIMode": "gdb"
        }
    ]
}
EOF

    print_status "IDE configuration completed"
}

# Main setup flow
main() {
    print_status "=== REChain Network Solutions LLC - Development Environment Setup ==="

    detect_os
    install_dependencies
    install_rust
    install_nodejs_deps
    setup_git_hooks
    setup_database
    setup_monitoring
    setup_env_files
    setup_ide
    build_project

    print_status "🎉 Development environment setup completed!"
    print_status ""
    print_status "=== Next Steps ==="
    print_status "1. Start developing:"
    print_status "   cargo build --workspace"
    print_status "   cargo test --workspace"
    print_status ""
    print_status "2. Start local network:"
    print_status "   cargo run --bin rechain -- --dev"
    print_status ""
    print_status "3. Access applications:"
    print_status "   Polkadot.js Apps: http://localhost:9944"
    print_status "   Prometheus: http://localhost:9090"
    print_status "   Grafana: http://localhost:3000"
    print_status ""
    print_status "4. View documentation:"
    print_status "   docs/README.md"
    print_status ""
    print_status "=== Support ==="
    print_status "📚 Documentation: https://docs.rechain.network"
    print_status "💬 Discord: https://discord.gg/rechain"
    print_status "📧 Email: info@rechain.network"
}

# Run main function
main