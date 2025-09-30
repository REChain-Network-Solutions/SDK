# Deployment Guide

Complete deployment guide for the REChain SDK blockchain platform.

## 🚀 Quick Deployment

### Using Scripts
```bash
# Deploy to testnet
make deploy

# Or use the deployment script directly
./scripts/deploy.sh rechain-testnet full 4
```

### Using Docker
```bash
# Start with Docker Compose
docker-compose up -d

# Access your node
# RPC: ws://localhost:9944
# Explorer: http://localhost:3000
```

## 📋 Deployment Options

### 1. Local Development Network

**Single Node (Development)**
```bash
cargo run --bin rechain -- --dev --ws-external --rpc-external
```

**Multi-Node (Testing)**
```bash
# Terminal 1 - Validator 1
cargo run --bin rechain -- --alice --validator --ws-port 9944

# Terminal 2 - Validator 2
cargo run --bin rechain -- --bob --validator --ws-port 9945 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/ALICE_NODE_ID
```

### 2. Public Testnet

**Automated Deployment**
```bash
# Deploy to public testnet
./scripts/deploy.sh rechain-testnet full 4

# Monitor deployment
docker-compose logs -f
```

**Manual Deployment**
```bash
# Build release binary
cargo build --release --workspace

# Generate chain specification
./target/release/rechain build-spec --chain testnet > testnet-spec.json

# Start validator nodes
./target/release/rechain --chain testnet-spec.json --validator --name "validator-1"
```

### 3. Production Mainnet

**Prerequisites**
- Secure server infrastructure
- SSL certificates
- Monitoring setup
- Backup systems

**Production Deployment**
```bash
# Use production deployment script
./scripts/deploy.sh rechain-mainnet full 10

# Or deploy with Kubernetes
kubectl apply -f k8s/
```

## 🛠️ Infrastructure Setup

### System Requirements

**Minimum Requirements:**
- CPU: 4 cores
- RAM: 8 GB
- Storage: 100 GB SSD
- Network: 100 Mbps

**Recommended Requirements:**
- CPU: 8+ cores
- RAM: 16+ GB
- Storage: 1 TB NVMe SSD
- Network: 1 Gbps

### Network Configuration

**Ports to Open:**
- `30333` - P2P communication
- `9933` - RPC (HTTP)
- `9944` - WebSocket
- `9615` - Metrics
- `9090` - Prometheus
- `3000` - Grafana

**Firewall Configuration:**
```bash
# Ubuntu/Debian
sudo ufw allow 30333
sudo ufw allow 9933
sudo ufw allow 9944

# CentOS/RHEL
sudo firewall-cmd --permanent --add-port=30333/tcp
sudo firewall-cmd --permanent --add-port=9933/tcp
sudo firewall-cmd --permanent --add-port=9944/tcp
```

## 📊 Monitoring Setup

### Prometheus Configuration

**prometheus.yml:**
```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

scrape_configs:
  - job_name: 'rechain-nodes'
    static_configs:
      - targets: ['localhost:9615']
    scrape_interval: 5s
```

### Grafana Dashboards

**Access Grafana:**
- URL: http://localhost:3000
- Username: admin
- Password: admin (change in production)

**Key Metrics to Monitor:**
- Block height and finalization
- Transaction throughput (TPS)
- Validator performance
- Network latency
- Storage usage
- Memory consumption

## 🔒 Security Setup

### Key Management

**Validator Keys:**
```bash
# Generate session keys
./target/release/rechain key generate --scheme Sr25519

# Generate node keys
./target/release/rechain key generate-node-key
```

**Key Storage:**
```bash
# Store keys securely
mkdir -p /secure/keystore
chmod 700 /secure/keystore

# Move keys to secure location
mv validator-keys/* /secure/keystore/
```

### SSL/TLS Setup

**Generate Certificates:**
```bash
# Generate self-signed certificate (development)
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes

# Or use Let's Encrypt (production)
certbot certonly --standalone -d your-domain.com
```

**Configure SSL:**
```toml
# Add to node configuration
[rpc]
enable = true
port = 9933
cors = "https://polkadot.js.org,https://app.rechain.network"

[rpc_ws]
enable = true
port = 9944
```

## 🚀 Scaling Setup

### Horizontal Scaling

