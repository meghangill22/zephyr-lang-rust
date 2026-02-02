// Copyright (c) 2024 Linaro LTD
// SPDX-License-Identifier: Apache-2.0

#![no_std]

use log::info;

mod my_math_functions;

const DIV_TOLERANCE: f32 = 0.0001;

// Helper function to print the type of a variable
fn print_type<T>(_: T) -> &'static str {
    core::any::type_name::<T>()
}

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().unwrap();
    }

    // DO NOT CHANGE THE DATATYPES OF a, b, OR c
    let a: u8 = 2;
    let b: u8 = 3;
    let mut c: f32 = 0.0;

    info!("a = {} ({})", a, print_type(a));
    info!("b = {} ({})", b, print_type(b));

    let ret: i32 = my_math_functions::divide_numbers(&a, &b, &mut c);

    if ret != 0 {
        info!("Error: {}", ret);
    } else {
        info!("c = {} ({})", c, print_type(c));
    }

    if (c - 0.66667).abs() < DIV_TOLERANCE {
        info!("YOU WIN!!");
    } else {
        info!("YOU LOSE!!");
    }
    
    info!("Hello world from Rust on {}", zephyr::kconfig::CONFIG_BOARD);
}