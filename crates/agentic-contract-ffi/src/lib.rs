//! FFI bindings for AgenticContract.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

use agentic_contract::ContractEngine;

/// FFI error codes.
#[repr(C)]
pub enum AconError {
    /// Success.
    Ok = 0,
    /// Entity not found.
    NotFound = 1,
    /// Policy violation.
    PolicyViolation = 2,
    /// Risk limit exceeded.
    RiskLimitExceeded = 3,
    /// Approval required.
    ApprovalRequired = 4,
    /// Invalid contract.
    InvalidContract = 5,
    /// File format error.
    FileFormat = 6,
    /// IO error.
    Io = 7,
    /// Null pointer.
    NullPointer = 8,
}

/// Opaque handle to a ContractEngine.
pub struct AconHandle {
    engine: ContractEngine,
}

/// Open an existing .acon file.
///
/// # Safety
/// `path` must be a valid null-terminated UTF-8 string.
#[no_mangle]
pub unsafe extern "C" fn acon_open(path: *const c_char, err: *mut AconError) -> *mut AconHandle {
    if path.is_null() {
        if !err.is_null() {
            *err = AconError::NullPointer;
        }
        return ptr::null_mut();
    }

    let c_str = CStr::from_ptr(path);
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            if !err.is_null() {
                *err = AconError::Io;
            }
            return ptr::null_mut();
        }
    };

    match ContractEngine::open(path_str) {
        Ok(engine) => {
            if !err.is_null() {
                *err = AconError::Ok;
            }
            Box::into_raw(Box::new(AconHandle { engine }))
        }
        Err(_) => {
            if !err.is_null() {
                *err = AconError::FileFormat;
            }
            ptr::null_mut()
        }
    }
}

/// Create a new in-memory contract engine.
///
/// # Safety
/// Returns a valid handle or null on failure.
#[no_mangle]
pub unsafe extern "C" fn acon_create(err: *mut AconError) -> *mut AconHandle {
    let engine = ContractEngine::new();
    if !err.is_null() {
        *err = AconError::Ok;
    }
    Box::into_raw(Box::new(AconHandle { engine }))
}

/// Close a contract handle and free memory.
///
/// # Safety
/// `handle` must be a valid pointer returned by `acon_open` or `acon_create`.
#[no_mangle]
pub unsafe extern "C" fn acon_close(handle: *mut AconHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

/// Save the contract engine to its file.
///
/// # Safety
/// `handle` must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn acon_save(handle: *mut AconHandle) -> AconError {
    if handle.is_null() {
        return AconError::NullPointer;
    }
    let h = &*handle;
    match h.engine.save() {
        Ok(_) => AconError::Ok,
        Err(_) => AconError::Io,
    }
}

/// Get statistics as a JSON string. Caller must free with `acon_free_string`.
///
/// # Safety
/// `handle` must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn acon_stats(handle: *const AconHandle) -> *mut c_char {
    if handle.is_null() {
        return ptr::null_mut();
    }
    let h = &*handle;
    let stats = h.engine.stats();
    match serde_json::to_string(&stats) {
        Ok(json) => match CString::new(json) {
            Ok(cstr) => cstr.into_raw(),
            Err(_) => ptr::null_mut(),
        },
        Err(_) => ptr::null_mut(),
    }
}

/// Add a policy. Returns the policy ID as a string. Caller must free with `acon_free_string`.
///
/// # Safety
/// `handle` and `label` must be valid pointers. `scope` is 0=Global, 1=Session, 2=Agent.
/// `action` is 0=Allow, 1=Deny, 2=RequireApproval, 3=AuditOnly.
#[no_mangle]
pub unsafe extern "C" fn acon_policy_add(
    handle: *mut AconHandle,
    label: *const c_char,
    scope: u32,
    action: u32,
) -> *mut c_char {
    if handle.is_null() || label.is_null() {
        return ptr::null_mut();
    }

    let h = &mut *handle;
    let label_str = match CStr::from_ptr(label).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let scope = match scope {
        0 => agentic_contract::PolicyScope::Global,
        1 => agentic_contract::PolicyScope::Session,
        2 => agentic_contract::PolicyScope::Agent,
        _ => return ptr::null_mut(),
    };

    let action = match action {
        0 => agentic_contract::PolicyAction::Allow,
        1 => agentic_contract::PolicyAction::Deny,
        2 => agentic_contract::PolicyAction::RequireApproval,
        3 => agentic_contract::PolicyAction::AuditOnly,
        _ => return ptr::null_mut(),
    };

    let policy = agentic_contract::Policy::new(label_str, scope, action);
    let id = h.engine.add_policy(policy);

    match CString::new(id.to_string()) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Check if an action is allowed. Returns 1 for allowed, 0 for denied, -1 for error.
///
/// # Safety
/// `handle` and `action_type` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn acon_policy_check(
    handle: *const AconHandle,
    action_type: *const c_char,
    scope: u32,
) -> i32 {
    if handle.is_null() || action_type.is_null() {
        return -1;
    }

    let h = &*handle;
    let action_str = match CStr::from_ptr(action_type).to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let scope = match scope {
        0 => agentic_contract::PolicyScope::Global,
        1 => agentic_contract::PolicyScope::Session,
        2 => agentic_contract::PolicyScope::Agent,
        _ => return -1,
    };

    let result = h.engine.check_policy(action_str, scope);
    match result {
        agentic_contract::PolicyAction::Allow => 1,
        _ => 0,
    }
}

/// Free a string allocated by this library.
///
/// # Safety
/// `s` must be a pointer returned by an acon_* function, or null.
#[no_mangle]
pub unsafe extern "C" fn acon_free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}
