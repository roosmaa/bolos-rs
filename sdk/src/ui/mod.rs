mod bolos;
pub mod menu;

use core::cmp::{min, max};
use core::marker::PhantomData;
use core::convert::Into;
use pic::Pic;
use time::Duration;
use seproxyhal::Channel;
use seproxyhal::event::{Event, ButtonPushEvent};
use seproxyhal::status::{
    ScreenDisplayStatus, ScreenDisplayStatusTypeId, ScreenDisplayShapeStatus,
    ScreenDisplayTextStatus, ScreenDisplaySystemIconStatus, ScreenDisplayCustomIconStatus,
};
use state::Store;

struct ButtonActionMap<A> {
    left: Option<A>,
    right: Option<A>,
    both: Option<A>,
}

impl<A> Default for ButtonActionMap<A> {
    fn default() -> Self {
        Self{
            left: None,
            right: None,
            both: None,
        }
    }
}

pub enum ButtonAction<A: Copy> {
    Map{
        left: Option<A>,
        right: Option<A>,
        both: Option<A>,
    },
    ForAll(A),
    None,
}

impl<A> Into<ButtonActionMap<A>> for ButtonAction<A>
    where A: Copy
{
    fn into(self) -> ButtonActionMap<A> {
        match self {
            ButtonAction::Map{ left: l, right: r, both: b } => ButtonActionMap{
                left: l,
                right: r,
                both: b,
            },
            ButtonAction::ForAll(a) => ButtonActionMap{
                left: Some(a),
                right: Some(a),
                both: Some(a),
            },
            ButtonAction::None => Default::default(),
        }
    }
}

pub enum AutoAction<A> {
    Countdown{
        min_wait_time: Option<Duration>,
        max_wait_time: Option<Duration>,
        wait_time: Duration,
        wait_for_scroll: bool,
        action: A,
    },
    None,
}

struct ScheduledAction<A> {
    time_left: Duration,
    action: A,
}

impl<A> ScheduledAction<A> {
    fn new(time: Duration, action: A) -> Self {
        Self{
            time_left: time,
            action,
        }
    }
}

pub struct Middleware<A, D> {
    ui_version: Option<u16>,
    next_view_index: usize,
    sent_view_index: usize,
    button_actions: ButtonActionMap<A>,
    button_bits: u8,
    button_timer: usize,
    max_scroll_time: Duration,
    auto_action: Option<ScheduledAction<A>>,
    phantom_delegate: PhantomData<D>,
}

