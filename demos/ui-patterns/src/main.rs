#![no_std]
#![no_main]

#[macro_use]
extern crate bolos;

use bolos::seproxyhal::MessageLoop;
use bolos::runtime::exit;
use bolos::ui;
use bolos::ui::menu;
use bolos::state::{Store, BasicAction};

enum UiState {
    MainMenu(MainMenuItem),
    // SettingsMenu(SettingsMenuItem),
    // AboutMenu(AboutMenuItem),
}

struct AppState {
    ui_state: UiState,
    ui_version: u16,
}

impl AppState {
    fn new() -> Self {
        Self{
            ui_state: UiState::MainMenu(MainMenuItem::Line1),
            ui_version: 0,
        }
    }

    fn update_ui(&mut self, new_state: UiState) {
        self.ui_version += 1;
        self.ui_state = new_state;
    }
}

impl Store for AppState {
    type Action = BasicAction;

    fn process_action(&mut self, action: Self::Action) {
        match self.ui_state {
            UiState::MainMenu(current_item) => {
                match action {
                    BasicAction::Previous => {
                        let new_item = menu::previous_item(current_item, self)
                            .unwrap_or(MainMenuItem::Line1);
                        self.update_ui(UiState::MainMenu(new_item));
                    },
                    BasicAction::Next => {
                        let new_item = menu::next_item(current_item, self)
                            .unwrap_or(MainMenuItem::Quit);
                        self.update_ui(UiState::MainMenu(new_item));
                    },
                    BasicAction::Confirm => match current_item {
                        MainMenuItem::Quit => exit(0),
                        _ => self.update_ui(UiState::MainMenu(MainMenuItem::Quit)),
                    },
                }
            },
        }
    }
}

impl ui::Delegate for AppState {
    fn ui_version(&self) -> u16 {
        self.ui_version
    }

    fn prepare_ui(&self, ctrl: &mut ui::Controller<Self::Action>) {
        ctrl.set_button_actions(ui::ButtonAction::Map{
            left: Some(BasicAction::Previous),
            right: Some(BasicAction::Next),
            both: Some(BasicAction::Confirm),
        });

        // We always clear the screen of old content
        ctrl.add_view(|| ui::RectangleView{
            frame: ui::Frame{ x: 0, y: 0, width: 128, height: 32 },
            fill: ui::FillMode::Fill,
            ..Default::default()
        }.into());

        match self.ui_state {
            UiState::MainMenu(item) => menu::prepare_menu(item, self, ctrl),
        }

        // ctrl.set_auto_action(ui::AutoAction::Countdown{
        //     min_wait_time: Some(Duration::from_millis(3000)),
        //     max_wait_time: Some(Duration::from_secs(10)),
        //     wait_time: Duration::from_millis(1000),
        //     wait_for_scroll: true,
        //     action: BasicAction::Next,
        // });
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum MainMenuItem {
    Line1,
    Line2,
    Line3,
    Line4,
    Quit,
}

impl menu::Delegate<MainMenuItem> for AppState {
    fn prepare_menu_item(&self, ctrl: &mut menu::Controller<MainMenuItem, Self::Action>) {
        ctrl.add_item(MainMenuItem::Line1, || menu::ItemSpec{
            icon: None,
            line_1: "Line 1",
            line_2: "",
            action: Some(BasicAction::Confirm),
        });
        ctrl.add_item(MainMenuItem::Line2, || menu::ItemSpec{
            icon: None,
            line_1: "Line 2",
            line_2: "",
            action: Some(BasicAction::Confirm),
        });
        ctrl.add_item(MainMenuItem::Line3, || menu::ItemSpec{
            icon: Some(ui::SystemIcon::Check.into()),
            line_1: "Line 3.1",
            line_2: "Line 3.2",
            action: Some(BasicAction::Confirm),
        });
        ctrl.add_item(MainMenuItem::Line4, || menu::ItemSpec{
            icon: None,
            line_1: "Line 4",
            line_2: "",
            action: Some(BasicAction::Confirm),
        });
        ctrl.add_item(MainMenuItem::Quit, || menu::ItemSpec{
            icon: Some(ui::SystemIcon::DashboardBadge.into()),
            line_1: "Quit",
            line_2: "",
            action: Some(BasicAction::Confirm),
        });
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
