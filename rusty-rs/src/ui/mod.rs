mod bolos;

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
    current_view_index: usize,
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
            current_view_index: 0,
            button_actions: Default::default(),
            button_bits: 0,
            button_timer: 0,
            max_scroll_time: Duration::zero(),
            auto_action: None,
            phantom_delegate: PhantomData,
        }
    }

    fn reset_for_redraw(&mut self) {
        let this = self.pic();
        this.current_view_index = 0;
        this.button_actions = Default::default();
        this.max_scroll_time = Duration::zero();
        this.auto_action = None;
    }

    fn send_next_view(&mut self, ch: Channel, delegate: &mut D) -> Option<Channel> {
        let this = self.pic();

        // Coordinate our rendering with the system UI
        match bolos::event() {
            bolos::Response::Redraw => {
                this.reset_for_redraw();
            },
            bolos::Response::Ignore | bolos::Response::Continue => {
                return Some(ch);
            },
            _ => {},
        }

        // See if there's another view to render
        let mut ctrl = Controller::new(this.current_view_index);
        delegate.prepare_ui(&mut ctrl);
        if let Some(ref view) = ctrl.target_view {
            let scroll_time = if let View::LabelLine(ref v) = view {
                v.estimate_scroll_time()
            } else {
                None
            };

            if let Some(scroll_time) = scroll_time {
                this.max_scroll_time = max(this.max_scroll_time, scroll_time);
            }

            let status = view.to_display_status(0).into();
            ch.send_status(status);

            None
        } else {
            this.button_actions = ctrl.button_actions;

            if let AutoAction::Countdown{
                min_wait_time,
                max_wait_time,
                wait_time,
                wait_for_scroll,
                action,
            } = ctrl.auto_action {
                let mut time = wait_time;
                if wait_for_scroll {
                    time += this.max_scroll_time;
                }
                if let Some(min_wait_time) = min_wait_time {
                    time = min(time, min_wait_time);
                }
                if let Some(max_wait_time) = max_wait_time {
                    time = max(time, max_wait_time);
                }
                this.auto_action = Some(ScheduledAction{
                    time_left: time,
                    action: action,
                });
            }

            Some(ch)
        }
    }

    fn process_button_presses(&mut self, button_bits: u8, delegate: &mut D) {
        let this = self.pic();

        const KEY_REPEAT_THRESHOLD: usize = 8; // 800ms
        const KEY_REPEAT_DELAY: usize = 3; // 300ms
        const LEFT_BUTTON: u8 = 1 << 0;
        const RIGHT_BUTTON: u8 = 1 << 1;
        const BOTH_BUTTONS: u8 = LEFT_BUTTON | RIGHT_BUTTON;
        let is_released = button_bits == 0;
        let previous_bits = this.button_bits;

        if this.button_bits == button_bits {
            this.button_timer += 1; // once every ~100ms
        } else {
            this.button_timer = 0;
            if button_bits != 0 {
                this.button_bits |= button_bits;
            } else {
                this.button_bits = 0;
            }
        }

        let (pressed_bits, repeating) = if is_released {
            (previous_bits, false)
        } else if this.button_timer > KEY_REPEAT_THRESHOLD
            && this.button_timer % KEY_REPEAT_DELAY == 0 {
            (button_bits, true)
        } else {
            (0, false)
        };

        let action = match (pressed_bits, repeating) {
            (LEFT_BUTTON, _) => this.button_actions.left,
            (RIGHT_BUTTON, _) => this.button_actions.right,
            (BOTH_BUTTONS, false) => this.button_actions.both,
            _ => None,
        };

        if let Some(action) = action {
            delegate.process_action(action);
        }
    }

    pub fn process_event(&mut self, ch: Channel, delegate: &mut D) -> Option<Channel> {
        let this = self.pic();

        match ch.event {
            Event::DisplayProcessed(_) => {
                this.current_view_index += 1;
            },
            Event::ButtonPush(ButtonPushEvent{ flags }) => {
                this.process_button_presses(flags >> 1, delegate);
            },
            _ => {},
        }

        if delegate.should_redraw() {
            this.reset_for_redraw();
        }
        this.send_next_view(ch, delegate)
    }

    pub fn redraw_if_needed(&mut self, ch: Channel, delegate: &mut D) -> Option<Channel> {
        let this = self.pic();

        if delegate.should_redraw() {
            this.reset_for_redraw();
            this.send_next_view(ch, delegate)
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
        let this = self.pic();
        if this.target_index == this.current_index {
            this.target_view = lazy_view().into();
        }
        this.current_index += 1;
    }

    pub fn set_button_actions(&mut self, actions: ButtonAction<A>) {
        let this = self.pic();
        this.button_actions = actions.into();
    }

    pub fn set_auto_action(&mut self, auto_action: AutoAction<A>) {
        let this = self.pic();
        this.auto_action = auto_action;
    }
}

#[derive(Copy, Clone)]
pub enum BasicAction {
    Previous,
    Next,
    Confirm,
}

pub trait Delegate {
    type Action: Copy;

    fn prepare_ui(&mut self, ctrl: &mut Controller<Self::Action>);
    fn should_redraw(&self) -> bool;
    fn process_action(&mut self, _action: Self::Action) {}
}

pub enum FillMode {
    NoFill,
    Fill,
    Outline,
}

impl FillMode {
    fn to_wire_format(&self) -> u8 {
        let this = self.pic();
        match this {
            &FillMode::NoFill => 0,
            &FillMode::Fill => 1,
            &FillMode::Outline => 2,
        }
    }
}

pub struct Color(u32);