impl<A, D> Middleware<A, D>
    where A: Copy,
          D: Delegate<Action=A>,
{
    pub fn new() -> Self {
        Self{
            ui_version: None,
            next_view_index: 0,
            sent_view_index: usize::max_value(),
            button_actions: Default::default(),
            button_bits: 0,
            button_timer: 0,
            max_scroll_time: Duration::zero(),
            auto_action: None,
            phantom_delegate: PhantomData,
        }
    }

    fn should_redraw(&self, delegate: &D) -> bool {
        Some(delegate.ui_version()) != self.ui_version
    }

    fn reset_for_redraw(&mut self, delegate: &D) {
        self.ui_version = Some(delegate.ui_version());
        self.next_view_index = 0;
        self.sent_view_index = usize::max_value();
        self.button_actions = Default::default();
        self.max_scroll_time = Duration::zero();
        self.auto_action = None;
    }

    fn send_next_view(&mut self, ch: Channel, delegate: &mut D) -> Option<Channel> {
        // Coordinate our rendering with the system UI
        match bolos::event() {
            bolos::Response::Redraw => {
                self.reset_for_redraw(delegate);
            },
            bolos::Response::Ignore | bolos::Response::Continue => {
                return Some(ch);
            },
            _ => {},
        }

        // Everything is displayed already, nothing to be done
        if self.sent_view_index == self.next_view_index {
            return Some(ch);
        }

        // See if there's another view to render
        let mut ctrl = Controller::new(self.next_view_index);
        delegate.prepare_ui(&mut ctrl);
        self.sent_view_index = self.next_view_index;

        if let Some(ref view) = ctrl.target_view {
            let scroll_time = if let View::LabelLine(ref v) = view {
                v.estimate_scroll_time()
            } else {
                None
            };

            if let Some(scroll_time) = scroll_time {
                self.max_scroll_time = max(self.max_scroll_time, scroll_time);
            }

            let status = view.to_display_status(0).into();
            ch.send_status(status);

            None
        } else {
            self.button_actions = ctrl.button_actions;

            if let AutoAction::Countdown{
                min_wait_time,
                max_wait_time,
                wait_time,
                wait_for_scroll,
                action,
            } = ctrl.auto_action {
                let mut time = wait_time;
                if wait_for_scroll {
                    time = max(time, self.max_scroll_time);
                }
                if let Some(min_wait_time) = min_wait_time {
                    time = max(time, min_wait_time);
                }
                if let Some(max_wait_time) = max_wait_time {
                    time = min(time, max_wait_time);
                }
                self.auto_action = Some(ScheduledAction{
                    time_left: time,
                    action: action,
                });
            }

            Some(ch)
        }
    }

    fn process_button_presses(&mut self, button_bits: u8, delegate: &mut D) {
        const KEY_REPEAT_THRESHOLD: usize = 8; // 800ms
        const KEY_REPEAT_DELAY: usize = 3; // 300ms
        const LEFT_BUTTON: u8 = 1 << 0;
        const RIGHT_BUTTON: u8 = 1 << 1;
        const BOTH_BUTTONS: u8 = LEFT_BUTTON | RIGHT_BUTTON;
        let is_released = button_bits == 0;
        let previous_bits = self.button_bits;

        if self.button_bits == button_bits {
            self.button_timer += 1; // once every ~100ms
        } else {
            self.button_timer = 0;
            if button_bits != 0 {
                self.button_bits |= button_bits;
            } else {
                self.button_bits = 0;
            }
        }

        let (pressed_bits, repeating) = if is_released {
            (previous_bits, false)
        } else if self.button_timer > KEY_REPEAT_THRESHOLD
            && self.button_timer % KEY_REPEAT_DELAY == 0 {
            (button_bits, true)
        } else {
            (0, false)
        };

        let action = match (pressed_bits, repeating) {
            (LEFT_BUTTON, _) => self.button_actions.left,
            (RIGHT_BUTTON, _) => self.button_actions.right,
            (BOTH_BUTTONS, false) => self.button_actions.both,
            _ => None,
        };

        if let Some(action) = action {
            delegate.process_action(action);
        }
    }

    fn process_tick(&mut self, delegate: &mut D) {
        let passed = Duration::from_millis(100);

        if let Some(ScheduledAction{time_left, action}) = self.auto_action {
            if time_left > passed {
                self.auto_action = Some(ScheduledAction{
                    time_left: time_left - passed,
                    action,
                });
            } else {
                self.auto_action = None;
                delegate.process_action(action);
            }
        }
    }

    pub fn process_event(&mut self, ch: Channel, delegate: &mut D) -> Option<Channel> {
        match ch.event {
            Event::DisplayProcessed(_) => {
                self.next_view_index += 1;
            },
            Event::ButtonPush(ButtonPushEvent{ flags }) => {
                self.process_button_presses(flags >> 1, delegate);
            },
            Event::Ticker(_) => {
                self.process_tick(delegate);
            },
            _ => {},
        }

        if self.should_redraw(delegate) {
            bolos::wake_up();
            self.reset_for_redraw(delegate);
        }
        self.send_next_view(ch, delegate)
    }

    pub fn redraw_if_needed(&mut self, ch: Channel, delegate: &mut D) -> Option<Channel> {
        if self.should_redraw(delegate) {
            bolos::wake_up();
            self.reset_for_redraw(delegate);
            self.send_next_view(ch, delegate)
        } else {
            Some(ch)
        }
    }
}

pub struct Controller<'a, A: Copy> {
    target_index: usize,
    current_index: usize,
    target_view: Option<View<'a>>,
    button_actions: ButtonActionMap<A>,
    auto_action: AutoAction<A>,
}

