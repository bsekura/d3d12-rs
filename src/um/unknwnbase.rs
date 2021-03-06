use crate::ctypes::c_void;
use crate::shared::guiddef::REFIID;
use crate::shared::minwindef::{BOOL, ULONG};
use crate::um::winnt::HRESULT;

RIDL!{#[uuid(0x00000000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IUnknown(IUnknownVtbl) {
    fn QueryInterface(
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    fn AddRef() -> ULONG,
    fn Release() -> ULONG,
}}
pub type LPUNKNOWN = *mut IUnknown;
RIDL!{#[uuid(0x000e0000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface AsyncIUnknown(AsyncIUnknownVtbl): IUnknown(IUnknownVtbl) {
    fn Begin_QueryInterface(
        riid: REFIID,
    ) -> HRESULT,
    fn Finish_QueryInterface(
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    fn Begin_AddRef() -> HRESULT,
    fn Finish_AddRef() -> ULONG,
    fn Begin_Release() -> HRESULT,
    fn Finish_Release() -> ULONG,
}}
RIDL!{#[uuid(0x00000001, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IClassFactory(IClassFactoryVtbl): IUnknown(IUnknownVtbl) {
    fn CreateInstance(
        pUnkOuter: *mut IUnknown,
        riid: REFIID,
        ppvObject: *mut *mut c_void,
    ) -> HRESULT,
    fn LockServer(
        fLock: BOOL,
    ) -> HRESULT,
}}
