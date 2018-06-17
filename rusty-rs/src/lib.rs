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
use syscall::os_sched_exit;

struct AppState {
}

impl ui::Delegate for AppState {
    fn prepare_ui(&mut self, ctrl: &mut ui::Controller) {
        ctrl.add_view(|| ui::RectangleView{
            frame: ui::Frame{ x: 0, y: 0, width: 128, height: 32 },
            fill: ui::FillMode::Fill,
            ..Default::default()
        }.into());
        ctrl.add_view(|| ui::IconView{
            frame: ui::Frame{ x: 3, y: 12, width: 7, height: 7 },
            icon: ui::SystemIcon::Cross.into(),
            ..Default::default()
        }.into());
        ctrl.add_view(|| ui::IconView{
            frame: ui::Frame{ x: 117, y: 13, width: 8, height: 6 },
            icon: ui::SystemIcon::Check.into(),
            ..Default::default()
        }.into());
        ctrl.add_view(|| ui::LabelLineView{
            frame: ui::Frame{ x: 0, y: 12, width: 128, height: 12 },
            font: ui::TextFont::OpenSansRegular11px,
            horizontal_alignment: ui::TextHorizontalAlignment::Center,
            text: "Hello!",
            ..Default::default()
        }.into());
        ctrl.add_view(|| ui::LabelLineView{
            user_id: 0x02,
            frame: ui::Frame{ x: 23, y: 26, width: 82, height: 12 },
            font: ui::TextFont::OpenSansRegular11px,
            horizontal_alignment: ui::TextHorizontalAlignment::Center,
            scroll: ui::ScrollMode::Once{ delay: 10, speed: 26 },
            text: "Rust",
            ..Default::default()
        }.into());
    }

    /*
    fn button_event(&mut self, btn) {
        // handle button events
    }
    */
}

#[no_mangle]
pub extern "C" fn rust_main() {
    // let mut state = RefCell::new(AppState{
    //     ui: UiState::WelcomeScreen,
    //     apdu: ApduState::Idle,
    // });
    let mut state = AppState{};

    let mut ui = ui::Middleware::new();

    MessageLoop::new().for_each(|ch| {
        let ch = Some(ch)
            .and_then(|ch| ui.process_event(ch, &mut state));
        //     .and_then(|ch| apdu.handle_event(ch, &mut state));
        //     .and_then(|ch| ui.push_changes(ch, &state));

        // if let Some(ch) = ch {
            // Custom event handling code here
        // }

        if let Some(ch) = ch {
            match ch.event {
                Event::ButtonPush(_) => {
                    os_sched_exit(1).is_ok();
                },
                _ => {},
            }
        }
    });
}