impl<'a, A> Controller<'a, A>
    where A: Copy
{
    fn new(target_index: usize) -> Self {
        Self{
            target_index,
            current_index: 0,
            target_view: None,
            button_actions: Default::default(),
            auto_action: AutoAction::None,
        }
    }

    #[inline(always)]
    pub fn add_view<F>(&mut self, lazy_view: F)
        where F: FnOnce() -> View<'a>
    {
        if self.target_index == self.current_index {
            self.target_view = lazy_view().into();
        }
        self.current_index += 1;
    }

    pub fn set_button_actions(&mut self, actions: ButtonAction<A>) {
        self.button_actions = actions.into();
    }

    pub fn set_auto_action(&mut self, auto_action: AutoAction<A>) {
        self.auto_action = auto_action;
    }
}

pub trait Delegate: Store {
    fn ui_version(&self) -> u16;
    fn prepare_ui(&self, ctrl: &mut Controller<Self::Action>);
}

pub enum FillMode {
    NoFill,
    Fill,
    Outline,
}

impl FillMode {
    fn to_wire_format(&self) -> u8 {
        match self {
            &FillMode::NoFill => 0,
            &FillMode::Fill => 1,
            &FillMode::Outline => 2,
        }
    }
}

pub struct Color(u32);

impl Color {
    fn to_wire_format(&self) -> u32 {
        self.0
    }

    pub fn white() -> Self {
        Color(0xFFFFFF)
    }

    pub fn black() -> Self {
        Color(0x000000)
    }
}

#[derive(Default)]
pub struct Frame {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

impl Frame {
    fn new(position: Position, size: Size) -> Self {
        Self{
            x: position.x,
            y: position.y,
            width: size.width,
            height: size.height,
        }
    }
}

#[derive(Default)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    fn new(x: i16, y: i16) -> Self {
        Self{ x, y }
    }
}

#[derive(Default)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    fn new(width: u16, height: u16) -> Self {
        Self{ width, height }
    }
}

pub struct RectangleView {
    pub frame: Frame,
    pub stroke: u8,
    pub radius: u8,
    pub fill: FillMode,
    pub foreground: Color,
    pub background: Color,
}

impl RectangleView {
    fn to_display_status(&self, user_id: u8) -> ScreenDisplayStatus {
        ScreenDisplayShapeStatus{
            type_id: ScreenDisplayStatusTypeId::Rectangle,
            user_id,
            x: self.frame.x,
            y: self.frame.y,
            width: self.frame.width,
            height: self.frame.height,
            stroke: self.stroke,
            radius: self.radius,
            fill: self.fill.to_wire_format(),
            foreground_color: self.foreground.to_wire_format(),
            background_color: self.background.to_wire_format(),
        }.into()
    }
}

impl Default for RectangleView {
    fn default() -> Self {
        Self{
            frame: Default::default(),
            stroke: 0,
            radius: 0,
            fill: FillMode::NoFill,
            foreground: Color::black(),
            background: Color::white(),
        }
    }
}

impl<'a> Into<View<'a>> for RectangleView {
    fn into(self) -> View<'a> {
        View::Rectangle(self)
    }
}

pub enum SystemIcon {
    Check,
    Cross,
    Left,
    Right,
    Up,
    Down,
    DashboardBadge,
}

impl SystemIcon {
    pub fn dimensions(&self) -> Size {
        match self {
            &SystemIcon::Check => Size::new(8, 6),
            &SystemIcon::Cross => Size::new(7, 7),
            &SystemIcon::Left => Size::new(4, 7),
            &SystemIcon::Right => Size::new(4, 7),
            &SystemIcon::Up => Size::new(7, 4),
            &SystemIcon::Down => Size::new(7, 4),
            &SystemIcon::DashboardBadge => Size::new(14, 14),
        }
    }

