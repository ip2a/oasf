# Migrating OASF schema v1 to v2

Release date: 2026-08-05

OASF schema v2 is a breaking revision. It adds stable Command correlation,
terminal Tool outcomes, and typed Session lineage. This guide covers only the
changes required to convert v1 data and integrations to v2.

## Version changes

| Component | Before | After |
| --- | --- | --- |
| OASF schema | v1 | v2 |
| Rust crate | 0.1.x | 0.2.0 |

A converted document must set:

```json
{
  "schema": { "name": "oasf", "version": 2 }
}
```

Do not label unchanged v1 data as schema v2.

## 1. ToolResult: replace `is_error` with `outcome`

### v1

```json
{
  "type": "tool_result",
  "tool_call_id": "call_1",
  "content": "ok",
  "is_error": false
}
```

### v2

```json
{
  "type": "tool_result",
  "tool_call_id": "call_1",
  "content": "ok",
  "outcome": "succeeded"
}
```

Default conversion:

| v1 | v2 |
| --- | --- |
| `is_error: false` | `outcome: "succeeded"` |
| `is_error: true` | `outcome: "failed"` |

When the source contains more precise information, use `cancelled`, `declined`,
`timed_out`, or `unknown` instead of reducing it to `failed`.

## 2. Command: add `command_id`

### v1

```json
{
  "type": "command",
  "command": "git status",
  "cwd": "/workspace"
}
```

### v2

```json
{
  "type": "command",
  "command_id": "cmd_1",
  "command": "git status",
  "cwd": "/workspace"
}
```

Every Command requires a stable `command_id` within its session. Reuse a source
execution ID when one exists; otherwise generate a new ID.

If the Command was caused by a ToolCall, add its existing `tool_call_id`:

```json
{
  "type": "command",
  "command_id": "cmd_1",
  "command": "git status",
  "tool_call_id": "call_1"
}
```

Do not invent a ToolCall relationship for runtime-originated Commands.

## 3. CommandResult: correlate by `command_id`

### v1

```json
{
  "type": "command_result",
  "command": "git status",
  "exit_code": 0,
  "stdout": "clean"
}
```

### v2

```json
{
  "type": "command_result",
  "command_id": "cmd_1",
  "exit_code": 0,
  "stdout": "clean"
}
```

Remove the duplicated `command` field and use the ID of the corresponding
Command.

When converting historical data:

1. Prefer an existing provider execution ID.
2. Otherwise pair requests and results only when the relationship is
   unambiguous.
3. If multiple identical or concurrent Commands make pairing ambiguous, keep
   the source as v1 or preserve the unresolved source data in an extension;
   do not guess and publish a false v2 relationship.

## 4. Session: add lineage only when known

Schema v2 accepts an optional `lineage` list. It may be omitted when the Session
has no known cross-session relationship.

### Forked history

```json
{
  "type": "forked_from",
  "session_id": "sess_parent",
  "at_event_id": "evt_12",
  "at_turn_id": "turn_4"
}
```

### Delegated child session

```json
{
  "type": "spawned_by",
  "session_id": "sess_parent",
  "at_tool_call_id": "call_7"
}
```

Do not infer Session lineage from `parent_event_id`; that field links events
inside a session. Do not encode `revert` as lineage. Keep untyped
`derived_from` data in extensions until its derivation semantics are known.

## 5. Rust API changes

Update the dependency:

```toml
[dependencies]
oasf = "0.2.0"
```

Update struct and enum construction sites:

- add `Session.lineage`;
- add `command_id` and optional `tool_call_id` to `Block::Command`;
- replace `command` with `command_id` in `Block::CommandResult`;
- replace `is_error` with `ExecutionOutcome` in `Block::ToolResult`.

## What v2 does not add

Schema v2 does not define:

- a universal Tool runtime state machine;
- Shell as a required Core Block;
- `derived_from` as a Core Session relation;
- `revert` as Session lineage;
- a requirement that every Command belongs to a ToolCall.

See [`SPEC.md`](SPEC.md) for the normative schema definition.