**Load Balancer Configuration:**
```nginx
upstream rechain_nodes {
    server 127.0.0.1:9944;
    server 127.0.0.1:9945;
    server 127.0.0.1:9946;
}

server {
    listen 80;
    server_name api.rechain.network;

    location / {
        proxy_pass http://rechain_nodes;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

### Database Scaling

**PostgreSQL Optimization:**
```sql
-- Optimize for blockchain data
CREATE INDEX CONCURRENTLY idx_blocks_number ON blocks(number);
CREATE INDEX CONCURRENTLY idx_transactions_hash ON transactions(hash);
CREATE INDEX CONCURRENTLY idx_events_block ON events(block_number);
```

## 🔧 Maintenance Procedures

### Regular Backups

**Automated Backups:**
```bash
# Set up cron job for daily backups
echo "0 2 * * * /path/to/rechain/scripts/backup.sh /backup/daily 30 rechain" | sudo crontab -

# Set up cron job for weekly backups
echo "0 3 * * 0 /path/to/rechain/scripts/backup.sh /backup/weekly 90 rechain" | sudo crontab -
```

### Log Rotation

**Logrotate Configuration:**
```
/var/log/rechain/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 rechain rechain
}
```

### Health Monitoring

**Health Check Script:**
```bash
#!/bin/bash
# Node health check

RPC_ENDPOINT="http://localhost:9933"
WS_ENDPOINT="ws://localhost:9944"

# Check RPC health
if curl -s "$RPC_ENDPOINT/health" > /dev/null; then
    echo "✅ RPC healthy"
else
    echo "❌ RPC unhealthy"
    systemctl restart rechain
fi

# Check WebSocket health
if timeout 10s curl -s "$WS_ENDPOINT" > /dev/null; then
    echo "✅ WebSocket healthy"
else
    echo "❌ WebSocket unhealthy"
    systemctl restart rechain
fi
```

## 🚨 Emergency Procedures

### Node Recovery

**If Node Stops Syncing:**
```bash
# Check node status
journalctl -u rechain -f

# Restart node
sudo systemctl restart rechain

# Resync if needed
./target/release/rechain purge-chain --chain testnet
./target/release/rechain --chain testnet --sync warp
```

### Database Recovery

**If Database Corrupts:**
```bash
# Stop services
sudo systemctl stop rechain

# Restore from backup
./scripts/restore.sh /path/to/backup.tar.gz

# Restart services
sudo systemctl start rechain
```

## 📈 Performance Optimization

### Node Optimization

**Runtime Configuration:**
```toml
[execution]
wasm_method = "interpreted"

[network]
max_parallel_downloads = 10
max_blocks_in_response = 100

[database]
cache_size = 134217728  # 128MB
```

### System Optimization

**Linux Kernel Tuning:**
```bash
# Increase file descriptor limit
echo "fs.file-max = 100000" >> /etc/sysctl.conf

# Increase network buffer sizes
echo "net.core.rmem_max = 134217728" >> /etc/sysctl.conf
echo "net.core.wmem_max = 134217728" >> /etc/sysctl.conf

# Apply changes
sysctl -p
```

## 🔍 Troubleshooting

### Common Issues

**Issue: Node won't start**
```bash
# Check logs
journalctl -u rechain -n 50

# Check configuration
./target/release/rechain --version

# Check ports
netstat -tlnp | grep 30333
```

**Issue: Slow synchronization**
```bash
# Enable warp sync
./target/release/rechain --sync warp

# Increase cache size
./target/release/rechain --db-cache 256
```

**Issue: High memory usage**
```bash
# Check memory usage
htop

# Optimize memory settings
./target/release/rechain --db-cache 128 --state-cache-size 1
```

## 📞 Support

### Getting Help

**Development Issues:**
- GitHub Issues: https://github.com/REChain-Network-Solutions/SDK/issues
- GitHub Discussions: https://github.com/REChain-Network-Solutions/SDK/discussions

**Production Issues:**
- Email: support@rechain.network
- Emergency: emergency@rechain.network

**Community Support:**
- Discord: https://discord.gg/rechain
- Stack Exchange: https://substrate.stackexchange.com

## 📚 Additional Resources

- [Official Documentation](https://docs.rechain.network)
- [API Reference](https://docs.rechain.network/api)
- [Node Operators Guide](https://docs.rechain.network/operators)
- [Security Best Practices](https://docs.rechain.network/security)
- [Performance Tuning](https://docs.rechain.network/performance)

---

*This deployment guide was generated by REChain Network Solutions LLC*
*Last updated: $(date)*