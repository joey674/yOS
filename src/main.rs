/*  */
#![no_std]  /* 不链接 Rust 标准库 */
#![no_main] /*  禁用所有 Rust 层级的入口点 */
#![feature(custom_test_frameworks)]/* 通过从新定义cargo test的入口，程序就会执行自定义测试框架 */
#![test_runner(crate::test_runner)] 
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use yOS::*;


/* /////////////////////////////////////////////////////////////////////////////////////
    程序入口 panic处理 

*/ /////////////////////////////////////////////////////////////////////////////////////
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]/* 如果用cargo test，会从这里开始执行 */
    test_main();

    panic!();
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { /* cargo run:这个函数将在 panic 时被调用 */
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! { /* cargo test: 会将测试结果通过串口输出由QEMU转到我们的命令行 */
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}