    fn to_wire_format(&self) -> u8 {
        match self {
            &SystemIcon::Check => 6,
            &SystemIcon::Cross => 7,
            &SystemIcon::Left => 9,
            &SystemIcon::Right => 10,
            &SystemIcon::Up => 11,
            &SystemIcon::Down => 12,
            &SystemIcon::DashboardBadge => 15,
        }
    }
}

impl<'a> Into<Icon<'a>> for SystemIcon {
    fn into(self) -> Icon<'a> {
        Icon::System(self)
    }
}

pub struct CustomIcon<'a> {
    pub width: u16,
    pub height: u16,
    pub bits_per_pixel: u8,
    pub colors: &'a [u32],
    pub bitmap: &'a [u8],
}

impl<'a> CustomIcon<'a> {
    pub fn dimensions(&self) -> Size {
        Size::new(self.width, self.height)
    }
}

impl<'a> Into<Icon<'a>> for CustomIcon<'a> {
    fn into(self) -> Icon<'a> {
        Icon::Custom(self)
    }
}

pub enum Icon<'a> {
    System(SystemIcon),
    Custom(CustomIcon<'a>),
}

impl<'a> Icon<'a> {
    pub fn dimensions(&self) -> Size {
        match self {
            &Icon::System(ref ico) => ico.dimensions(),
            &Icon::Custom(ref ico) => ico.dimensions(),
        }
    }
}

pub struct IconView<'a> {
    pub position: Position,
    pub icon: Icon<'a>,
}

impl<'a> IconView<'a> {
    fn to_display_status(&self, user_id: u8) -> ScreenDisplayStatus {
        let size = self.icon.dimensions();

        match self.icon {
            Icon::Custom(ref icon) => {
                ScreenDisplayCustomIconStatus{
                    user_id,
                    x: self.position.x,
                    y: self.position.y,
                    width: size.width,
                    height: size.height,
                    bits_per_pixel: icon.bits_per_pixel,
                    colors: icon.colors,
                    bitmap: icon.bitmap,
                }.into()
            },
            Icon::System(ref icon) => {
                ScreenDisplaySystemIconStatus{
                    user_id,
                    x: self.position.x,
                    y: self.position.y,
                    width: size.width,
                    height: size.height,
                    icon_id: icon.to_wire_format(),
                }.into()
            },
        }
    }
}

impl<'a> Default for IconView<'a> {
    fn default() -> Self {
        Self{
            position: Default::default(),
            icon: CustomIcon{
                width: 0,
                height: 0,
                bits_per_pixel: 0,
                colors: &[],
                bitmap: &[],
            }.into(),
        }
    }
}

impl<'a> Into<View<'a>> for IconView<'a> {
    fn into(self) -> View<'a> {
        View::Icon(self)
    }
}

pub struct ScrollFinishedEvent<A> {
    minimum_time: usize,
    additional_time: usize,
    action: A,
}

impl<A> From<A> for ScrollFinishedEvent<A> {
    fn from(action: A) -> Self {
        Self{
            minimum_time: 3000,
            additional_time: 1000,
            action,
        }
    }
}

pub enum ScrollMode {
    Disabled,
    Once{ delay: Duration, speed: u8 },
    Infinite{ delay: Duration, speed: u8 },
}

impl ScrollMode {
    fn delay_as_decis(delay: Duration) -> u8 {
        let decis = delay.as_millis() / 100;
        // We only have 7 bits for the decisecond value, so cap the maximum at 0x7F
        min(0x7F, decis) as u8
    }

    fn to_wire_format(&self) -> (u8, u8) {
        match self {
            &ScrollMode::Disabled => (0, 0),
            &ScrollMode::Once{ delay, speed } => (ScrollMode::delay_as_decis(delay) | 0x80, speed),
            &ScrollMode::Infinite{ delay, speed } => (ScrollMode::delay_as_decis(delay), speed),
        }
    }
}

#[derive(Copy, Clone)]
pub enum TextHorizontalAlignment {
    Left,
    Center,
    Right,
}

impl TextHorizontalAlignment {
    fn to_wire_format(&self) -> u16 {
        match self {
            &TextHorizontalAlignment::Left => 0x0000,
            &TextHorizontalAlignment::Center => 0x8000,
            &TextHorizontalAlignment::Right => 0x4000,
        }
    }
}

