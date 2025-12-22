# GitTop Enterprise Evolution Roadmap

This roadmap outlines the strategic technical evolution of GitTop from a single-user notification tool to a robust Enterprise Organization Management suite.

## Core Philosophy
- **Command Center Design**: Enterprise users need a "Cockpit". Top-level controls for broad context (Orgs), side-level for navigation, and detail-level for action.
- **Idiomatic Rust/Iced**: Use the type system for state (Enums for Modes), pure functions for views, and actors/subscriptions for async data.
- **Performance First**: Zero-cost abstractions, aggressive memory trimming, and smart caching.

## Phase 1: Foundation (Data & State)
*Goal: Decouple the application from a single-user context and introduce robust data handling.*

- [x] **Multi-Account Architecture**
    - Refactor `AuthManager` to `AccountKeyring` (secure multiple token storage).
    - Create `SessionManager` struct to hold active `GitHubClient` instances for multiple accounts simultaneously.
- [/] **Smart Caching Layer**
    - [ ] Implement a write-through cache for GitHub responses (`ETag` support).
    - [ ] **Thread Skimming**: Background task that fetches *metadata only* to check for updates.
    - [x] Disk Persistence: Cache critical state (read status) to `sled`.

## Phase 2: Power Mode UI (The Cockpit)
*Goal: Introduce the "Enterprise" layout capabilities as a transformation of the UI.*

- [x] **Power Mode Engine**
    - `enum LayoutMode { Simple, Enterprise }`.
    - Toggle implementation in Settings.
- [x] **The Top Bar (High-Level Control)**
    - Global Account/Org Switcher.
    - "Omnibar" placeholder for command-palette style actions.
- [x] **The Right Panel (Details/Triage)**
    - Specific "Inspector" view for the active item.
    - Allows skimming through list without losing context (Outlook/VSCode style).
- [x] **The Bottom Bar (System State)**
    - Status indicators, sync time, active background tasks.

## Phase 3: Organization Intelligence
*Goal: Add "Org Tool" capabilities as a specialized view.*

- [ ] **Org Data Model**
    - Hierarchy mapping: `Org -> Team -> Repository` tree.
- [ ] **Organization Dashboard**
    - A specific `Content` view.
    - "Team Pulse": Activity heatmap.
    - "Mention Matrix": See where you are needed.
- [ ] **Smart Filtering**:
    - "Focus Mode": Filter inbox by specific Org Context.

## Phase 4: Performance & Refinement
*Goal: Handle "Heavy flows" gracefully.*

- [ ] **Virtualized Lists**
    - Implement `iced::widget::virtual_scroll` for notification lists.
- [ ] **Memory Hygiene**
    - **Resource Pools**: Reuse allocation buffers.
    - **Image Caching**: Global LRU cache.