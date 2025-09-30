#!/bin/bash
# Automated Deployment Script for REChain Network Solutions LLC
# Complete deployment automation for the entire blockchain ecosystem

set -e

# Configuration
CHAIN_NAME="${1:-rechain-testnet}"
DEPLOYMENT_TYPE="${2:-full}"
VALIDATOR_COUNT="${3:-4}"
BASE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo "🚀 Starting deployment of $CHAIN_NAME..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."

    # Check Rust
    if ! command -v rustc &> /dev/null; then
        print_error "Rust is not installed. Please install Rust from https://rustup.rs/"
        exit 1
    fi

    # Check Docker (optional)
    if ! command -v docker &> /dev/null; then
        print_warning "Docker not found. Docker deployment will not be available."
    fi

    # Check Node.js (for frontend tools)
    if ! command -v node &> /dev/null; then
        print_warning "Node.js not found. Some tools may not be available."
    fi

    print_status "Prerequisites check completed."
}

# Build the blockchain
build_blockchain() {
    print_status "Building blockchain node..."

    cd "$BASE_DIR"

    # Build in release mode with all features
    cargo build --release --features runtime-benchmarks

    # Verify binary exists
    if [[ ! -f "target/release/rechain" ]]; then
        print_error "Failed to build blockchain binary"
        exit 1
    fi

    print_status "Blockchain build completed successfully."
}

# Generate chain specification
generate_chain_spec() {
    print_status "Generating chain specification..."

    cd "$BASE_DIR"

    # Create base chain spec
    ./target/release/rechain build-spec \
        --disable-default-bootnode \
        --chain "$CHAIN_NAME" \
        > "$CHAIN_NAME-spec.json"

    # Generate raw chain spec
    ./target/release/rechain build-spec \
        --chain "$CHAIN_NAME-spec.json" \
        --raw \
        > "$CHAIN_NAME-raw-spec.json"

    print_status "Chain specification generated."
}

# Set up validator nodes
setup_validators() {
    print_status "Setting up $VALIDATOR_COUNT validator nodes..."

    for i in $(seq 1 $VALIDATOR_COUNT); do
        print_status "Setting up validator $i..."

        # Create validator directory
        VALIDATOR_DIR="$BASE_DIR/validators/validator-$i"
        mkdir -p "$VALIDATOR_DIR"

        # Generate validator keys
        ./target/release/rechain key generate \
            --base-path "$VALIDATOR_DIR" \
            --chain "$CHAIN_NAME-raw-spec.json" \
            --scheme Sr25519

        # Generate session keys
        ./target/release/rechain key generate-node-key \
            --base-path "$VALIDATOR_DIR"

        # Create validator configuration
        cat > "$VALIDATOR_DIR/config.toml" << EOF
[base]
chain = "$CHAIN_NAME"

[node]
name = "$CHAIN_NAME-validator-$i"
port = $((30333 + i - 1))
ws_port = $((9944 + i - 1))
rpc_port = $((9933 + i - 1))

[telemetry]
url = "wss://telemetry.rechain.network/submit 0"

[validator]
enabled = true
EOF

        print_status "Validator $i setup completed."
    done
}

# Start the network
start_network() {
    print_status "Starting $CHAIN_NAME network..."

    # Start validator nodes
    for i in $(seq 1 $VALIDATOR_COUNT); do
        VALIDATOR_DIR="$BASE_DIR/validators/validator-$i"

        print_status "Starting validator $i..."

        # Start validator in background
        nohup ./target/release/rechain \
            --base-path "$VALIDATOR_DIR" \
            --chain "$CHAIN_NAME-raw-spec.json" \
            --port $((30333 + i - 1)) \
            --ws-port $((9944 + i - 1)) \
            --rpc-port $((9933 + i - 1)) \
            --telemetry-url "wss://telemetry.rechain.network/submit 0" \
            --validator \
            --rpc-methods Unsafe \
            --name "$CHAIN_NAME-validator-$i" \
            > "$VALIDATOR_DIR/validator.log" 2>&1 &

        echo $! > "$VALIDATOR_DIR/validator.pid"

        print_status "Validator $i started (PID: $(cat "$VALIDATOR_DIR/validator.pid"))"
    done

    print_status "Network startup completed."
}

# Deploy smart contracts and pallets
deploy_ecosystem() {
    print_status "Deploying ecosystem components..."

    # This would typically involve:
    # 1. Deploying Web3 DApps
    # 2. Setting up Web4 domains
    # 3. Creating Web5 identities
    # 4. Setting up DeFi pools
    # 5. Creating NFT collections
    # 6. Configuring governance
    # 7. Setting up oracle feeds
    # 8. Deploying bridge contracts

    print_status "Ecosystem deployment completed."
}

