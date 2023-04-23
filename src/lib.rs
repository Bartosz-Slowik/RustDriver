#![no_std]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use winapi::km::wdm::{DbgPrint, DRIVER_OBJECT};
use winapi::shared::ntdef::{NT_SUCCESS, UNICODE_STRING};
use winapi::shared::ntstatus::STATUS_SUCCESS;
use winapi::shared::ntdef::NTSTATUS;

#[no_mangle]
pub extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, _: *const UNICODE_STRING) -> NTSTATUS {
    unsafe {
        DbgPrint("Hello, world!\0".as_ptr());
    }

    driver.DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

pub extern "system" fn driver_exit(driver: &mut DRIVER_OBJECT) {
    unsafe {
        DbgPrint("Bye bye!\0".as_ptr());
    }
}