---
name: Antigravity Redacted OAuth Record Fixture
description: Redacted shape of a user-supplied Antigravity OAuth record
type: fixture
date: 2026-06-18
status: redacted_shape_only
---

# Antigravity Redacted OAuth Record Fixture

This fixture records only the credential shape. It does not approve runtime
execution and does not contain raw tokens.

```json
{
  "access_token": "<redacted>",
  "disabled": false,
  "email": "<redacted-email>",
  "expired": "2026-06-18T20:02:01+08:00",
  "expires_in": 3599,
  "project_id": "keen-host-1gdxg",
  "refresh_token": "<redacted>",
  "timestamp": 1781780522303,
  "type": "antigravity"
}
```

## Use

- Validates the imported record shape for `type = antigravity`.
- Confirms the record carries `project_id`, expiry metadata, and refresh-token
  presence.
- Does not prove endpoint compatibility or runtime response shape.

## Security Note

The original pasted token values must be treated as compromised and rotated or
revoked outside this repository.
