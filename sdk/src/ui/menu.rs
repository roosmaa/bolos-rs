use core::cmp::max;
use state::{Store, BasicAction};
use ui;

pub trait MenuAction {
    fn action_for_previous_menu_item() -> Self;
    fn action_for_next_menu_item() -> Self;
}

impl MenuAction for BasicAction {
    fn action_for_previous_menu_item() -> Self {
        BasicAction::Previous
    }

    fn action_for_next_menu_item() -> Self {
        BasicAction::Next
    }
}

pub trait Delegate<I>: Store {
    fn prepare_menu_item(&self, ctrl: &mut Controller<I, Self::Action>);
}

pub fn prepare_menu<M, A, D>(menu_item: M, delegate: &D, ui_ctrl: &mut ui::Controller<A>)
    where M: Copy,
          A: MenuAction + Copy,
          D: Delegate<M> + Store<Action=A>,
{
    let mut ctrl = Controller::new(menu_item, true);
    delegate.prepare_menu_item(&mut ctrl);

    ui_ctrl.add_view(|| ui::RectangleView{
        frame: ui::Frame{ x: 0, y: 0, width: 128, height: 32 },
        fill: ui::FillMode::Fill,
        ..Default::default()
    }.into());

    if ctrl.previous_spec.is_some() {
        ui_ctrl.add_view(|| ui::IconView{
            position: ui::Position{ x: 3, y: 14 },
            icon: ui::SystemIcon::Up.into(),
            ..Default::default()
        }.into());
    }
    if ctrl.next_spec.is_some() {
        ui_ctrl.add_view(|| ui::IconView{
            position: ui::Position{ x: 118, y: 14 },
            icon: ui::SystemIcon::Down.into(),
            ..Default::default()
        }.into());
    }
    if let Some(ItemSpec{
        icon,
        line_1,
        line_1_font,
        line_2,
        line_2_font,
        action,
    }) = ctrl.current_spec {
        let is_multiline = line_2.len() > 0;

        ui_ctrl.set_button_actions(ui::ButtonAction::Map{
            left: if ctrl.previous_spec.is_some() {
                Some(A::action_for_previous_menu_item())
            } else {
                None
            },
            right: if ctrl.next_spec.is_some() {
                Some(A::action_for_next_menu_item())
            } else {
                None
            },
            both: action,
        });

        if !is_multiline {
            if let Some(prev_spec) = ctrl.previous_spec {
                ui_ctrl.add_view(|| ui::LabelLineView{
                    frame: ui::Frame{ x: 14, y: 3, width: 100, height: 12 },
                    font: ui::TextFont::OpenSansRegular11px,
                    horizontal_alignment: ui::TextHorizontalAlignment::Center,
                    text: if prev_spec.line_2.len() > 0 {
                        prev_spec.line_2
                    } else {
                        prev_spec.line_1
                    },
                    ..Default::default()
                }.into());
            }
            if let Some(next_spec) = ctrl.next_spec {
                ui_ctrl.add_view(|| ui::LabelLineView{
                    frame: ui::Frame{ x: 14, y: 35, width: 100, height: 12 },
                    font: ui::TextFont::OpenSansRegular11px,
                    horizontal_alignment: ui::TextHorizontalAlignment::Center,
                    text: next_spec.line_1,
                    ..Default::default()
                }.into());
            }
        }

        // Choose the best layout for the data in the item spec
        let icon_width = if let Some(ref icon) = icon {
            icon.dimensions().width + 7
        } else {
            0
        };
        let line_1_width = line_1_font.width_for_text(line_1) as u16;
        let line_2_width = line_2_font.width_for_text(line_2) as u16;
        let total_width = icon_width + max(line_1_width, line_2_width);

        let available_width = 100;
        let icon_offset_x;
        let text_offset_x;
        let text_alignment;
        if icon_width == 0 {
            // No icon layout, just use the centered labels. If the 2nd
            // line is too long, enable scrolling for it
            icon_offset_x = 0;
            text_alignment = ui::TextHorizontalAlignment::Center;
            text_offset_x = 0;
            if line_2_width >= available_width {
                // TODO: Enable scroll for label
            }
        } else {
            // Left align the text labels to the icon and then
            // try to center them all
            text_alignment = ui::TextHorizontalAlignment::Left;
            icon_offset_x = max(0, (available_width - total_width) / 2);
            text_offset_x = icon_offset_x + icon_width;
        }

        if let Some(icon) = icon {
            let size = icon.dimensions();
            ui_ctrl.add_view(move || ui::IconView{
                position: ui::Position{
                    x: 14 + icon_offset_x as i16,
                    y: (32 - size.height as i16) / 2,
                },
                icon,
                ..Default::default()
            }.into());
        }

        if line_2.len() == 0 {
            // 1-line layout
            ui_ctrl.add_view(move || ui::LabelLineView{
                frame: ui::Frame{
                    x: 14 + text_offset_x as i16, y: 19,
                    width: available_width - text_offset_x, height: 12,
                },
                font: line_1_font,
                horizontal_alignment: text_alignment,
                text: line_1,
                ..Default::default()
            }.into());
        } else {
            // 2-line layout
            ui_ctrl.add_view(move || ui::LabelLineView{
                frame: ui::Frame{
                    x: 14 + text_offset_x as i16, y: 12,
                    width: available_width - text_offset_x, height: 12,
                },
                font: line_1_font,
                horizontal_alignment: text_alignment,
                text: line_1,
                ..Default::default()
            }.into());
            ui_ctrl.add_view(move || ui::LabelLineView{
                frame: ui::Frame{
                    x: 14 + text_offset_x as i16, y: 26,
                    width: available_width - text_offset_x, height: 12,
                },
                font: line_2_font,
                horizontal_alignment: text_alignment,
                text: line_2,
                ..Default::default()
            }.into());
        }
    }
}

