use crate::shared::minwindef::{BOOL, DWORD, LPDWORD, LPHANDLE};
use crate::um::winnt::HANDLE;

pub const INVALID_HANDLE_VALUE: HANDLE = -1isize as HANDLE;
extern "system" {
    pub fn CloseHandle(
        hObject: HANDLE,
    ) -> BOOL;
    pub fn DuplicateHandle(
        hSourceProcessHandle: HANDLE,
        hSourceHandle: HANDLE,
        hTargetProcessHandle: HANDLE,
        lpTargetHandle: LPHANDLE,
        dwDesiredAccess: DWORD,
        bInheritHandle: BOOL,
        dwOptions: DWORD,
    ) -> BOOL;
    pub fn CompareObjectHandles(
        hFirstObjectHandle: HANDLE,
        hSecondObjectHandle: HANDLE,
    ) -> BOOL;
    pub fn GetHandleInformation(
        hObject: HANDLE,
        lpdwFlags: LPDWORD,
    ) -> BOOL;
    pub fn SetHandleInformation(
        hObject: HANDLE,
        dwMask: DWORD,
        dwFlags: DWORD,
    ) -> BOOL;
}
