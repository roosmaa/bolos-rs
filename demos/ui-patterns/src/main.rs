#![no_std]
#![no_main]

#[macro_use]
extern crate bolos;

mod icon;

use bolos::seproxyhal::MessageLoop;
use bolos::runtime::exit;
use bolos::time::Duration;
use bolos::ui;
use bolos::ui::menu;
use bolos::state::{Store, BasicAction};

enum UiState {
    MainMenu(MainMenuItem),
    Demo(DemoState),
    SettingsMenu(SettingsMenuItem),
    TruncateAddressMenu(TruncateAddressMenuItem),
    RecipientMenu(RecipientMenuItem),
    AboutMenu(AboutMenuItem),
}

#[derive(Copy, Clone)]
enum DemoState {
    Notice,
    Address,
    Recipient,
}

impl DemoState {
    fn next(&self) -> Self {
        match *self {
            DemoState::Notice => DemoState::Address,
            DemoState::Address => DemoState::Recipient,
            DemoState::Recipient => DemoState::Notice,
        }
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

enum Recipient {
    Government,
    Charity,
    Myself,
}

struct AppState {
    demo_confirms: u32,
    truncate_address: bool,
    recipient: Recipient,
    ui_state: UiState,
    ui_version: u16,
}

impl AppState {
    fn new() -> Self {
        Self{
            demo_confirms: 0,
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
                        MainMenuItem::RunDemo => self.update_ui(UiState::Demo(DemoState::Notice)),
                        MainMenuItem::Settings => self.update_ui(UiState::SettingsMenu(SettingsMenuItem::TruncateAddress)),
                        MainMenuItem::Quit => exit(0),
                    },
                }
            },
            UiState::Demo(state) => {
                match action {
                    BasicAction::Previous => {
                        self.update_ui(UiState::MainMenu(MainMenuItem::RunDemo));
                    },
                    BasicAction::Confirm => {
                        self.demo_confirms += 1;
                        self.update_ui(UiState::MainMenu(MainMenuItem::RunDemo));
                    },
                    BasicAction::Next => {
                        self.update_ui(UiState::Demo(state.next()));
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

            UiState::Demo(state) => {
                ctrl.set_button_actions(ui::ButtonAction::Map{
                    left: Some(BasicAction::Previous),
                    right: Some(BasicAction::Confirm),
                    both: None,
                });

                ctrl.add_view(|| ui::RectangleView{
                    frame: ui::Frame{ x: 0, y: 0, width: 128, height: 32 },
                    fill: ui::FillMode::Fill,
                    ..Default::default()
                }.into());
                ctrl.add_view(|| ui::IconView{
                    position: ui::Position{ x: 3, y: 12 },
                    icon: ui::SystemIcon::Cross.into(),
                    ..Default::default()
                }.into());
                ctrl.add_view(|| ui::IconView{
                    position: ui::Position{ x: 117, y: 13 },
                    icon: ui::SystemIcon::Check.into(),
                    ..Default::default()
                }.into());

                let line_1_base = ui::LabelLineView{
                    frame: ui::Frame{ x: 0, y: 12, width: 128, height: 12 },
                    font: ui::TextFont::OpenSansExtraBold11px,
                    horizontal_alignment: ui::TextHorizontalAlignment::Center,
                    ..Default::default()
                };
                let line_2_base = ui::LabelLineView{
                    frame: ui::Frame{ x: 23, y: 26, width: 82, height: 12 },
                    font: ui::TextFont::OpenSansRegular11px,
                    horizontal_alignment: ui::TextHorizontalAlignment::Center,
                    scroll: ui::ScrollMode::Once{ delay: Duration::from_secs(1), speed: 26 },
                    ..Default::default()
                };

                match state {
                    DemoState::Notice => {
                        ctrl.add_view(|| ui::LabelLineView{
                            text: "Confirm",
                            ..line_1_base
                        }.into());
                        ctrl.add_view(|| ui::LabelLineView{
                            font: ui::TextFont::OpenSansExtraBold11px,
                            text: "details",
                            scroll: ui::ScrollMode::Disabled,
                            ..line_2_base
                        }.into());
                    },
                    DemoState::Address => {
                        ctrl.add_view(|| ui::LabelLineView{
                            text: "Address",
                            ..line_1_base
                        }.into());
                        ctrl.add_view(|| ui::LabelLineView{
                            text: match self.truncate_address {
                                true => "4ecb..4d51",
                                false => "4ecbde854d829816358041c8b393fa4d51",
                            },
                            ..line_2_base
                        }.into());
                    },
                    DemoState::Recipient => {
                        ctrl.add_view(|| ui::LabelLineView{
                            text: "Recipient",
                            ..line_1_base
                        }.into());
                        ctrl.add_view(|| ui::LabelLineView{
                            text: match self.recipient {
                                Recipient::Government => "Government",
                                Recipient::Charity => "Charity",
                                Recipient::Myself => "Myself",
                            },
                            ..line_2_base
                        }.into());
                    },
                }

                ctrl.set_auto_action(ui::AutoAction::Countdown{
                    min_wait_time: Some(Duration::from_secs(3)),
                    max_wait_time: Some(Duration::from_secs(30)),
                    wait_time: Duration::from_secs(1),
                    wait_for_scroll: true,
                    action: BasicAction::Next,
                });
            },
        }
    }
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
