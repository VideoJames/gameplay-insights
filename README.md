# Gameplay Insights

**A "Chrome DevTools for gameplay systems".** A standalone desktop tool for inspecting,
visualizing, replaying, and analyzing complex, event-driven game behaviour in real time.

> **Status: early development.** The instrumentation SDK and the transport layer are
> built and working. Events now cross a real process boundary over a socket. The
> desktop viewer application is next. See the [roadmap](#roadmap) for what exists today
> versus what's coming.

## The problem

Modern games lean on event-driven architectures, state machines, AI behaviours, gameplay
tags, abilities, buffs, quests, and networked interactions. As these systems grow,
answering *why a particular outcome occurred* gets hard. Gameplay Insights exists to make
that answerable:

- Why did this happen?
- What caused this event?
- Which systems participated?
- What changed?
- Can I reproduce it?

## Who it's for

- **Gameplay programmers**: debug logic, inspect state changes, follow event chains.
- **Designers**: validate gameplay rules and tune systems without reading code.
- **QA**: record a session when a bug occurs and hand a reproducible report to a developer.

## Architecture

Three layers connect a running game to the tool:

```
Game
 │  telemetry
 ▼
Instrumentation SDK   ✅ lightweight, portable API the game calls
 │
 ▼
Transport Layer       ✅ length-prefixed JSON frames over localhost TCP
 │                       (later: binary + compression)
 ▼
Gameplay Insights     ⬜ desktop app: event DB, timeline, visualization, replay
```

The first two layers are built. The desktop viewer is the next milestone.

## Current state: SDK and transport, end to end

The project is a Cargo workspace with three crates: the `insights-sdk` library and two
binaries that exercise it across a real socket.

**The SDK** models the three core telemetry types (`Event`, `StateTransition`, and
`EntitySnapshot`) over a shared `FieldValue` sum type (`Text`/`Int`/`Float`/`Bool`),
Rust's typed answer to a `Dictionary<string, object>`. `Event` and `EntitySnapshot` are
built fluently:

```rust
use insights_sdk::{Event, FieldValue};

let event = Event::new("PlayerAttack")
    .field("weapon", FieldValue::Text("Sword".into()))
    .field("damage", FieldValue::Int(42));
```

**The transport layer** wraps each message in an `Envelope { timestamp, payload }`, where
the `Payload` enum's variant name is the type discriminator on the wire, and frames it
as a big-endian `u32` length followed by the JSON body, so the receiver always knows
exactly how many bytes to read. `write_frame`/`read_frame` are generic over
`std::io::Write`/`Read` rather than `std::net` directly, so the same code will serve
socket traffic today and `.gpi` file replay later.

### Run the demo

Two processes talk over loopback TCP. Start the receiver first, then the emitter:

```sh
# terminal 1: the tool (server, waits for a connection)
cargo run -p gameplay-analysis

# terminal 2: a stand-in game (client, emits two events then exits)
cargo run -p dummy-game
```

The receiver prints each decoded `Envelope` in order and reports a clean disconnect on
EOF. Run the tests with `cargo test`.

## Roadmap

Mapped to the project's development milestones. Honest status, updated as work lands.

| Milestone | Scope | Status |
|-----------|-------|--------|
| 1 | Instrumentation SDK · Transport layer · Event timeline | 🚧 SDK ✅ · Transport ✅ · Timeline next |
| 2 | Entity inspector · State machine inspector | ⬜ Planned |
| 3 | Event-chain graph visualization | ⬜ Planned |
| 4 | Session recording · Replay (time-travel debugging) | ⬜ Planned |
| 5 | Performance dashboard · Gameplay query language | ⬜ Planned |

### Planned features

- **Event timeline**: chronological feed with filtering (type/entity/system/severity/tag)
  and search (`damage > 100`, `event:EnemyDied`).
- **Event-chain visualization**: interactive cause-and-effect graph.
- **State machine inspector**: current state, time-in-state, transition reasons.
- **Entity inspector**: live state, property-change tracking, diffs, historical snapshots.
- **Time-travel debugging**: record and replay sessions; play/pause/step/scrub; rewind to
  before an event fired.
- **Session recording**: save a `.gpi` session a developer can load and reproduce offline.
- **Performance overlay**: per-system frame cost, event throughput, heatmaps.
- **Gameplay query language**: `event = LootDropped AND rarity = Legendary`.
- **Visual debug dashboard**: rearrangeable, persisted widget layouts.

## Technology

- **Rust**: the implementation language for both the SDK and the tool.
- **egui / eframe**: immediate-mode UI for the desktop application (planned).
- Transport: **localhost TCP + JSON** initially, with a **binary + compressed** protocol
  and remote connections planned.

## UX intent

This is built to feel like a genuine studio tool in the realm of Unreal Insights, Chrome
DevTools, and the Unity Profiler: information-dense, keyboard-driven, search-first, with
dockable panels, persisted layouts, and dark mode.

## Repository layout

A Cargo workspace:

```
insights-sdk/        Library crate: telemetry types, builders, and the
                       transport layer (transport.rs: framing + (de)serialization)
dummy-game/          Binary: a stand-in game that emits events (TCP client)
gameplay-analysis/   Binary: the tool, currently a headless receiver (TCP server)
Cargo.toml           Workspace manifest
README.md            This file
```

The viewer application (egui/eframe) will grow inside `gameplay-analysis` as the desktop
UI comes online. A shared protocol crate may be split out if the wire format warrants it.
