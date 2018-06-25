#![no_std]
#![no_main]

#[macro_use]
extern crate bolos;

use bolos::seproxyhal::MessageLoop;
use bolos::exit;
use bolos::ui;

enum UiState {
    MainManu{
        active_item: usize,
    },
    // SettingsMenu{
    //     active_item: usize,
    // },
    // AboutMenu{
    //     active_item: usize,
    // },
}

struct AppState {
    main_menu_items: [&'static str; 5],
    ui_state: UiState,
    ui_version: u16,
}

impl AppState {
    fn new() -> Self {
        Self{
            main_menu_items: [
                "Item 1",
                "Item 2",
                "Item 3",
                "Item 4",
                "Quit app",
            ],
            ui_state: UiState::MainManu{ active_item: 0 },
            ui_version: 0,
        }
    }

    fn update_ui(&mut self, new_state: UiState) {
        self.ui_version += 1;
        self.ui_state = new_state;
    }
}

impl ui::Delegate for AppState {
    type Action = ui::BasicAction;

    fn ui_version(&self) -> u16 {
        self.ui_version
    }

    fn prepare_ui(&self, ctrl: &mut ui::Controller<Self::Action>) {
        ctrl.set_button_actions(ui::ButtonAction::Map{
            left: Some(ui::BasicAction::Previous),
            right: Some(ui::BasicAction::Next),
            both: Some(ui::BasicAction::Confirm),
        });

        // We always clear the screen of old content
        ctrl.add_view(|| ui::RectangleView{
            frame: ui::Frame{ x: 0, y: 0, width: 128, height: 32 },
            fill: ui::FillMode::Fill,
            ..Default::default()
        }.into());

        match self.ui_state {
            UiState::MainManu{ active_item } => {
                if active_item > 0 {
                    ctrl.add_view(|| ui::IconView{
                        position: ui::Position{ x: 3, y: 14 },
                        icon: ui::SystemIcon::Up.into(),
                        ..Default::default()
                    }.into());
                }
                if active_item + 1 < self.main_menu_items.len() {
                    ctrl.add_view(|| ui::IconView{
                        position: ui::Position{ x: 118, y: 14 },
                        icon: ui::SystemIcon::Down.into(),
                        ..Default::default()
                    }.into());
                }
                if active_item > 0 {
                    ctrl.add_view(|| ui::LabelLineView{
                        frame: ui::Frame{ x: 14, y: 3, width: 100, height: 12 },
                        font: ui::TextFont::OpenSansRegular11px,
                        horizontal_alignment: ui::TextHorizontalAlignment::Center,
                        text: self.main_menu_items[active_item-1],
                        ..Default::default()
                    }.into());
                }
                if active_item + 1 < self.main_menu_items.len() {
                    ctrl.add_view(|| ui::LabelLineView{
                        frame: ui::Frame{ x: 14, y: 35, width: 100, height: 12 },
                        font: ui::TextFont::OpenSansRegular11px,
                        horizontal_alignment: ui::TextHorizontalAlignment::Center,
                        text: self.main_menu_items[active_item+1],
                        ..Default::default()
                    }.into());
                }
                if active_item < self.main_menu_items.len() {
                    if active_item == 4 {
                        ctrl.add_view(|| ui::IconView{
                            position: ui::Position{ x: 29, y: 9 },
                            icon: ui::SystemIcon::DashboardBadge.into(),
                            ..Default::default()
                        }.into());
                        ctrl.add_view(|| ui::LabelLineView{
                            frame: ui::Frame{ x: 50, y: 19, width: 100, height: 12 },
                            font: ui::TextFont::OpenSansExtraBold11px,
                            text: self.main_menu_items[active_item],
                            ..Default::default()
                        }.into());
                    } else {
                        ctrl.add_view(|| ui::LabelLineView{
                            frame: ui::Frame{ x: 14, y: 19, width: 100, height: 12 },
                            font: ui::TextFont::OpenSansExtraBold11px,
                            horizontal_alignment: ui::TextHorizontalAlignment::Center,
                            text: self.main_menu_items[active_item],
                            ..Default::default()
                        }.into());
                    }
                }
            },
        }

        // ctrl.set_auto_action(ui::AutoAction::Countdown{
        //     min_wait_time: Some(Duration::from_millis(3000)),
        //     max_wait_time: Some(Duration::from_secs(10)),
        //     wait_time: Duration::from_millis(1000),
        //     wait_for_scroll: true,
        //     action: ui::BasicAction::Next,
        // });
    }

    fn process_action(&mut self, action: Self::Action) {
        match self.ui_state {
            UiState::MainManu{ active_item } => {
                match action {
                    ui::BasicAction::Previous => {
                        if active_item > 0 {
                            self.update_ui(UiState::MainManu{
                                active_item: active_item - 1,
                            });
                        }
                    },
                    ui::BasicAction::Next => {
                        if active_item + 1 < self.main_menu_items.len() {
                            self.update_ui(UiState::MainManu{
                                active_item: active_item + 1,
                            });
                        }
                    },
                    ui::BasicAction::Confirm => exit(0),
                }
            },
        }
    }
}

fn main() {
    let mut state = AppState::new();

    let mut ui = ui::Middleware::new();

    MessageLoop::new().for_each(|ch| {
        let _ch = Some(ch)
            .and_then(|ch| ui.process_event(ch, &mut state))
            .and_then(|ch| ui.redraw_if_needed(ch, &mut state));
    });
}

entry!(main);
