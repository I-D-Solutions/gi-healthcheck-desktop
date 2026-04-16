# Software Architecture

> Current state of the GI Health Check desktop application.

```mermaid
flowchart TB
    subgraph Desktop ["Tauri Desktop Shell"]
        main["main.rs"] --> lib["lib.rs"]
        lib --> commands
        lib --> plugins["tauri-plugin-log"]

        subgraph commands ["Commands (IPC)"]
            healthCmd["health.rs\nrun_health_checks()"]
        end
    end

    subgraph Frontend ["SvelteKit Frontend"]
        layout["+layout.svelte\nShell / Sidebar"]
        page["+page.svelte"]
        page --> dashboard

        subgraph dashboard ["Dashboard"]
            HealthCheckDashboard["HealthCheckDashboard.svelte"]
            CategorySection["CategorySection.svelte"]
            CheckCard["CheckCard.svelte"]
            StatusBadge["StatusBadge.svelte"]
        end

        types["health.ts\nCheckResult / groupByCategory"]
    end

    subgraph Checks ["Health Check Modules (Rust)"]
        mod_rs["mod.rs\nrun_all_checks()"]

        subgraph SystemChecks ["System Checks"]
            cpu["cpu.rs"]
            memory["memory.rs"]
            disk["disk.rs"]
            runtime["runtime.rs"]
        end

        subgraph NetworkChecks ["Network Checks"]
            network["network.rs\nDNS + Reachability"]
            ethernet["ethernet.rs\nInterface Enumeration"]
        end

        subgraph PodSubsystems ["Pod Subsystem Checks"]
            iff["iff.rs"]
            gps["gps.rs\n+ NMEA detection"]
            link16["link16.rs\n+ TCP probe"]
            securelink["securelink.rs"]
        end

        subgraph Helpers ["Shared Helpers"]
            serial["serial.rs\nPort enumeration"]
            process_mod["process.rs\nProcess monitoring"]
            manifest["manifest.rs\nTOML loader"]
        end

        config["expected-devices.toml"]
    end

    HealthCheckDashboard -->|"invoke('run_health_checks')"| healthCmd
    healthCmd --> mod_rs

    mod_rs --> cpu
    mod_rs --> memory
    mod_rs --> disk
    mod_rs --> runtime
    mod_rs --> network
    mod_rs --> ethernet
    mod_rs --> iff
    mod_rs --> gps
    mod_rs --> link16
    mod_rs --> securelink

    manifest --> config
    iff --> serial
    iff --> process_mod
    iff --> manifest
    gps --> serial
    gps --> process_mod
    gps --> manifest
    link16 --> serial
    link16 --> process_mod
    link16 --> manifest
    securelink --> process_mod
    securelink --> manifest

    healthCmd -->|"Vec of CheckResult"| HealthCheckDashboard
    HealthCheckDashboard --> CategorySection
    CategorySection --> CheckCard
    CheckCard --> StatusBadge
    HealthCheckDashboard --> types
```

## Layer Summary

| Layer | Technology | Role |
|-------|-----------|------|
| Desktop Shell | Tauri v2 (Rust) | Native window, IPC, packaging |
| Frontend | SvelteKit + TypeScript | Dashboard UI, result rendering |
| Commands | Tauri IPC | Bridge between UI and Rust checks |
| System Checks | Rust (sysinfo) | CPU, memory, disk, hostname, OS |
| Network Checks | Rust (tokio, sysinfo) | DNS, internet reachability, interface enumeration |
| Pod Subsystems | Rust (serialport, tokio) | IFF, GPS, Link 16, Secure Link |
| Helpers | Rust (serialport, sysinfo, toml) | Serial port, process, manifest utilities |
| Configuration | TOML | Expected device manifest |

## Data Flow

1. User clicks **Run Checks** in the SvelteKit dashboard
2. Frontend calls `invoke('run_health_checks')` via Tauri IPC
3. Rust `run_all_checks()` executes all check modules sequentially
4. Pod subsystem checks load `expected-devices.toml` and validate against actual hardware
5. Results return as `Vec<CheckResult>` with pass/warn/fail status
6. Frontend groups results by category and renders them dynamically
