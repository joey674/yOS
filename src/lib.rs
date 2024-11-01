#![allow(non_snake_case)]
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub use serial::*;

pub mod vga_buffer;
pub use vga_buffer::*;

pub mod interrupts;

use core::panic::PanicInfo;



/* /////////////////////////////////////////////////////////////////////////////////////
    测试框架

*/ /////////////////////////////////////////////////////////////////////////////////////
pub trait Testable { /* 为所有函数类型都实现测试特性 这样的话执行run就可以自动打印执行结果 */
    fn run(&self) -> ();
}

impl<T> Testable for T
where T: Fn() {
    fn run(&self) -> () {
        serial_print!("{}...  ", core::any::type_name::<T>());  /* 这个可以直接打印出函数名！ */
        self();     /* 执行自身 */
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

/* /////////////////////////////////////////////////////////////////////////////////////
    内核退出

*/ /////////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}