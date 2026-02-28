/* AgenticContract FFI — C header */
#ifndef AGENTIC_CONTRACT_H
#define AGENTIC_CONTRACT_H

#ifdef __cplusplus
extern "C" {
#endif

/* Error codes */
enum AconError {
    ACON_OK = 0,
    ACON_NOT_FOUND = 1,
    ACON_POLICY_VIOLATION = 2,
    ACON_RISK_LIMIT_EXCEEDED = 3,
    ACON_APPROVAL_REQUIRED = 4,
    ACON_INVALID_CONTRACT = 5,
    ACON_FILE_FORMAT = 6,
    ACON_IO = 7,
    ACON_NULL_POINTER = 8,
};

/* Opaque handle */
typedef struct AconHandle AconHandle;

/* Lifecycle */
AconHandle* acon_open(const char* path, enum AconError* err);
AconHandle* acon_create(enum AconError* err);
void acon_close(AconHandle* handle);
int acon_save(AconHandle* handle);

/* Queries */
char* acon_stats(const AconHandle* handle);

/* Policy operations */
/* scope: 0=Global, 1=Session, 2=Agent */
/* action: 0=Allow, 1=Deny, 2=RequireApproval, 3=AuditOnly */
char* acon_policy_add(AconHandle* handle, const char* label, unsigned int scope, unsigned int action);
int acon_policy_check(const AconHandle* handle, const char* action_type, unsigned int scope);

/* Memory management */
void acon_free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif /* AGENTIC_CONTRACT_H */
