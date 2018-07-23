#![no_std]
#![no_main]

#[macro_use]
extern crate bolos;

mod icon;

use bolos::seproxyhal::MessageLoop;
use bolos::runtime::exit;
use bolos::ui;
use bolos::ui::menu;
use bolos::state::{Store, BasicAction};

enum UiState {
    MainMenu(MainMenuItem),
    SettingsMenu(SettingsMenuItem),
    TruncateAddressMenu(TruncateAddressMenuItem),
    RecipientMenu(RecipientMenuItem),
    AboutMenu(AboutMenuItem),
}

enum Recipient {
    Government,
    Charity,
    Myself,
}

struct AppState {
    truncate_address: bool,
    recipient: Recipient,
    ui_state: UiState,
    ui_version: u16,
}

impl AppState {
    fn new() -> Self {
        Self{
            truncate_address: false,
            recipient: Recipient::Charity,
            ui_state: UiState::MainMenu(MainMenuItem::RunDemo),
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
                            .unwrap_or(MainMenuItem::RunDemo);
                        self.update_ui(UiState::MainMenu(new_item));
                    },
                    BasicAction::Next => {
                        let new_item = menu::next_item(current_item, self)
                            .unwrap_or(MainMenuItem::Quit);
                        self.update_ui(UiState::MainMenu(new_item));
                    },
                    BasicAction::Confirm => match current_item {
                        MainMenuItem::Settings => self.update_ui(UiState::SettingsMenu(SettingsMenuItem::TruncateAddress)),
                        MainMenuItem::Quit => exit(0),
                        _ => self.update_ui(UiState::MainMenu(MainMenuItem::Quit)),
                    },
                }
            },
            UiState::SettingsMenu(current_item) => {
                match action {
                    BasicAction::Previous => {
                        let new_item = menu::previous_item(current_item, self)
                            .unwrap_or(SettingsMenuItem::TruncateAddress);
                        self.update_ui(UiState::SettingsMenu(new_item));
                    },
                    BasicAction::Next => {
                        let new_item = menu::next_item(current_item, self)
                            .unwrap_or(SettingsMenuItem::Back);
                        self.update_ui(UiState::SettingsMenu(new_item));
                    },
                    BasicAction::Confirm => match current_item {
                        SettingsMenuItem::TruncateAddress => {
                            let menu_item = if self.truncate_address {
                                TruncateAddressMenuItem::Yes
                            } else {
                                TruncateAddressMenuItem::No
                            };
                            self.update_ui(UiState::TruncateAddressMenu(menu_item))
                        },
                        SettingsMenuItem::Recipient => {
                            let menu_item = match self.recipient {
                                Recipient::Government => RecipientMenuItem::Government,
                                Recipient::Charity => RecipientMenuItem::Charity,
                                Recipient::Myself => RecipientMenuItem::Myself,
                            };
                            self.update_ui(UiState::RecipientMenu(menu_item))
                        },
                        SettingsMenuItem::About => self.update_ui(UiState::AboutMenu(AboutMenuItem::Version)),
                        SettingsMenuItem::Back => self.update_ui(UiState::MainMenu(MainMenuItem::Settings)),
                    },
                }
            },
            UiState::TruncateAddressMenu(current_item) => {
                match action {
                    BasicAction::Previous => {
                        let new_item = menu::previous_item(current_item, self)
                            .unwrap_or(TruncateAddressMenuItem::Yes);
                        self.update_ui(UiState::TruncateAddressMenu(new_item));
                    },
                    BasicAction::Next => {
                        let new_item = menu::next_item(current_item, self)
                            .unwrap_or(TruncateAddressMenuItem::No);
                        self.update_ui(UiState::TruncateAddressMenu(new_item));
                    },
                    BasicAction::Confirm => {
                        self.truncate_address = match current_item {
                            TruncateAddressMenuItem::Yes => true,
                            TruncateAddressMenuItem::No => false,
                        };
                        self.update_ui(UiState::SettingsMenu(SettingsMenuItem::TruncateAddress));
                    },
                }
            },
            UiState::RecipientMenu(current_item) => {
                match action {
                    BasicAction::Previous => {
                        let new_item = menu::previous_item(current_item, self)
                            .unwrap_or(RecipientMenuItem::Government);
                        self.update_ui(UiState::RecipientMenu(new_item));
                    },
                    BasicAction::Next => {
                        let new_item = menu::next_item(current_item, self)
                            .unwrap_or(RecipientMenuItem::Myself);
                        self.update_ui(UiState::RecipientMenu(new_item));
                    },
                    BasicAction::Confirm => {
                        self.recipient = match current_item {
                            RecipientMenuItem::Government => Recipient::Government,
                            RecipientMenuItem::Charity => Recipient::Charity,
                            RecipientMenuItem::Myself => Recipient::Myself,
                        };
                        self.update_ui(UiState::SettingsMenu(SettingsMenuItem::Recipient));
                    },
                }
            },
            UiState::AboutMenu(current_item) => {
                match action {
                    BasicAction::Previous => {
                        let new_item = menu::previous_item(current_item, self)
                            .unwrap_or(AboutMenuItem::Version);
                        self.update_ui(UiState::AboutMenu(new_item));
                    },
                    BasicAction::Next => {
                        let new_item = menu::next_item(current_item, self)
                            .unwrap_or(AboutMenuItem::Back);
                        self.update_ui(UiState::AboutMenu(new_item));
                    },
                    BasicAction::Confirm => match current_item {
                        AboutMenuItem::Back => self.update_ui(UiState::SettingsMenu(SettingsMenuItem::About)),
                        _ => self.update_ui(UiState::AboutMenu(AboutMenuItem::Back)),
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
        match self.ui_state {
            UiState::MainMenu(item) => menu::prepare_menu(item, self, ctrl),
            UiState::SettingsMenu(item) => menu::prepare_menu(item, self, ctrl),
            UiState::TruncateAddressMenu(item) => menu::prepare_menu(item, self, ctrl),
            UiState::RecipientMenu(item) => menu::prepare_menu(item, self, ctrl),
            UiState::AboutMenu(item) => menu::prepare_menu(item, self, ctrl),
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
    RunDemo,
    Settings,
    Quit,
}


#[derive(Copy, Clone, PartialEq, Eq)]
enum SettingsMenuItem {
    TruncateAddress,
    Recipient,
    About,
    Back,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum TruncateAddressMenuItem {
    Yes,
    No,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum RecipientMenuItem {
    Government,
    Charity,
    Myself,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum AboutMenuItem {
    Version,
    Developer,
    SourceCode,
    Back,
}

impl menu::Delegate<MainMenuItem> for AppState {
    fn prepare_menu_item(&self, ctrl: &mut menu::Controller<MainMenuItem, Self::Action>) {
        ctrl.add_item(MainMenuItem::RunDemo, || menu::ItemSpec{
            icon: Some(icon::badge_rust()),
            line_1: "Press buttons",
            line_2: "to start demo",
            action: Some(BasicAction::Confirm),
            ..Default::default()
        });
        ctrl.add_item(MainMenuItem::Settings, || menu::ItemSpec{
            line_1: "Settings",
            action: Some(BasicAction::Confirm),
            ..Default::default()
        });
        ctrl.add_item(MainMenuItem::Quit, || menu::ItemSpec{
            icon: Some(ui::SystemIcon::DashboardBadge.into()),
            line_1: "Quit",
            action: Some(BasicAction::Confirm),
            ..Default::default()
        });
    }
}

impl menu::Delegate<SettingsMenuItem> for AppState {
    fn prepare_menu_item(&self, ctrl: &mut menu::Controller<SettingsMenuItem, Self::Action>) {
        ctrl.add_item(SettingsMenuItem::TruncateAddress, || menu::ItemSpec{
            line_1: "Truncate address",
            action: Some(BasicAction::Confirm),
            ..Default::default()
        });
        ctrl.add_item(SettingsMenuItem::Recipient, || menu::ItemSpec{
            line_1: "Recipient",
            action: Some(BasicAction::Confirm),
            ..Default::default()
        });
        ctrl.add_item(SettingsMenuItem::About, || menu::ItemSpec{
            line_1: "About",
            action: Some(BasicAction::Confirm),
            ..Default::default()
        });
        ctrl.add_item(SettingsMenuItem::Back, || menu::ItemSpec{
            icon: Some(icon::badge_back()),
            line_1: "Back",
            action: Some(BasicAction::Confirm),
            ..Default::default()
        });
    }
}

impl menu::Delegate<TruncateAddressMenuItem> for AppState {
    fn prepare_menu_item(&self, ctrl: &mut menu::Controller<TruncateAddressMenuItem, Self::Action>) {
        ctrl.add_item(TruncateAddressMenuItem::Yes, || menu::ItemSpec{
            line_1: "Yes",
            action: Some(BasicAction::Confirm),
            ..Default::default()
        });
        ctrl.add_item(TruncateAddressMenuItem::No, || menu::ItemSpec{
            line_1: "No",
            action: Some(BasicAction::Confirm),
            ..Default::default()
        });
    }
}

impl menu::Delegate<RecipientMenuItem> for AppState {
    fn prepare_menu_item(&self, ctrl: &mut menu::Controller<RecipientMenuItem, Self::Action>) {
        ctrl.add_item(RecipientMenuItem::Government, || menu::ItemSpec{
            line_1: "Government",
            action: Some(BasicAction::Confirm),
            ..Default::default()
        });
        ctrl.add_item(RecipientMenuItem::Charity, || menu::ItemSpec{
            line_1: "Charity",
            action: Some(BasicAction::Confirm),
            ..Default::default()
        });
        ctrl.add_item(RecipientMenuItem::Myself, || menu::ItemSpec{
            line_1: "Myself",
            action: Some(BasicAction::Confirm),
            ..Default::default()
        });
    }
}

impl menu::Delegate<AboutMenuItem> for AppState {
    fn prepare_menu_item(&self, ctrl: &mut menu::Controller<AboutMenuItem, Self::Action>) {
        ctrl.add_item(AboutMenuItem::Version, || menu::ItemSpec{
            line_1: "Version",
            line_2: "1.2.3",
            line_1_font: ui::TextFont::OpenSansRegular11px,
            ..Default::default()
        });
        ctrl.add_item(AboutMenuItem::Developer, || menu::ItemSpec{
            line_1: "Developer",
            line_2: "Mart Roosmaa",
            line_1_font: ui::TextFont::OpenSansRegular11px,
            ..Default::default()
        });
        ctrl.add_item(AboutMenuItem::SourceCode, || menu::ItemSpec{
            line_1: "Source code",
            line_2: "github.com/roosmaa/bolos-rs",
            line_1_font: ui::TextFont::OpenSansRegular11px,
            ..Default::default()
        });
        ctrl.add_item(AboutMenuItem::Back, || menu::ItemSpec{
            icon: Some(icon::badge_back()),
            line_1: "Back",
            action: Some(BasicAction::Confirm),
            ..Default::default()
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
