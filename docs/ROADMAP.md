# Roadmap

A modern tunneling tool inspired by ngrok and [jprq.io](https://jprq.io/), built in Rust for performance and reliability.

## Core Tunneling

- [ ] **Basic HTTP Tunnel** - Forward local HTTP traffic through secure tunnel
- [ ] **HTTPS Support** - SSL/TLS termination and forwarding
- [ ] **WebSocket Tunneling** - Real-time bidirectional communication support
- [ ] **TCP Tunneling** - Raw TCP connection forwarding
- [ ] **Multiple Protocol Support** - HTTP/1.1, HTTP/2, WebSocket protocols

## Network & Infrastructure

- [x] **Port Scanner** - Network discovery and port availability checking
- [x] **HTTP Request Parser** - Parse and validate HTTP requests
- [x] **Basic HTTP Server** - Foundation server implementation
- [ ] **Reverse Proxy** - Request forwarding and load balancing
- [ ] **Connection Pooling** - Efficient connection management
- [ ] **Rate Limiting** - Traffic control and abuse prevention

## Client Features

- [ ] **CLI Interface** - Command-line tool for tunnel management
- [ ] **Auto-reconnection** - Automatic tunnel recovery on connection loss
- [ ] **Custom Subdomains** - User-defined subdomain allocation
- [ ] **Local Server Discovery** - Automatic detection of local services
- [ ] **Configuration Files** - Persistent tunnel configurations

## Server Features

- [ ] **URL Assignment** - Dynamic subdomain/URL generation
- [ ] **Request Routing** - Intelligent traffic direction
- [ ] **Multi-tenant Support** - Isolated tunnels per user
- [ ] **Domain Management** - Custom domain support
- [ ] **Load Balancing** - Distribute traffic across multiple tunnels

## Developer Experience

- [ ] **Web UI Dashboard** - Real-time tunnel monitoring interface
- [ ] **Request Inspector** - Debug incoming requests and responses
- [ ] **Traffic Analytics** - Connection metrics and statistics
- [ ] **Request Replay** - Resend captured requests for testing
- [ ] **Webhook Testing** - Validate webhook integrations
- [ ] **API Access** - Programmatic tunnel management

## Security & Auth

- [ ] **Authentication** - User accounts and API keys
- [ ] **Access Control** - IP whitelisting and tunnel permissions
- [ ] **Request Filtering** - Block malicious or unwanted traffic
- [ ] **Audit Logging** - Security event tracking
- [ ] **Encrypted Tunnels** - End-to-end encryption for sensitive data

## Performance & Reliability

- [ ] **Connection Multiplexing** - Efficient tunnel resource usage
- [ ] **Compression** - Reduce bandwidth usage
- [ ] **Health Monitoring** - Tunnel status and uptime tracking
- [ ] **Failover Support** - Backup tunnel endpoints
- [ ] **Performance Metrics** - Latency and throughput monitoring

## Advanced Features

- [ ] **Custom Headers** - Inject or modify HTTP headers
- [ ] **Request Transformation** - Modify requests before forwarding
- [ ] **Bandwidth Controls** - Upload/download limits
- [ ] **Geographic Routing** - Region-specific tunnel endpoints
- [ ] **Integration APIs** - Connect with external services and tools

---

*This roadmap is subject to change based on community feedback and development priorities.*
