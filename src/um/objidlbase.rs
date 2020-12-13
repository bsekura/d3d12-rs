use crate::um::unknwnbase::{IUnknown, IUnknownVtbl};
use crate::ctypes::{c_int, c_void};
use crate::shared::basetsd::{SIZE_T};
use crate::shared::guiddef::{CLSID};
use crate::shared::minwindef::{DWORD, FILETIME, ULONG};
use crate::um::winnt::{HRESULT, LARGE_INTEGER, WCHAR, ULARGE_INTEGER};

pub type OLECHAR = WCHAR;
pub type LPOLESTR = *mut OLECHAR;
pub type LPCOLESTR = *const OLECHAR;

RIDL! {#[uuid(0x00000002, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IMalloc(IMallocVtbl): IUnknown(IUnknownVtbl) {
    fn Alloc(
        cb: SIZE_T,
    ) -> *mut c_void,
    fn Realloc(
        pv: *mut c_void,
        cb: SIZE_T,
    ) -> *mut c_void,
    fn Free(
        pv: *mut c_void,
    ) -> (),
    fn GetSize(
        pv: *mut c_void,
    ) -> SIZE_T,
    fn DidAlloc(
        pv: *mut c_void,
    ) -> c_int,
    fn HeapMinimize() -> (),
}}

RIDL! {#[uuid(0x0c733a30, 0x2a1c, 0x11ce, 0xad, 0xe5, 0x00, 0xaa, 0x00, 0x44, 0x77, 0x3d)]
interface ISequentialStream(ISequentialStreamVtbl): IUnknown(IUnknownVtbl) {
    fn Read(
        pv: *mut c_void,
        cb: ULONG,
        pcbRead: *mut ULONG,
    ) -> HRESULT,
    fn Write(
        pv: *const c_void,
        cb: ULONG,
        pcbWritten: *mut ULONG,
    ) -> HRESULT,
}}
STRUCT! {struct STATSTG {
    pwcsName: LPOLESTR,
    type_: DWORD,
    cbSize: ULARGE_INTEGER,
    mtime: FILETIME,
    ctime: FILETIME,
    atime: FILETIME,
    grfMode: DWORD,
    grfLocksSupported: DWORD,
    clsid: CLSID,
    grfStateBits: DWORD,
    reserved: DWORD,
}}
ENUM! {enum STGTY {
    STGTY_STORAGE = 1,
    STGTY_STREAM = 2,
    STGTY_LOCKBYTES = 3,
    STGTY_PROPERTY = 4,
}}
ENUM! {enum STREAM_SEEK {
    STREAM_SEEK_SET = 0,
    STREAM_SEEK_CUR = 1,
    STREAM_SEEK_END = 2,
}}
ENUM! {enum LOCKTYPE {
    LOCK_WRITE = 1,
    LOCK_EXCLUSIVE = 2,
    LOCK_ONLYONCE = 4,
}}
pub type LPSTREAM = *mut IStream;
RIDL! {#[uuid(0x0000000c, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IStream(IStreamVtbl): ISequentialStream(ISequentialStreamVtbl) {
    fn Seek(
        dlibMove: LARGE_INTEGER,
        dwOrigin: DWORD,
        plibNewPosition: *mut ULARGE_INTEGER,
    ) -> HRESULT,
    fn SetSize(
        libNewSize: ULARGE_INTEGER,
    ) -> HRESULT,
    fn CopyTo(
        pstm: *mut IStream,
        cb: ULARGE_INTEGER,
        pcbRead: *mut ULARGE_INTEGER,
        pcbWritten: *mut ULARGE_INTEGER,
    ) -> HRESULT,
    fn Commit(
        grfCommitFlags: DWORD,
    ) -> HRESULT,
    fn Revert() -> HRESULT,
    fn LockRegion(
        libOffset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        dwLockType: DWORD,
    ) -> HRESULT,
    fn UnlockRegion(
        libOffset: ULARGE_INTEGER,
        cb: ULARGE_INTEGER,
        dwLockType: DWORD,
    ) -> HRESULT,
    fn Stat(
        pstatstg: *mut STATSTG,
        grfStatFlag: DWORD,
    ) -> HRESULT,
    fn Clone(
        ppstm: *mut *mut IStream,
    ) -> HRESULT,
}}
