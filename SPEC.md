# Open Agent Session Format (OASF)

Schema `oasf` version **1** (crate 0.1.2). Vendor-neutral JSON for a session's
identity, context, and ordered event stream. Normative Rust types:
[`src/lib.rs`](src/lib.rs).

## Session

```json
{
  "schema": { "name": "oasf", "version": 1 },
  "identity": { "id": "01J…", "title": "Refactor auth" },
  "context": { "workspace": "/repo", "tags": [] },
  "events": [],
  "extensions": {}
}
```

| Field | Required |
| --- | --- |
| `schema` | no (defaults to `oasf` v1) |
| `identity` | yes |
| `context`, `events`, `extensions` | no |

## Event

```json
{
  "id": "evt_1",
  "kind": "message",
  "role": "user",
  "timestamp": "2026-07-31T12:00:00Z",
  "links": {},
  "blocks": [{ "type": "text", "text": "Hello." }],
  "metadata": {}
}
```

| Field | Required |
| --- | --- |
| `id`, `kind`, `role`, `timestamp`, `metadata` | yes |
| `links`, `blocks` | no |

**Kinds:** `message`, `action`, `observation`, `lifecycle`, `other`.

**Roles:** `user`, `assistant`, `tool`, `system`, `developer`, `other`.

## Links

`parent_event_id`, `turn_id`, `turn_outcome` (`completed` | `failed` |
`interrupted` | `incomplete`), `related_event_ids`.

## Blocks

Discriminated by `type`: `text`, `thinking`, `tool_call`, `tool_result`,
`command`, `command_result`, `patch`, `file`, `image`, `compressed`, `other`.

## Metadata

`model` (optional string). `usage` (optional): `input_tokens`, `output_tokens`,
`cache_read_tokens`, `cache_write_tokens`, `reasoning_tokens`.

## Compatibility

Reader and writer must share the session's schema version. Unknown core
`kind`, `role`, or block `type` values are errors. `other` blocks and
`extensions` carry producer-defined data outside the core set.