pub fn previous_item<M, D>(menu_item: M, delegate: &D) -> Option<M>
    where M: Copy,
          D: Delegate<M>
{
    let mut c = Controller::new(menu_item, false);
    delegate.prepare_menu_item(&mut c);
    c.previous_item
}

pub fn next_item<M, D>(menu_item: M, delegate: &D) -> Option<M>
    where M: Copy,
          D: Delegate<M>
{
    let mut c = Controller::new(menu_item, false);
    delegate.prepare_menu_item(&mut c);
    c.next_item
}

pub struct ItemSpec<'a, A> {
    pub icon: Option<ui::Icon<'a>>,
    pub line_1: &'a str,
    pub line_1_font: ui::TextFont,
    pub line_2: &'a str,
    pub line_2_font: ui::TextFont,
    pub action: Option<A>,
}

impl<'a, A> Default for ItemSpec<'a, A> {
    fn default() -> Self {
        Self{
            icon: None,
            line_1: "",
            line_1_font: ui::TextFont::OpenSansExtraBold11px,
            line_2: "",
            line_2_font: ui::TextFont::OpenSansExtraBold11px,
            action: None,
        }
    }
}

enum ControllerState {
    NeedTargetItem,
    NeedNextItem,
    Done,
}

pub struct Controller<'a, I, A> {
    target_item: I,
    state: ControllerState,
    previous_item: Option<I>,
    next_item: Option<I>,
    resolve_spec: bool,
    previous_spec: Option<ItemSpec<'a, A>>,
    current_spec: Option<ItemSpec<'a, A>>,
    next_spec: Option<ItemSpec<'a, A>>,
}

impl<'a, I, A> Controller<'a, I, A> {
    fn new(target_item: I, resolve_spec: bool) -> Self {
        Self{
            target_item,
            state: ControllerState::NeedTargetItem,
            previous_item: None,
            next_item: None,
            resolve_spec,
            previous_spec: None,
            current_spec: None,
            next_spec: None,
        }
    }

    #[inline]
    pub fn add_item<F>(&mut self, menu_item: I, lazy_spec: F)
        where I: Eq,
              F: FnOnce() -> ItemSpec<'a, A>
    {
        match self.state {
            ControllerState::NeedTargetItem => {
                if menu_item != self.target_item {
                    self.previous_item = Some(menu_item);
                    if self.resolve_spec {
                        self.previous_spec = Some(lazy_spec());
                    }
                } else {
                    self.state = ControllerState::NeedNextItem;
                    if self.resolve_spec {
                        self.current_spec = Some(lazy_spec());
                    }
                }
            },
            ControllerState::NeedNextItem => {
                self.state = ControllerState::Done;
                self.next_item = Some(menu_item);
                if self.resolve_spec {
                    self.next_spec = Some(lazy_spec());
                }
            },
            ControllerState::Done => {},
        }
    }
}