use crate::shared::minwindef::{BOOL, DWORD};

use crate::um::winnt::{LPCSTR, STATUS_WAIT_0, STATUS_ABANDONED_WAIT_0, STATUS_USER_APC};

pub const WAIT_FAILED: DWORD = 0xFFFFFFFF;
pub const WAIT_OBJECT_0: DWORD = STATUS_WAIT_0 as u32;
pub const WAIT_ABANDONED: DWORD = STATUS_ABANDONED_WAIT_0 as u32;
pub const WAIT_ABANDONED_0: DWORD = STATUS_ABANDONED_WAIT_0 as u32;
pub const WAIT_IO_COMPLETION: DWORD = STATUS_USER_APC as u32;
pub const INFINITE: DWORD = 0xFFFFFFFF;

extern "system" {
    pub fn SetDllDirectoryA(
        lpPathName: LPCSTR,
    ) -> BOOL;
}
