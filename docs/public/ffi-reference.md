---
status: stable
---

# FFI Reference

AgenticContract provides a C FFI for embedding the policy engine in non-Rust applications.

## Error Codes

| Code | Name | Description |
|------|------|-------------|
| 0 | `ACON_OK` | Success |
| 1 | `ACON_ERR_NULL_PTR` | Null pointer argument |
| 2 | `ACON_ERR_INVALID_UTF8` | Invalid UTF-8 string |
| 3 | `ACON_ERR_NOT_FOUND` | Contract or entity not found |
| 4 | `ACON_ERR_IO` | I/O error |
| 5 | `ACON_ERR_FORMAT` | File format error |
| 6 | `ACON_ERR_POLICY_VIOLATION` | Policy violation |
| 7 | `ACON_ERR_RISK_EXCEEDED` | Risk limit exceeded |
| 8 | `ACON_ERR_APPROVAL_REQUIRED` | Approval required |
| 9 | `ACON_ERR_INTERNAL` | Internal error |

## Functions

### `acon_create`

Create a new contract engine with an empty contract.

```c
int32_t acon_create(AconHandle **out);
```

**Parameters:**
- `out` — Pointer to receive the new handle

**Returns:** `ACON_OK` on success, error code on failure.

### `acon_open`

Open an existing `.acon` file.

```c
int32_t acon_open(const char *path, AconHandle **out);
```

**Parameters:**
- `path` — Path to the `.acon` file (UTF-8)
- `out` — Pointer to receive the handle

### `acon_close`

Close and free a contract engine handle.

```c
void acon_close(AconHandle *handle);
```

### `acon_save`

Save the contract to a file.

```c
int32_t acon_save(AconHandle *handle, const char *path);
```

### `acon_stats`

Get JSON statistics about the contract.

```c
int32_t acon_stats(const AconHandle *handle, char **out_json);
```

**Note:** Caller must free `out_json` with `acon_free_string`.

### `acon_policy_add`

Add a policy to the contract.

```c
int32_t acon_policy_add(AconHandle *handle, const char *label, int32_t scope, int32_t action);
```

**Parameters:**
- `label` — Policy label (UTF-8)
- `scope` — 0=Global, 1=Session, 2=Agent
- `action` — 0=Allow, 1=Deny, 2=RequireApproval, 3=AuditOnly

### `acon_policy_check`

Check if an action is allowed under current policies.

```c
int32_t acon_policy_check(const AconHandle *handle, const char *action_desc, char **out_json);
```

### `acon_free_string`

Free a string allocated by the FFI layer.

```c
void acon_free_string(char *s);
```

## Language Examples

### Python (ctypes)

```python
from agentic_contract import ContractEngine

engine = ContractEngine()
engine.add_policy("No deploys on Friday", scope="global", action="deny")
result = engine.check_policy("deploy to production")
```

### C

```c
#include "agentic_contract.h"

AconHandle *h;
acon_create(&h);
acon_policy_add(h, "spending-cap", 0, 1);
acon_close(h);
```
