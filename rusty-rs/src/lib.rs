#![no_std]
#![feature(asm)]
#![feature(panic_implementation)]
#![allow(dead_code)]

#![feature(const_slice_as_ptr)]

extern crate byteorder;

pub mod error;
mod syscall;
mod panic;
pub mod time;
pub mod seproxyhal;
pub mod ui;
pub mod pic;

use pic::Pic;
use seproxyhal::MessageLoop;
use syscall::os_sched_exit;
use time::Duration;

enum UiState {
    Welcome,
}

struct AppState {
    ui_state: UiState,
    ui_updated: bool,
}

impl AppState {
    fn new() -> Self {
        Self{
            ui_state: UiState::Welcome,
            ui_updated: true,
        }
    }

    fn update_ui(&mut self, new_state: UiState) {
        let this = self.pic();
        this.ui_updated = true;
        this.ui_state = new_state;
    }
}

impl ui::Delegate for AppState {
    type Action = ui::BasicAction;

    fn should_redraw(&self) -> bool {
        let this = self.pic();
        this.ui_updated
    }

    fn prepare_ui(&mut self, ctrl: &mut ui::Controller<Self::Action>) {
        let this = self.pic();
        this.ui_updated = false;

        ctrl.set_button_actions(ui::ButtonAction::ForAll(ui::BasicAction::Previous));
        ctrl.set_auto_action(ui::AutoAction::Countdown{
            min_wait_time: Some(Duration::from_millis(3000)),
            max_wait_time: Some(Duration::from_secs(10)),
            wait_time: Duration::from_millis(1000),
            wait_for_scroll: true,
            action: ui::BasicAction::Next,
        });

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
            frame: ui::Frame{ x: 23, y: 26, width: 82, height: 12 },
            font: ui::TextFont::OpenSansRegular11px,
            horizontal_alignment: ui::TextHorizontalAlignment::Center,
            scroll: ui::ScrollMode::Once{ delay_secs: 10, speed: 26 },
            text: "Rust",
            ..Default::default()
        }.into());
    }

    fn process_action(&mut self, action: Self::Action) {
        // Handle actions generated by user interactions and ui events
        match action {
            ui::BasicAction::Previous => {
                os_sched_exit(1).is_ok();
            },
            _ => {},
        }
    }
}

#[no_mangle]
pub extern "C" fn rust_main() {
    let mut state = AppState::new();

    let mut ui = ui::Middleware::new();

    MessageLoop::new().for_each(|ch| {
        let _ch = Some(ch)
            .and_then(|ch| ui.process_event(ch, &mut state))
            // .and_then(|ch| apdu.handle_event(ch, &mut state));
            .and_then(|ch| ui.redraw_if_needed(ch, &mut state));
    });
}
