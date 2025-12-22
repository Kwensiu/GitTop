# GitTop Feature Documentation

This document provides detailed technical documentation for complex features in GitTop.

---

## Command Center & Rule Engine

### Overview

The **Command Center** is a dedicated settings page that serves as the control hub for GitTop's "Power Mode" - an enterprise-grade notification management system. When Power Mode is enabled, users gain access to the **Rule Engine**, a sophisticated tool for configuring complex notification filtering logic.

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Settings Screen                         │
├─────────────┬─────────────┬─────────────┬────────────┬──────────┤
│ Appearance  │  Behavior   │   Command   │ Notific.   │ Accounts │
│             │             │   Center    │            │          │
└─────────────┴─────────────┴──────┬──────┴────────────┴──────────┘
                                   │
                    ┌──────────────┴──────────────┐
                    │     Command Center Page     │
                    │  ┌────────────────────────┐ │
                    │  │  Power Mode Toggle     │ │
                    │  ├────────────────────────┤ │
                    │  │  [Open Rule Engine]    │ │ ──► Opens new window
                    │  └────────────────────────┘ │
                    └─────────────────────────────┘
                                   │
                    ┌──────────────┴──────────────┐
                    │      Rule Engine Window     │
                    │  (Separate Window)          │
                    │                             │
                    │  • Time-Based Rules         │
                    │  • Account Filters          │
                    │  • Day/Week Scheduling      │
                    │  • Priority Organizations   │
                    │  • Type Suppression         │
                    │  • Combined Logic           │
                    └─────────────────────────────┘
```

### Data Model

#### NotificationRuleSet

The root configuration object persisted in `settings.json`:

```rust
pub struct NotificationRuleSet {
    /// Global enable/disable
    pub enabled: bool,
    /// Time-based quiet hours
    pub time_rules: Vec<TimeRule>,
    /// Per-account filtering
    pub account_rules: Vec<AccountRule>,
    /// Day-of-week scheduling  
    pub schedule_rules: Vec<ScheduleRule>,
    /// Organizations to prioritize or suppress
    pub org_rules: Vec<OrgRule>,
    /// Notification type filtering
    pub type_rules: Vec<TypeRule>,
}
```

#### TimeRule

Defines quiet hours when notifications are silenced:

```rust
pub struct TimeRule {
    pub id: Uuid,
    pub name: String,
    /// Start time (HH:MM in 24h format)
    pub start_time: String,
    /// End time (HH:MM in 24h format)  
    pub end_time: String,
    /// Actions during this period
    pub action: RuleAction,
}
```

#### AccountRule

Per-account notification preferences:

```rust
pub struct AccountRule {
    pub id: Uuid,
    /// GitHub username
    pub account: String,
    /// Show/hide/mute notifications from this account
    pub action: RuleAction,
}
```

#### ScheduleRule

Day-of-week based scheduling:

```rust
pub struct ScheduleRule {
    pub id: Uuid,
    pub name: String,
    /// Days this rule applies (0=Sunday, 6=Saturday)
    pub days: Vec<u8>,
    /// Optional time range within those days
    pub time_range: Option<(String, String)>,
    pub action: RuleAction,
}
```

#### OrgRule

Organization-level priority and filtering:

```rust
pub struct OrgRule {
    pub id: Uuid,
    /// GitHub organization name
    pub org: String,
    /// Priority level (higher = more important)
    pub priority: i32,
    pub action: RuleAction,
}
```

#### TypeRule

Notification type suppression:

```rust
pub struct TypeRule {
    pub id: Uuid,
    /// Notification reason type (mention, review_requested, etc.)
    pub notification_type: String,
    pub action: RuleAction,
}
```

#### RuleDecision

Trace of why a specific rule was applied, used for observability:

```rust
pub struct RuleDecision {
    pub applied_rule_id: String,
    pub action: RuleAction,
    pub priority: i32,
    pub reason: RuleDecisionReason,
}