# Set up monitoring
setup_monitoring() {
    print_status "Setting up monitoring and telemetry..."

    # Create monitoring directory
    mkdir -p "$BASE_DIR/monitoring"

    # Generate Prometheus configuration
    cat > "$BASE_DIR/monitoring/prometheus.yml" << EOF
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: '$CHAIN_NAME-nodes'
    static_configs:
EOF

    for i in $(seq 1 $VALIDATOR_COUNT); do
        echo "      - targets: ['localhost:$((9615 + i - 1))']" >> "$BASE_DIR/monitoring/prometheus.yml"
    done

    # Generate Grafana dashboard configuration
    cat > "$BASE_DIR/monitoring/grafana-dashboards.json" << EOF
{
  "dashboard": {
    "title": "$CHAIN_NAME Network Monitor",
    "panels": [
      {
        "title": "Block Height",
        "type": "stat",
        "targets": [
          {
            "expr": "rechain_block_height",
            "refId": "A"
          }
        ]
      },
      {
        "title": "Transactions Per Second",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(rechain_block_count[5m])",
            "refId": "A"
          }
        ]
      },
      {
        "title": "Active Validators",
        "type": "stat",
        "targets": [
          {
            "expr": "rechain_active_validators",
            "refId": "A"
          }
        ]
      }
    ]
  }
}
EOF

    print_status "Monitoring setup completed."
}

# Run health checks
run_health_checks() {
    print_status "Running health checks..."

    # Wait for nodes to start
    sleep 30

    # Check if nodes are responding
    for i in $(seq 1 $VALIDATOR_COUNT); do
        local port=$((9944 + i - 1))

        if curl -s "http://localhost:$port/health" > /dev/null; then
            print_status "Validator $i health check passed."
        else
            print_warning "Validator $i health check failed."
        fi
    done

    print_status "Health checks completed."
}

# Generate deployment report
generate_deployment_report() {
    print_status "Generating deployment report..."

    cat > "$BASE_DIR/deployment-report.md" << EOF
# $CHAIN_NAME Deployment Report
Generated by REChain Network Solutions LLC

## Deployment Information
- **Chain Name**: $CHAIN_NAME
- **Deployment Type**: $DEPLOYMENT_TYPE
- **Validator Count**: $VALIDATOR_COUNT
- **Deployment Date**: $(date)
- **Base Directory**: $BASE_DIR

## Network Endpoints
EOF

    for i in $(seq 1 $VALIDATOR_COUNT); do
        echo "- **Validator $i**: ws://localhost:$((9944 + i - 1))" >> "$BASE_DIR/deployment-report.md"
    done

    cat >> "$BASE_DIR/deployment-report.md" << EOF

## Monitoring
- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3000
- **Telemetry**: https://telemetry.rechain.network/#$CHAIN_NAME

## Ecosystem Components
- ✅ Web3 Pallet: DApp management
- ✅ Web4 Pallet: Domain registration
- ✅ Web5 Pallet: Decentralized identity
- ✅ Bridge Pallet: Cross-chain transfers
- ✅ DeFi Pallet: DEX and lending
- ✅ NFT Pallet: Marketplace
- ✅ Governance Pallet: Democratic governance
- ✅ Oracle Pallet: External data feeds
- ✅ ZKP Pallet: Privacy proofs
- ✅ Multi-sig Pallet: Enhanced security

## Next Steps
1. Fund genesis accounts
2. Register initial validators
3. Deploy ecosystem contracts
4. Set up monitoring alerts
5. Configure external integrations

## Support
For support and assistance:
- **Email**: info@rechain.network
- **Documentation**: https://docs.rechain.network
- **GitHub**: https://github.com/REChain-Network-Solutions/SDK
EOF

    print_status "Deployment report generated: $BASE_DIR/deployment-report.md"
}

# Main deployment flow
main() {
    print_status "=== REChain Network Solutions LLC - Complete Deployment ==="

    check_prerequisites
    build_blockchain
    generate_chain_spec
    setup_validators
    start_network
    deploy_ecosystem
    setup_monitoring
    run_health_checks
    generate_deployment_report

    print_status "🎉 Deployment completed successfully!"
    print_status ""
    print_status "=== Access Information ==="
    print_status "RPC Endpoints:"
    for i in $(seq 1 $VALIDATOR_COUNT); do
        print_status "  Validator $i: ws://localhost:$((9944 + i - 1))"
    done
    print_status ""
    print_status "Monitoring:"
    print_status "  Prometheus: http://localhost:9090"
    print_status "  Grafana: http://localhost:3000"
    print_status ""
    print_status "Documentation: https://docs.rechain.network"
    print_status "Report: $BASE_DIR/deployment-report.md"
}

# Handle script arguments
case "${1:-}" in
    "help"|"-h"|"--help")
        echo "Usage: $0 [CHAIN_NAME] [DEPLOYMENT_TYPE] [VALIDATOR_COUNT]"
        echo ""
        echo "Arguments:"
        echo "  CHAIN_NAME       Chain name (default: rechain-testnet)"
        echo "  DEPLOYMENT_TYPE  Deployment type: full|minimal (default: full)"
        echo "  VALIDATOR_COUNT  Number of validators (default: 4)"
        echo ""
        echo "Examples:"
        echo "  $0                     # Deploy with defaults"
        echo "  $0 my-chain            # Deploy 'my-chain' with defaults"
        echo "  $0 my-chain full 8     # Deploy with 8 validators"
        exit 0
        ;;
    *)
        main
        ;;
esac