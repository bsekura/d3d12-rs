use crate::shared::minwindef::{BOOL, DWORD, LPVOID, ULONG, HMODULE};
use crate::um::winnt::{LPWSTR, RTL_CRITICAL_SECTION, PRTL_CRITICAL_SECTION};

STRUCT!{struct SECURITY_ATTRIBUTES {
    nLength: DWORD,
    lpSecurityDescriptor: LPVOID,
    bInheritHandle: BOOL,
}}
pub type PSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
pub type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;

pub type CRITICAL_SECTION = RTL_CRITICAL_SECTION;
pub type PCRITICAL_SECTION = PRTL_CRITICAL_SECTION;
pub type LPCRITICAL_SECTION = PRTL_CRITICAL_SECTION;

STRUCT!{struct REASON_CONTEXT_Detailed {
    LocalizedReasonModule: HMODULE,
    LocalizedReasonId: ULONG,
    ReasonStringCount: ULONG,
    ReasonStrings: *mut LPWSTR,
}}
UNION!{union REASON_CONTEXT_Reason {
    [u32; 4] [u64; 3],
    Detailed Detailed_mut: REASON_CONTEXT_Detailed,
    SimpleReasonString SimpleReasonString_mut: LPWSTR,
}}
STRUCT!{struct REASON_CONTEXT {
    Version: ULONG,
    Flags: DWORD,
    Reason: REASON_CONTEXT_Reason,
}}
pub type PREASON_CONTEXT = *mut REASON_CONTEXT;
