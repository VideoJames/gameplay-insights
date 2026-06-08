# Gameplay Insights

**A "Chrome DevTools for gameplay systems"** — a standalone desktop tool for inspecting,
visualizing, replaying, and analyzing complex, event-driven game behaviour in real time.

> **Status: early development.** The instrumentation SDK is the first component under
> construction. The transport layer and the desktop viewer application are planned but
> not yet started. See the [roadmap](#roadmap) for what exists today versus what's coming.

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

- **Gameplay programmers** — debug logic, inspect state changes, follow event chains.
- **Designers** — validate gameplay rules and tune systems without reading code.
- **QA** — record a session when a bug occurs and hand a reproducible report to a developer.

## Architecture

Three layers connect a running game to the tool:

```
Game
 │  telemetry
 ▼
Instrumentation SDK   ← lightweight, portable API the game calls
 │
 ▼
Transport Layer       ← localhost TCP + JSON (later: binary + compression)
 │
 ▼
Gameplay Insights     ← desktop app: event DB, timeline, visualization, replay
```

## Current state — the instrumentation SDK

The `insights-sdk` crate currently models the core telemetry type, an `Event` with a
small set of typed field values, built fluently:

```rust
use insights_sdk::{Event, FieldValue};

let event = Event::new("PlayerAttack")
    .field("weapon", FieldValue::Text("Sword".into()))
    .field("damage", FieldValue::Int(42));
```

Run the tests:

```sh
cd insights-sdk
cargo test
```

The ergonomic, fully fluent API sketched in the design notes
(`trace_event("PlayerAttack").field("weapon", "Sword")`) is the target the SDK is being
shaped toward.

## Roadmap

Mapped to the project's development milestones. Honest status, updated as work lands.

| Milestone | Scope | Status |
|-----------|-------|--------|
| 1 | Instrumentation SDK · Transport layer · Event timeline | 🚧 In progress (SDK) |
| 2 | Entity inspector · State machine inspector | ⬜ Planned |
| 3 | Event-chain graph visualization | ⬜ Planned |
| 4 | Session recording · Replay (time-travel debugging) | ⬜ Planned |
| 5 | Performance dashboard · Gameplay query language | ⬜ Planned |

### Planned features

- **Event timeline** — chronological feed with filtering (type/entity/system/severity/tag)
  and search (`damage > 100`, `event:EnemyDied`).
- **Event-chain visualization** — interactive cause-and-effect graph.
- **State machine inspector** — current state, time-in-state, transition reasons.
- **Entity inspector** — live state, property-change tracking, diffs, historical snapshots.
- **Time-travel debugging** — record and replay sessions; play/pause/step/scrub; rewind to
  before an event fired.
- **Session recording** — save a `.gpi` session a developer can load and reproduce offline.
- **Performance overlay** — per-system frame cost, event throughput, heatmaps.
- **Gameplay query language** — `event = LootDropped AND rarity = Legendary`.
- **Visual debug dashboard** — rearrangeable, persisted widget layouts.

## Technology

- **Rust** — the implementation language for both the SDK and the tool.
- **egui / eframe** — immediate-mode UI for the desktop application (planned).
- Transport: **localhost TCP + JSON** initially, with a **binary + compressed** protocol
  and remote connections planned.

## UX intent

This is built to feel like a genuine studio tool, not a flashy game UI — closer to Unreal
Insights, RenderDoc, Chrome DevTools, and the Unity Profiler: information-dense,
keyboard-driven, search-first, with dockable panels, persisted layouts, and dark mode.

## Repository layout

```
insights-sdk/   The instrumentation SDK crate (telemetry types and builders)
README.md       This file
```

Additional crates (transport, shared protocol types, the viewer application) will be added
as the project grows toward a Cargo workspace.
