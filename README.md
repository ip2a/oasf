<h1 align="center">OASF</h1>
<p align="center"><strong>Open Agent Session Format</strong></p>

<p align="center">
Vendor-neutral JSON for an AI agent session.<br>
Crate <code>0.2.0</code> · schema <code>oasf</code> v2
</p>

## Install

```toml
[dependencies]
oasf = "0.2.0"
chrono = "0.4"
serde_json = "1"
```

## Example

```rust
use oasf::{Block, Event, EventKind, Identity, Metadata, Role, Schema, Session};

let session = Session {
    schema: Schema::default(),
    identity: Identity { id: "01J…".into(), title: None },
    lineage: vec![],
    context: Default::default(),
    events: vec![Event {
        id: "evt_1".into(),
        kind: EventKind::Message,
        role: Role::User,
        timestamp: chrono::Utc::now(),
        links: Default::default(),
        blocks: vec![Block::Text { text: "Hello.".into() }],
        metadata: Metadata { model: None, usage: None },
    }],
    extensions: Default::default(),
};

println!("{}", serde_json::to_string_pretty(&session).unwrap());
```

Format details: [`SPEC.md`](SPEC.md). Migrating from schema v1: [`MIGRATION.md`](MIGRATION.md). License: Apache-2.0.
