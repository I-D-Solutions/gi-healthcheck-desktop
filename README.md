# GI Health Check Desktop

A cross-platform desktop health check application built with **Tauri** and **SvelteKit**. Part of the GI Orchestrator initiative by [I-D-Solutions](https://github.com/I-D-Solutions).

## Stack

- **Desktop shell** -- Tauri v2 (Rust)
- **Frontend** -- SvelteKit + TypeScript
- **Diagnostics** -- Rust sysinfo, tokio, chrono

## Health Checks (v0.1)

| Check | Category | Description |
|-------|----------|-------------|
| Hostname | Runtime | Resolves the machine hostname |
| OS Info | Runtime | OS name and architecture |
| App Data Dir | Runtime | Verifies writable app data directory |
| Memory | System | Available vs total RAM with thresholds |
| CPU | System | Core count, brand, average utilization |
| Disk Space | Storage | Free space per mount with low-disk warning |
| DNS Resolution | Network | Resolves `dns.google` to verify DNS |
| Network Reachability | Network | TCP connect to `1.1.1.1:443` with timeout |

## Prerequisites

- [Node.js](https://nodejs.org/) >= 20
- [Rust](https://www.rust-lang.org/tools/install) >= 1.88
- Platform build tools (Xcode CLI on macOS, Visual Studio Build Tools on Windows, etc.)

## Getting Started

```bash
# Install dependencies
npm install

# Run in development mode (opens desktop window)
npm run tauri:dev

# Production build
npm run tauri:build
```

## Project Structure

```
src/                      # SvelteKit frontend
  routes/                 # Pages
  lib/
    components/           # Svelte components
    types/health.ts       # Shared TypeScript types
src-tauri/                # Tauri / Rust backend
  src/
    commands/health.rs    # Tauri IPC command handlers
    services/checks/      # Individual health check modules
  tauri.conf.json         # Tauri configuration
```

## License

MIT
