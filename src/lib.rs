//! Open Agent Session Format (OASF).
//!
//! Vendor-neutral schema for an AI agent session as an ordered stream of
//! events. Each event has a kind, role, timestamp, optional [`Links`],
//! structured [`Block`]s, and [`Metadata`]. Sessions pin a schema version;
//! unrecognized enum or block-type values are errors.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// Schema identifier every OASF session declares.
pub const OASF_SCHEMA_NAME: &str = "oasf";

/// Current OASF schema version (single incrementing integer; see SPEC §13).
pub const OASF_SCHEMA_VERSION: u32 = 1;

/// Schema name and version a session conforms to.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Schema {
    pub name: String,
    pub version: u32,
}

impl Default for Schema {
    fn default() -> Self {
        Self {
            name: OASF_SCHEMA_NAME.to_string(),
            version: OASF_SCHEMA_VERSION,
        }
    }
}

/// A complete agent session: schema, identity, context, and ordered events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Session {
    #[serde(default)]
    pub schema: Schema,
    pub identity: Identity,
    #[serde(default)]
    pub context: Context,
    #[serde(default)]
    pub events: Vec<Event>,
    #[serde(default)]
    pub extensions: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Identity {
    /// Globally unique session id (ULID or UUID recommended).
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Optional ambient context: workspace, timing, tags.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Context {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_active_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// One entry in the session event stream.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub kind: EventKind,
    pub role: Role,
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    pub links: Links,
    #[serde(default)]
    pub blocks: Vec<Block>,
    pub metadata: Metadata,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extensions: BTreeMap<String, Value>,
}

/// Coarse event category; specific payload lives in [`Block`]s.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    Message,
    Action,
    Observation,
    Lifecycle,
    Other,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
    Tool,
    System,
    Developer,
    Other,
}

/// Causal ancestry and turn structure for an event.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Links {
    /// Parent event in the causal tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_event_id: Option<String>,
    /// Shared by every event in the same turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_id: Option<String>,
    /// Present on the last event of a turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_outcome: Option<TurnOutcome>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_event_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TurnOutcome {
    Completed,
    Failed,
    Interrupted,
    Incomplete,
}

/// Per-event model identity and token usage.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Metadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// Token counts for an event. `input_tokens` is non-cached input;
/// `input_tokens + cache_read_tokens + cache_write_tokens` is total input.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Usage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_read_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write_tokens: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u64>,
}

/// Typed content fragment within an event. Discriminated by `type`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Text {
        text: String,
    },
    Thinking {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        signature: Option<String>,
    },
    ToolCall {
        tool_call_id: String,
        name: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        input: Option<Value>,
    },
    ToolResult {
        tool_call_id: String,
        content: String,
        #[serde(default)]
        is_error: bool,
    },
    Patch {
        #[serde(skip_serializing_if = "Option::is_none")]
        summary: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        diff_text: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        files: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        hash: Option<String>,
    },
    Command {
        command: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        argv: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        cwd: Option<String>,
    },
    CommandResult {
        #[serde(skip_serializing_if = "Option::is_none")]
        command: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        exit_code: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        stdout: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        stderr: Option<String>,
    },
    File {
        path: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        content: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        mime_type: Option<String>,
    },
    Image {
        mime_type: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        path: Option<String>,
    },
    /// Compressed prior context; `raw` is producer-defined.
    Compressed {
        raw: Value,
    },
    /// Escape hatch for non-standard content; `raw` carries the payload.
    Other {
        raw: Value,
    },
}
