# Migrating OASF schema v1 to v2

OASF schema v2 is a breaking revision. The Rust crate changes from 0.1.x to
0.2.0.

Converted sessions must declare:

```json
{ "schema": { "name": "oasf", "version": 2 } }
```

Do not relabel unchanged v1 data as v2.

## ToolResult

Replace `is_error` with `outcome`:

| v1 | v2 |
| --- | --- |
| `is_error: false` | `outcome: "succeeded"` |
| `is_error: true` | `outcome: "failed"` |

Use `cancelled`, `declined`, `timed_out`, or `unknown` when the source provides
that more precise result.

```json
{
  "type": "tool_result",
  "tool_call_id": "call_1",
  "content": "ok",
  "outcome": "succeeded"
}
```

## Command

Add a stable `command_id`. Add `tool_call_id` only when a ToolCall caused the
Command.

```json
{
  "type": "command",
  "command_id": "cmd_1",
  "command": "git status",
  "cwd": "/workspace",
  "tool_call_id": "call_1"
}
```

Reuse a provider execution ID when available; otherwise generate one within the
session.

## CommandResult

Remove the duplicated `command` field and correlate with `command_id`:

```json
{
  "type": "command_result",
  "command_id": "cmd_1",
  "exit_code": 0,
  "stdout": "clean"
}
```

For historical data, pair requests and results only when the relationship is
unambiguous. Do not guess across identical or concurrent Commands; keep such
data as v1 or preserve the unresolved source in an extension.

## Session lineage

Add `lineage` only when the relationship is known:

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

Do not infer lineage from `parent_event_id`. Do not encode `revert` or an
untyped `derived_from` as Core lineage.

## Rust API

```toml
[dependencies]
oasf = "0.2.0"
```

Update construction sites to:

- add `Session.lineage`;
- add `command_id` and optional `tool_call_id` to `Block::Command`;
- replace `command` with `command_id` in `Block::CommandResult`;
- replace `is_error` with `ExecutionOutcome` in `Block::ToolResult`.

Schema v2 does not add a universal Tool runtime state machine or require Shell
as a Core Block. See [`SPEC.md`](SPEC.md) for the normative definition.
