# Solution Architecture

> Synthesized from the GI Orchestrator solution architecture diagram.
> The **GI Health Check** app runs on the **Computer + Certified Software** inside the Pod.

![Reference diagram](solution-architecture-reference.png)

## Mermaid Diagram

```mermaid
flowchart TB

    %% ── External RF / Satellite Sources ──
    GBAD["GBAD"]
    GPSSat["GPS Satellite"]
    Link16Net["Link 16 Network"]

    %% ── Pod ──
    subgraph Pod [Pod]

        subgraph Antennas [Antennas]
            GBADAnt["GBAD Antenna"]
            GPSAnt["GPS Antenna"]
            Link16Ant["Link 16 Antenna"]
        end

        subgraph Cosite [Cosite]
            CositeUnit["Cosite Unit"]
        end

        subgraph Computer ["Computer + Certified Software"]
            IFF["IFF"]
            GPS["GPS Receiver"]
            Link16["Link 16 Terminal"]
            SecureLink["Secure Link"]
            HealthCheck["GI Health Check App"]
        end
    end

    %% ── Pilot Interface ──
    subgraph PilotIF ["Pilot I/F (Phase 2)"]
        PilotDevice["Handheld Device"]
    end

    %% ── Ground Support (GFE) ──
    subgraph GroundSupport ["Ground Support (GFE)"]
        CryptoLoad["Crypto Load"]
        GFE_IFF["IFF Ground Unit"]
        GFE_Link16["Link 16 Ground Unit"]
    end

    %% ── RF links from external sources to antennas ──
    GBAD -.->|RF| GBADAnt
    GPSSat -.->|RF| GPSAnt
    Link16Net -.->|RF| Link16Ant

    %% ── Antenna to Cosite (RF cable) ──
    GBADAnt <-->|RF Cable| CositeUnit
    GPSAnt <-->|RF Cable| CositeUnit
    Link16Ant <-->|RF Cable| CositeUnit

    %% ── Cosite to Computer subsystems (Serial/Ethernet) ──
    CositeUnit <-->|Serial/Ethernet| IFF
    CositeUnit <-->|Serial/Ethernet| GPS
    CositeUnit <-->|Serial/Ethernet| Link16

    %% ── Secure Link to Pilot (RF, Phase 2) ──
    SecureLink -.->|"RF (Phase 2)"| PilotDevice

    %% ── Ground Support connections (RF) ──
    CryptoLoad -.->|RF| IFF
    CryptoLoad -.->|RF| Link16
    GFE_IFF <-.->|RF| IFF
    GFE_Link16 <-.->|RF| Link16

    %% ── Health Check monitors the computer subsystems ──
    HealthCheck ---|monitors| IFF
    HealthCheck ---|monitors| GPS
    HealthCheck ---|monitors| Link16
    HealthCheck ---|monitors| SecureLink
```

## Interface Legend

| Line Style | Interface Type |
|------------|---------------|
| `<-->` solid | Serial / Ethernet |
| `-.->` dashed | RF |
| `<===>` bold | RF Cable |
| `---` plain | Internal monitoring |

## Notes

- **No Pilot I/F Secure Link in Phase 1** -- the Pilot handheld connection is deferred.
- The GI Health Check app runs directly on the Pod computer alongside the certified mission software.
- It monitors IFF, GPS, Link 16, and Secure Link subsystems via Serial/Ethernet interfaces.
- Ground Support equipment (Crypto Load, IFF, Link 16) are Government Furnished Equipment (GFE) connected via RF.
