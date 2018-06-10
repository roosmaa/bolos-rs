#![no_std]
#![feature(asm)]
#![allow(dead_code)]

extern crate panic_abort;

mod error;
mod syscall;
mod ui;
mod pic;

use core::slice;

#[no_mangle]
pub extern "C" fn update_label(ptr: *mut u8, len: usize) {
    let buf = unsafe { slice::from_raw_parts_mut(ptr, len) };

    if let Err(_) = syscall::cx_rng(buf) {
        buf[0] = 'E' as u8;
        buf[1] = 'r' as u8;
        buf[2] = 'r' as u8;
        buf[3] = 0;
        return;
    }

    buf[0] = 'a' as u8 + (buf[0] % ('z' as u8 - 'a' as u8));
    buf[1] = 'a' as u8 + (buf[1] % ('z' as u8 - 'a' as u8));
    buf[2] = 'a' as u8 + (buf[2] % ('z' as u8 - 'a' as u8));
    buf[3] = 'a' as u8 + (buf[3] % ('z' as u8 - 'a' as u8));
    buf[4] = 0;
}

static mut UI: [ui::View; 10] = [ui::View::None; 10];

fn ui_idle() {
    let next_ui: [ui::View; 5] = [
        ui::RectangleView{
            frame: ui::Frame{ x: 0, y: 0, width: 128, height: 32 },
            fill: ui::FillMode::Fill,
            ..Default::default()
        }.into(),
        ui::IconView{
            frame: ui::Frame{ x: 3, y: 12, width: 7, height: 7 },
            icon: ui::Icon::Cross,
            ..Default::default()
        }.into(),
        ui::IconView{
            frame: ui::Frame{ x: 117, y: 13, width: 8, height: 6 },
            icon: ui::Icon::Check,
            ..Default::default()
        }.into(),
        ui::LabelLineView{
            frame: ui::Frame{ x: 0, y: 12, width: 128, height: 12 },
            font: ui::TextFont::OpenSansRegular11px,
            horizontal_alignment: ui::TextHorizontalAlignment::Center,
            text: "Hello!".as_bytes(),
            ..Default::default()
        }.into(),
        ui::LabelLineView{
            user_id: 0x02,
            frame: ui::Frame{ x: 23, y: 26, width: 82, height: 12 },
            font: ui::TextFont::OpenSansRegular11px,
            horizontal_alignment: ui::TextHorizontalAlignment::Center,
            scroll: ui::ScrollMode::Once{ delay: 10, speed: 26 },
            text: "Rust".as_bytes(),
            ..Default::default()
        }.into(),
    ];

    let ui: &'static mut [ui::View] = unsafe { &mut UI };
    let len = if next_ui.len() >= ui.len() {
        ui.len()
    } else {
        next_ui.len()
    };

    ui[..len].copy_from_slice(&next_ui[..len]);
    if len < ui.len() {
        ui[len] = ui::View::None;
    }
}