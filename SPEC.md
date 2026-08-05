# Open Agent Session Format (OASF)

Schema `oasf` version **2** (crate 0.2.0). Vendor-neutral JSON for a session's
identity, context, lineage, and ordered event stream. Normative Rust types:
[`src/lib.rs`](src/lib.rs).

## Session

```json
{
  "schema": { "name": "oasf", "version": 2 },
  "identity": { "id": "01J…", "title": "Refactor auth" },
  "context": { "workspace": "/repo", "tags": [] },
  "events": [],
  "extensions": {}
}
```

| Field | Required |
| --- | --- |
| `schema` | no (defaults to `oasf` v2) |
| `identity` | yes |
| `lineage`, `context`, `events`, `extensions` | no |

`lineage` is omitted when empty. When absent, it is equivalent to an empty
list.

## Session lineage

`lineage` is an optional list of typed relationships to other sessions.

```json
{
  "lineage": [
    {
      "type": "forked_from",
      "session_id": "sess_parent",
      "at_event_id": "evt_12",
      "at_turn_id": "turn_4"
    },
    {
      "type": "spawned_by",
      "session_id": "sess_parent",
      "at_tool_call_id": "call_7"
    }
  ]
}
```

### `forked_from`

The current session branched from the source session's history. The optional
`at_event_id` and `at_turn_id` identify the history boundary at which the fork
was created.

```json
{
  "type": "forked_from",
  "session_id": "sess_parent",
  "at_event_id": "evt_12",
  "at_turn_id": "turn_4"
}
```

### `spawned_by`

The current session was started as a delegated child session by another
session. The optional `at_tool_call_id` identifies the parent invocation that
created or launched it.

```json
{
  "type": "spawned_by",
  "session_id": "sess_parent",
  "at_tool_call_id": "call_7"
}
```

`forked_from` and `spawned_by` are independent relationships. A child session
may contain both, even when both relationships point to the same session.

`derived_from` is not a Core relation in schema v2 because its meaning is too
broad without a derivation type. `revert` is a history or workspace operation
inside a session, not a Session lineage relation.

## Event

```json
{
  "id": "evt_1",
  "kind": "message",
  "role": "user",
  "timestamp": "2026-07-31T12:00:00Z",
  "links": {},
  "blocks": [{ "type": "text", "text": "Hello." }],
  "metadata": {},
  "tags": [],
  "extensions": {}
}
```

| Field | Required |
| --- | --- |
| `id`, `kind`, `role`, `timestamp`, `metadata` | yes |
| `links`, `blocks`, `tags`, `extensions` | no |

**Kinds:** `message`, `action`, `observation`, `lifecycle`, `other`.

**Roles:** `user`, `assistant`, `tool`, `system`, `developer`, `other`.

## Links

`parent_event_id`, `turn_id`, `turn_outcome` (`completed` | `failed` |
`interrupted` | `incomplete`), `related_event_ids`.

`parent_event_id` links events within the event stream. It does not replace or
represent a cross-session lineage relation.

## Blocks

Blocks are discriminated by the `type` field:

```text
text | thinking | tool_call | tool_result | command | command_result |
patch | file | image | compressed | other
```

### `tool_call`

A request to invoke an Agent or model capability.

```json
{
  "type": "tool_call",
  "tool_call_id": "call_1",
  "name": "search",
  "input": { "query": "..." }
}
```

`input` is optional because some tools take no structured input.

### `tool_result`

The terminal result associated with a `tool_call`.

```json
{
  "type": "tool_result",
  "tool_call_id": "call_1",
  "content": "ok",
  "outcome": "succeeded"
}
```

`outcome` is one of:

```text
succeeded | failed | cancelled | declined | timed_out | unknown
```

Schema v2 uses `outcome` instead of the v1 boolean `is_error`. Runtime states
such as `scheduled`, `awaiting_approval`, `executing`, `streaming`, and
`retrying` are not Core `tool_result` outcomes; producers may preserve them in
lifecycle events or extensions.

### `command`

A request to execute a process or command. A Command is independent of
`tool_call`; `tool_call_id` is present only when a Tool invocation caused the
Command.

```json
{
  "type": "command",
  "command_id": "cmd_1",
  "command": "git status",
  "argv": [],
  "cwd": "/workspace",
  "tool_call_id": "call_1"
}
```

| Field | Required |
| --- | --- |
| `command_id`, `command` | yes |
| `argv`, `cwd`, `tool_call_id` | no |

`command` is the command text. `argv` is an optional argument vector for
process-oriented execution. `cwd` is the optional working directory.
`tool_call_id` is only a correlation field; it does not make Command a subtype
of ToolCall.

A Command may also be emitted directly by the Agent runtime without any
ToolCall.

### `command_result`

The result associated with a `command`.

```json
{
  "type": "command_result",
  "command_id": "cmd_1",
  "exit_code": 0,
  "stdout": "clean",
  "stderr": null
}
```

| Field | Required |
| --- | --- |
| `command_id` | yes |
| `exit_code`, `stdout`, `stderr` | no |

The result is correlated by `command_id`; the command text is not duplicated in
`command_result`.

### Shell and execution environment

Shell is not a required Core Block. It may describe the execution mechanism
when that detail matters, for example an interpreter such as `bash` or a
persistent terminal session. Direct process execution may have no Shell at all.
Provider-specific Shell executable, version, terminal session, environment,
and scheduler details belong in extensions or runtime metadata.

## Metadata

`model` (optional string). `usage` (optional): `input_tokens`, `output_tokens`,
`cache_read_tokens`, `cache_write_tokens`, `reasoning_tokens`.

## Compatibility

Schema v2 is a breaking Core schema revision from v1. See [`MIGRATION.md`](MIGRATION.md) for conversion steps.


- v2 readers and writers must agree on `schema.version`;
- `command` and `command_result` use `command_id` correlation;
- `tool_result` uses terminal `outcome` instead of `is_error`;
- `lineage` is optional and may be omitted when empty.

Unknown Core `kind`, `role`, block `type`, outcome, or lineage relation values
are errors. `other` blocks and `extensions` carry producer-defined data outside
the Core set.
