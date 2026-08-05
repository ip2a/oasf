# Open Agent Session Format (OASF)

Schema `oasf` version **2** (crate 0.2.0). Normative Rust types are in
[`src/lib.rs`](src/lib.rs).

## Session

```json
{
  "schema": { "name": "oasf", "version": 2 },
  "identity": { "id": "sess_1", "title": "Refactor auth" },
  "context": { "workspace": "/repo" },
  "events": [],
  "extensions": {}
}
```

| Field | Required | Meaning |
| --- | --- | --- |
| `identity` | yes | Session identity |
| `schema` | no | Defaults to `oasf` v2 |
| `lineage` | no | Cross-session relationships |
| `context` | no | Workspace, timestamps, tags |
| `events` | no | Ordered event stream |
| `extensions` | no | Producer-defined data |

Empty `lineage` is omitted.

## Session lineage

| Type | Required fields | Optional boundary | Meaning |
| --- | --- | --- | --- |
| `forked_from` | `session_id` | `at_event_id`, `at_turn_id` | History branch source |
| `spawned_by` | `session_id` | `at_tool_call_id` | Delegating parent session |

```json
{
  "lineage": [
    {
      "type": "forked_from",
      "session_id": "sess_parent",
      "at_event_id": "evt_12"
    },
    {
      "type": "spawned_by",
      "session_id": "sess_parent",
      "at_tool_call_id": "call_7"
    }
  ]
}
```

The two relationships are independent. `derived_from` and `revert` are not
Core lineage relations in schema v2.

## Event

| Field | Required | Meaning |
| --- | --- | --- |
| `id`, `kind`, `role`, `timestamp`, `metadata` | yes | Event identity and classification |
| `links`, `blocks`, `tags`, `extensions` | no | Structure and content |

**Kinds:** `message`, `action`, `observation`, `lifecycle`, `other`.

**Roles:** `user`, `assistant`, `tool`, `system`, `developer`, `other`.

`links` may contain `parent_event_id`, `turn_id`, `turn_outcome`, and
`related_event_ids`. `parent_event_id` links events within one session; it is
not Session lineage.

## Blocks

Blocks use a `type` discriminator:

```text
text | thinking | tool_call | tool_result | command | command_result |
patch | file | image | compressed | other
```

### ToolCall

| Field | Required |
| --- | --- |
| `tool_call_id`, `name` | yes |
| `input` | no |

### ToolResult

| Field | Required |
| --- | --- |
| `tool_call_id`, `content`, `outcome` | yes |

`outcome` is one of:

```text
succeeded | failed | cancelled | declined | timed_out | unknown
```

Runtime states such as approval, scheduling, execution, streaming, and retry
belong in lifecycle events or extensions.

### Command

| Field | Required |
| --- | --- |
| `command_id`, `command` | yes |
| `argv`, `cwd`, `tool_call_id` | no |

A Command is an independent process-execution record. `tool_call_id` is present
only when a ToolCall caused it. Runtime-originated Commands may omit it.

### CommandResult

| Field | Required |
| --- | --- |
| `command_id` | yes |
| `exit_code`, `stdout`, `stderr` | no |

A result is correlated by `command_id`; command text is not duplicated.

Shell is not a required Core Block. Interpreter, terminal, environment, and
scheduler details belong in extensions or runtime metadata.

## Metadata

`model` is optional. `usage` may contain `input_tokens`, `output_tokens`,
`cache_read_tokens`, `cache_write_tokens`, and `reasoning_tokens`.

## Compatibility

Schema v2 is a breaking revision from v1:

- Command request/result correlation uses `command_id`;
- ToolResult uses `outcome` instead of `is_error`;
- Session may contain typed `lineage`.

Readers and writers must agree on `schema.version`. Unknown Core enum and block
type values are errors. Use `other` and `extensions` for non-Core data. See
[`MIGRATION.md`](MIGRATION.md) for conversion steps.