pub enum RuleDecisionReason {
    TimeRule(String),
    ScheduleRule(String),
    AccountRule(String),
    OrgRule(String),
    TypeRule(String),
}
```

#### RuleAction

Actions determine how a notification is presented to the user.

```rust
pub enum RuleAction {
    /// **Show**: Standard behavior. The notification appears in the list and triggers a system desktop notification.
    Show,

    /// **Silent**: The notification appears in the list but **does not** trigger a desktop notification or sound. Use this for low-priority items you want to see eventually but don't need immediate interruption for.
    Silent,

    /// **Suppress**: The notification is completely hidden from the list. It is effectively "auto-archived" or suppressed from the user's view in GitTop.
    Suppress,

    /// **Priority**: The notification is prominently highlighted (e.g., distinct color/icon) and always triggers a desktop notification, overriding any "Silent" or "Suppress" rules that might otherwise apply.
    Priority,
}
```

#### Priority Values

Rules can be assigned an integer `priority` value to resolve conflicts when multiple rules match the same notification.

- **Range**: Signed 32-bit integer (`i32`).
- **Logic**: Higher values take precedence.
- **Evaluation Order**:
    1.  **Highest Integer**: The rule with the highest `priority` value wins.
    2.  **Highest Logic (Within Band)**: If multiple rules have the *same highest priority*, `Priority` action wins.
    3.  **Suppression**: If no `Priority` action exists at that level, and a rule matches `Suppress`, it hides.
    4.  **Silence**: If no `Priority`/`Suppress`, and winning rule is `Silent`, it silences.
    5.  **Default**: `Show`.

**Priority Enforcement:**
The system "softly" enforces priority values between `-100` and `100`.
- Values outside this range will trigger a **warning** in the UI (⚠️ Non-std).
- **Important**: A `Priority` action rule with lower numeric priority (e.g., -10) will **NOT** override a rule with higher numeric priority (e.g., 50). Numeric priority always determines the "winner" first.

**Standard Priority Levels:**

| Level | Value | Use Case |
| :--- | :--- | :--- |
| **Max** | `100` | Critical overrides (e.g., Security Alerts) |
| **High** | `50` | Important organizational rules |
| **Default** | `0` | Standard user rules |
| **Low** | `-50` | Background tasks, minor bots |
| **Min** | `-100` | Catch-all low priority |


### Rule Evaluation Logic

Rules are evaluated in priority order:

1. **Priority rules always win** - If any rule marks a notification as `Priority`, it shows
2. **Suppress rules evaluated next** - If any matching rule suppresses, notification is hidden
3. **Silent rules** - Notification shows but no desktop alert
4. **Time-based rules checked against current time**
5. **Schedule rules checked against current day/time**
6. **Most specific rule wins** when multiple match

### UI Components

#### Command Center Settings Page

Located at: `src/ui/screens/settings/command_center.rs`

- Power Mode toggle with description
- "Open Rule Engine" button (only visible when Power Mode enabled)
- Quick overview of active rules count

#### Rule Engine Window

Located at: `src/ui/windows/rule_engine.rs`

The Rule Engine opens in a **separate window** to provide a dedicated, spacious interface for complex configuration. The window contains:

- **Tab Navigation**: Time Rules, Accounts, Schedule, Organizations, Types
- **Rule List**: Shows all rules for the selected category
- **Add/Edit Rule**: Modal or inline form for rule creation
- **Preview Panel**: Shows which notifications would be affected
- **Import/Export**: JSON-based rule backup

### Implementation Notes

1. **Window Management**: Uses Iced's multi-window support (`iced::window::spawn`)
2. **State Isolation**: Rule Engine has its own state struct, communicates via messages
3. **Persistence**: Rules saved to `%APPDATA%/gittop/rules.json` (separate from settings)
4. **Evaluation Cache**: Rules compiled into a decision tree on change for O(1) lookup
5. **Hot Reload**: Rule changes take effect immediately without restart

### Future Extensions

- Rule templates ("Focus Mode", "Weekend Mode", etc.)
- Rule import from team/organization
- Analytics: which rules triggered most
- Webhook integration for rule-triggered actions