impl Color {
    fn to_wire_format(&self) -> u32 {
        let this = self.pic();
        this.0
    }

    pub fn white() -> Self {
        Color(0xFFFFFF)
    }

    pub fn black() -> Self {
        Color(0x000000)
    }
}

pub struct Frame {
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

impl Default for Frame {
    fn default() -> Self {
        Self{ x: 0, y: 0, width: 0, height: 0 }
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
        let this = self.pic();

        ScreenDisplayShapeStatus{
            type_id: ScreenDisplayStatusTypeId::Rectangle,
            user_id,
            x: this.frame.x,
            y: this.frame.y,
            width: this.frame.width,
            height: this.frame.height,
            stroke: this.stroke,
            radius: this.radius,
            fill: this.fill.to_wire_format(),
            foreground_color: this.foreground.to_wire_format(),
            background_color: this.background.to_wire_format(),
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
}

impl SystemIcon {
    fn to_wire_format(&self) -> u8 {
        let this = self.pic();
        match this {
            &SystemIcon::Check => 6,
            &SystemIcon::Cross => 7,
        }
    }
}

impl<'a> Into<Icon<'a>> for SystemIcon {
    fn into(self) -> Icon<'a> {
        Icon::System(self)
    }
}

pub struct CustomIcon<'a> {
    pub bits_per_pixel: u8,
    pub colors: &'a [u32],
    pub bitmap: &'a [u8],
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

pub struct IconView<'a> {
    pub frame: Frame,
    pub icon: Icon<'a>,
}

impl<'a> IconView<'a> {
    fn to_display_status(&self, user_id: u8) -> ScreenDisplayStatus {
        let this = self.pic();

        match this.icon {
            Icon::Custom(ref icon) => {
                ScreenDisplayCustomIconStatus{
                    user_id,
                    x: this.frame.x,
                    y: this.frame.y,
                    width: this.frame.width,
                    height: this.frame.height,
                    bits_per_pixel: icon.bits_per_pixel,
                    colors: icon.colors,
                    bitmap: icon.bitmap,
                }.into()
            },
            Icon::System(ref icon) => {
                ScreenDisplaySystemIconStatus{
                    user_id,
                    x: this.frame.x,
                    y: this.frame.y,
                    width: this.frame.width,
                    height: this.frame.height,
                    icon_id: icon.to_wire_format(),
                }.into()
            },
        }
    }
}

impl<'a> Default for IconView<'a> {
    fn default() -> Self {
        Self{
            frame: Default::default(),
            icon: CustomIcon{
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
    Once{ delay_secs: u8, speed: u8 },
    Infinite{ delay_secs: u8, speed: u8 },
}

impl ScrollMode {
    fn to_wire_format(&self) -> (u8, u8) {
        let this = self.pic();
        match this {
            &ScrollMode::Disabled => (0, 0),
            &ScrollMode::Once{ delay_secs, speed } => (delay_secs | 0x80, speed),
            &ScrollMode::Infinite{ delay_secs, speed } => (delay_secs, speed),
        }
    }
}

pub enum TextHorizontalAlignment {
    Left,
    Center,
    Right,
}

impl TextHorizontalAlignment {
    fn to_wire_format(&self) -> u16 {
        let this = self.pic();
        match this {
            &TextHorizontalAlignment::Left => 0x0000,
            &TextHorizontalAlignment::Center => 0x8000,
            &TextHorizontalAlignment::Right => 0x4000,
        }
    }
}

pub enum TextVerticalAlignment {
    Top,
    Middle,
    Bottom,
}

impl TextVerticalAlignment {
    fn to_wire_format(&self) -> u16 {
        let this = self.pic();
        match this {
            &TextVerticalAlignment::Top => 0x0000,
            &TextVerticalAlignment::Middle => 0x2000,
            &TextVerticalAlignment::Bottom => 0x1000,
        }
    }
}

pub enum TextFont {
    OpenSansLight16px,
    OpenSansRegular11px,
    OpenSansExtraBold11px,
}

impl TextFont {
    fn to_wire_format(&self) -> u16 {
        let this = self.pic();
        match this {
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
        let this = self.pic();

        let (scroll_delay, scroll_speed) = this.scroll.to_wire_format();
        let font_id = this.font.to_wire_format()
            | this.horizontal_alignment.to_wire_format()
            | this.vertical_alignment.to_wire_format();

        ScreenDisplayTextStatus{
            type_id: ScreenDisplayStatusTypeId::LabelLine,
            user_id,
            x: this.frame.x,
            y: this.frame.y,
            width: this.frame.width,
            height: this.frame.height,
            scroll_delay,
            scroll_speed,
            fill: this.fill.to_wire_format(),
            foreground_color: this.foreground.to_wire_format(),
            background_color: this.background.to_wire_format(),
            font_id,
            text: this.text,
        }.into()
    }

    fn estimate_scroll_time(&self) -> Option<Duration> {
        let this = self.pic();

        match this.scroll {
            ScrollMode::Once{ delay_secs, speed } => {
                let text_width = this.font.width_for_text(this.text);
                let speed = speed as usize;
                let delay_secs = delay_secs as usize;
                let view_width = this.frame.width as usize;

                let scroll_time = if text_width > view_width {
                    2 * Duration::from_millis((text_width - view_width) * 1000 / speed)
                        + 2 * Duration::from_secs(delay_secs)
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
        let this = self.pic();
        match this {
            &View::Rectangle(ref v) => v.to_display_status(user_id),
            &View::Icon(ref v) => v.to_display_status(user_id),
            &View::LabelLine(ref v) => v.to_display_status(user_id),
        }
    }
}