#[derive(Copy, Clone)]
pub enum TextVerticalAlignment {
    Top,
    Middle,
    Bottom,
}

impl TextVerticalAlignment {
    fn to_wire_format(&self) -> u16 {
        match self {
            &TextVerticalAlignment::Top => 0x0000,
            &TextVerticalAlignment::Middle => 0x2000,
            &TextVerticalAlignment::Bottom => 0x1000,
        }
    }
}

#[derive(Copy, Clone)]
pub enum TextFont {
    OpenSansLight16px,
    OpenSansRegular11px,
    OpenSansExtraBold11px,
}

impl TextFont {
    fn to_wire_format(&self) -> u16 {
        match self {
            &TextFont::OpenSansLight16px => 9,
            &TextFont::OpenSansRegular11px => 10,
            &TextFont::OpenSansExtraBold11px => 8,
        }
    }

    fn width_for_text(&self, text: &str) -> usize {
        let avg_char_width = 7;
        text.pic().chars().count() * avg_char_width
    }
}

pub struct LabelLineView<'a> {
    pub frame: Frame,
    pub font: TextFont,
    pub horizontal_alignment: TextHorizontalAlignment,
    pub vertical_alignment: TextVerticalAlignment,
    pub scroll: ScrollMode,
    pub foreground: Color,
    pub background: Color,
    pub fill: FillMode,
    pub text: &'a str,
}

impl<'a> LabelLineView<'a> {
    fn to_display_status(&self, user_id: u8) -> ScreenDisplayStatus {
        let (scroll_delay, scroll_speed) = self.scroll.to_wire_format();
        let font_id = self.font.to_wire_format()
            | self.horizontal_alignment.to_wire_format()
            | self.vertical_alignment.to_wire_format();

        ScreenDisplayTextStatus{
            type_id: ScreenDisplayStatusTypeId::LabelLine,
            user_id,
            x: self.frame.x,
            y: self.frame.y,
            width: self.frame.width,
            height: self.frame.height,
            scroll_delay,
            scroll_speed,
            fill: self.fill.to_wire_format(),
            foreground_color: self.foreground.to_wire_format(),
            background_color: self.background.to_wire_format(),
            font_id,
            text: self.text,
        }.into()
    }

    fn estimate_scroll_time(&self) -> Option<Duration> {
        match self.scroll {
            ScrollMode::Once{ delay, speed } => {
                let text_width = self.font.width_for_text(self.text);
                let speed = speed as usize;
                let view_width = self.frame.width as usize;

                let scroll_time = if text_width > view_width {
                    2 * Duration::from_millis((text_width - view_width) * 1000 / speed)
                        + 2 * delay
                } else {
                    Duration::zero()
                };

                Some(scroll_time)
            },
            _ => None,
        }
    }
}

impl<'a> Default for LabelLineView<'a> {
    fn default() -> Self {
        Self{
            frame: Default::default(),
            font: TextFont::OpenSansRegular11px,
            horizontal_alignment: TextHorizontalAlignment::Left,
            vertical_alignment: TextVerticalAlignment::Top,
            scroll: ScrollMode::Disabled,
            foreground: Color::white(),
            background: Color::black(),
            fill: FillMode::NoFill,
            text: "",
        }
    }
}

impl<'a> Into<View<'a>> for LabelLineView<'a> {
    fn into(self) -> View<'a> {
        View::LabelLine(self)
    }
}

pub enum View<'a> {
    Rectangle(RectangleView),
    Icon(IconView<'a>),
    LabelLine(LabelLineView<'a>),
}

impl<'a> View<'a> {
    fn to_display_status(&self, user_id: u8) -> ScreenDisplayStatus {
        match self {
            &View::Rectangle(ref v) => v.to_display_status(user_id),
            &View::Icon(ref v) => v.to_display_status(user_id),
            &View::LabelLine(ref v) => v.to_display_status(user_id),
        }
    }
}