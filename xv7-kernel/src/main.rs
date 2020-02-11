#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![allow(unused_attributes)]

#[macro_use]
mod macros;
#[macro_use]
extern crate embedded_graphics;

mod ansi;
mod config;
mod console;
mod gdt;
mod interrupt;
mod lang_item;
mod memory;
mod video;

use boot::KernelArgs;

use ansi::CtrlSeq;
use ansi::EraseParam;

pub fn kmain(args: &KernelArgs) -> ! {
    // Disable interrupts for safety.
    interrupt::without_interrupts(|| {
        print!(
            "{}{}{}",
            CtrlSeq::EraseInDisplay(Some(EraseParam::Entire)),
            CtrlSeq::CursorPosition(None, None),
            CtrlSeq::SelectGraphicRendition(None),
        );
        println!("Now we are in kernel!");

        dbg!(args);

        dbg!(unsafe { x86_64::registers::model_specific::Msr::new(0x1b).read() });

        gdt::init();
        interrupt::init();

        memory::FRAME_ALLOCATOR.lock().hello();

        fun_things();
    });

    halt_loop();
}

#[inline(always)]
fn halt_loop() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

use embedded_graphics::{fonts::Font8x16, image::ImageBmp, pixelcolor::Rgb888, prelude::*};

fn fun_things() {
    let mut display = video::GopDisplay;

    display.clear(RgbColor::WHITE);

    let logo: ImageBmp<Rgb888> = ImageBmp::new(include_bytes!("../resources/logo.bmp")).unwrap();

    logo.translate(
        (
            (display.size().width - logo.width()) as i32 / 2,
            (display.size().height - logo.height()) as i32 / 2,
        )
            .into(),
    )
    .draw(&mut display);

    egtext!(
        text = "XV7: Yet Another Operating System by imtsuki",
        top_left = (100, 100),
        style = text_style!(font = Font8x16, text_color = RgbColor::BLACK)
    )
    .draw(&mut display);
}
