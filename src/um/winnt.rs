use crate::ctypes::*;
//use crate::shared::ntdef::CHAR;
use crate::shared::minwindef::{BYTE, DWORD, WORD};
use crate::shared::basetsd::{ULONG_PTR};

pub use crate::shared::ntdef::LARGE_INTEGER;
pub use crate::shared::ntdef::LUID;
pub use crate::shared::ntdef::ULARGE_INTEGER;


pub type PVOID = *mut c_void;
pub type HRESULT = c_long;
pub type LPCSTR = *const CHAR;
pub type LPCWSTR = *const WCHAR;
pub type LPWSTR = *mut WCHAR;
pub type HANDLE = *mut c_void;
pub type VOID = c_void;
pub type CHAR = c_char;
pub type SHORT = c_short;
pub type LONG = c_long;
pub type INT = c_int;
pub type WCHAR = wchar_t;

pub type BOOLEAN = BYTE;
pub type PBOOLEAN = *mut BOOLEAN;

pub const STATUS_WAIT_0: DWORD = 0x00000000;
pub const STATUS_ABANDONED_WAIT_0: DWORD = 0x00000080;
pub const STATUS_USER_APC: DWORD = 0x000000C0;
pub const STATUS_TIMEOUT: DWORD = 0x00000102;
pub const STATUS_PENDING: DWORD = 0x00000103;

STRUCT!{struct RTL_RUN_ONCE {
    Ptr: PVOID,
}}
pub type PRTL_RUN_ONCE = *mut RTL_RUN_ONCE;
STRUCT!{struct RTL_BARRIER {
    Reserved1: DWORD,
    Reserved2: DWORD,
    Reserved3: [ULONG_PTR; 2],
    Reserved4: DWORD,
    Reserved5: DWORD,
}}
pub type PRTL_BARRIER = *mut RTL_BARRIER;

STRUCT!{struct RTL_SRWLOCK {
    Ptr: PVOID,
}}
pub type PRTL_SRWLOCK = *mut RTL_SRWLOCK;
pub const RTL_SRWLOCK_INIT: RTL_SRWLOCK = RTL_SRWLOCK { Ptr: 0 as PVOID };

STRUCT!{struct RTL_CONDITION_VARIABLE {
    Ptr: PVOID,
}}
pub type PRTL_CONDITION_VARIABLE = *mut RTL_CONDITION_VARIABLE;
pub const RTL_CONDITION_VARIABLE_INIT: RTL_CONDITION_VARIABLE = RTL_CONDITION_VARIABLE {
    Ptr: 0 as PVOID,
};
pub const RTL_CONDITION_VARIABLE_LOCKMODE_SHARED: DWORD = 0x1;

STRUCT!{struct LIST_ENTRY {
    Flink: *mut LIST_ENTRY,
    Blink: *mut LIST_ENTRY,
}}
pub type PLIST_ENTRY = *mut LIST_ENTRY;

STRUCT!{struct RTL_CRITICAL_SECTION_DEBUG {
    Type: WORD,
    CreatorBackTraceIndex: WORD,
    CriticalSection: *mut RTL_CRITICAL_SECTION,
    ProcessLocksList: LIST_ENTRY,
    EntryCount: DWORD,
    ContentionCount: DWORD,
    Flags: DWORD,
    CreatorBackTraceIndexHigh: WORD,
    SpareWORD: WORD,
}}
pub type PRTL_CRITICAL_SECTION_DEBUG = *mut RTL_CRITICAL_SECTION_DEBUG;

STRUCT!{struct RTL_CRITICAL_SECTION {
    DebugInfo: PRTL_CRITICAL_SECTION_DEBUG,
    LockCount: LONG,
    RecursionCount: LONG,
    OwningThread: HANDLE,
    LockSemaphore: HANDLE,
    SpinCount: ULONG_PTR,
}}
pub type PRTL_CRITICAL_SECTION = *mut RTL_CRITICAL_SECTION;
