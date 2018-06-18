#![no_std]
#![feature(asm)]
#![feature(panic_implementation)]
#![allow(dead_code)]

#![feature(const_slice_as_ptr)]

extern crate byteorder;

pub mod error;
mod syscall;
mod panic;
pub mod seproxyhal;
pub mod ui;
pub mod pic;

use seproxyhal::MessageLoop;
use seproxyhal::event::Event;
use seproxyhal::status::{ScreenDisplayStatusTypeId, ScreenDisplayShapeStatus, ScreenDisplayTextStatus};
use syscall::os_sched_exit;

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
*/

#[no_mangle]
pub extern "C" fn rust_main() {
    // let mut state = RefCell::new(AppState{
    //     ui: UiState::WelcomeScreen,
    //     apdu: ApduState::Idle,
    // });

    // let ui = UI::new();

    let mut ui_idx = 0;

    MessageLoop::new().for_each(|ch| {
        // let ch = Some(ch)
        //     .and_then(|ch| ui.handle_event(ch, &mut state));
        //     .and_then(|ch| apdu.handle_event(ch, &mut state));
        //     .and_then(|ch| ui.push_changes(ch, &state));

        // if let Some(ch) = ch {
            // Custom event handling code here
        // }

        match ch.event {
            Event::ButtonPush(_) => {
                os_sched_exit(1).is_ok();
            },

            Event::DisplayProcessed(_) => {
                ui_idx += 1;

                if ui_idx == 1 {
                    ch.send_status(ScreenDisplayTextStatus{
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
                }
            },

            _ => {
                ui_idx = 0;

                ch.send_status(ScreenDisplayShapeStatus{
                    type_id: ScreenDisplayStatusTypeId::Rectangle,
                    user_id: 3,
                    x: 0, y: 0, width: 128, height: 32,
                    stroke: 0, radius: 0,
                    fill: 1,
                    foreground_color: 0x000000,
                    background_color: 0xFFFFFF,
                }.into())
            },
        }
    });
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
