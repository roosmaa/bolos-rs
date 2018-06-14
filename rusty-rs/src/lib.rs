#![no_std]
#![feature(asm)]
#![feature(panic_implementation)]
#![allow(dead_code)]

#![feature(const_slice_as_ptr)]

extern crate byteorder;

mod error;
mod syscall;
mod panic;
mod seproxyhal;
mod ui;
mod pic;

use core::slice;
use seproxyhal::event::Event;
use seproxyhal::status::{ScreenDisplayStatusTypeId, ScreenDisplayShapeStatus, ScreenDisplayTextStatus};
use syscall::os_sched_exit;

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

static mut EL_IDX: u8 = 0;

#[no_mangle]
pub extern "C" fn rust_process_event(ptr: *mut u8, len: usize) {
    let buf = unsafe { slice::from_raw_parts(ptr, len) };

    seproxyhal::process(buf, |ch| {
        let el_idx = unsafe { &mut EL_IDX };

        let status = match ch.event {
            Event::ButtonPush(_) => {
                os_sched_exit(1).is_ok();
                None
            },

            Event::DisplayProcessed(_) => {
                *el_idx += 1;

                if *el_idx == 1 {
                    Some(ScreenDisplayTextStatus{
                        type_id: ScreenDisplayStatusTypeId::LabelLine,
                        user_id: 0,
                        x: 0, y: 22, width: 128, height: 12,
                        scroll_delay: 0, scroll_speed: 0,
                        fill: 1,
                        foreground_color: 0xFFFFFF,
                        background_color: 0x000000,
                        font_id: 10 | 0x8000,
                        text: "Hello from Rust!",
                    }.into())
                } else {
                    None
                }
            },

            _ => {
                *el_idx = 0;

                Some(ScreenDisplayShapeStatus{
                    type_id: ScreenDisplayStatusTypeId::Rectangle,
                    user_id: 3,
                    x: 0, y: 0, width: 128, height: 32,
                    stroke: 0, radius: 0,
                    fill: 1,
                    foreground_color: 0x000000,
                    background_color: 0xFFFFFF,
                }.into())
            },
        };

        if let Some(status) = status {
            ch.send_status(status);
        }
    });
}

// TODO: Impleent a way to do event loops with no magical static variables
/*
struct AppState {
}

impl UiHooks for AppState {
    fn render(&self, ui) {
        ui.add_view();
    }

    fn button_event(&mut self, btn) {
        // handle button events
    }
}

#[no_mangle]
pub extern "C" fn rust_main() {
    let mut state = RefCell::new(AppState{
        ui: UiState::WelcomeScreen,
        apdu: ApduState::Idle,
    });

    let ui = UI::new();

    for ch in EventLoop::run() {
        let pipeline = Some(ch)
            .and_then(|ch| ui.handle_event(ch, &mut state));

        if let Some(ch) = pipeline {
            // Custom event handling here
        }
    }
}
*/

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
            text: "Hello!",
            ..Default::default()
        }.into(),
        ui::LabelLineView{
            user_id: 0x02,
            frame: ui::Frame{ x: 23, y: 26, width: 82, height: 12 },
            font: ui::TextFont::OpenSansRegular11px,
            horizontal_alignment: ui::TextHorizontalAlignment::Center,
            scroll: ui::ScrollMode::Once{ delay: 10, speed: 26 },
            text: "Rust",
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
