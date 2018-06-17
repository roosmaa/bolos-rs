mod bolos;

use core::convert::Into;
use pic::Pic;
use seproxyhal::Channel;
use seproxyhal::event::Event;
use seproxyhal::status::{
    ScreenDisplayStatus, ScreenDisplayStatusTypeId, ScreenDisplayShapeStatus,
    ScreenDisplayTextStatus, ScreenDisplaySystemIconStatus, ScreenDisplayCustomIconStatus,
};

pub struct Middleware {
    current_view: usize,
}

impl Middleware {
    pub fn new() -> Self {
        Self{
            current_view: 0,
        }
    }

    pub fn process_event<D>(&mut self, ch: Channel, delegate: &mut D) -> Option<Channel>
        where D: Delegate
    {
        if let Event::DisplayProcessed(_) = ch.event {
            self.current_view += 1;
        }

        // Coordinate our rendering with the system UI
        match bolos::event() {
            bolos::Response::Redraw => {
                self.current_view = 0;
            },
            bolos::Response::Ignore | bolos::Response::Continue => {
                return Some(ch);
            },
            _ => {},
        }

        // See if there's another view to render
        let mut renderer = DisplayList::new(self.current_view);
        delegate.render(&mut renderer);
        if let Some(ref view) = renderer.target_view {
            let status = view.to_display_status().into();
            ch.send_status(status);
            None
        } else {
            Some(ch)
        }
    }
}

pub struct DisplayList<'a> {
    target_index: usize,
    current_index: usize,
    target_view: Option<View<'a>>,
}

impl<'a> DisplayList<'a> {
    fn new(target_index: usize) -> Self {
        Self{
            target_index: target_index,
            current_index: 0,
            target_view: None,
        }
    }

    #[inline(always)]
    pub fn add<F>(&mut self, lazy_view: F)
        where F: FnOnce() -> View<'a>
    {
        if self.target_index == self.current_index {
            self.target_view = lazy_view().into();
        }
        self.current_index += 1;
    }
}

pub trait Delegate {
    fn render(&mut self, renderer: &mut DisplayList);
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
    pub user_id: u8,
    pub frame: Frame,
    pub stroke: u8,
    pub radius: u8,
    pub fill: FillMode,
    pub foreground: Color,
    pub background: Color,
}

impl RectangleView {
    fn to_display_status(&self) -> ScreenDisplayStatus {
        let this = self.pic();

        ScreenDisplayShapeStatus{
            type_id: ScreenDisplayStatusTypeId::Rectangle,
            user_id: this.user_id,
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
            user_id: 0,
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
    pub user_id: u8,
    pub frame: Frame,
    pub icon: Icon<'a>,
}

impl<'a> IconView<'a> {
    fn to_display_status(&self) -> ScreenDisplayStatus {
        let this = self.pic();

        match this.icon {
            Icon::Custom(ref icon) => {
                ScreenDisplayCustomIconStatus{
                    user_id: this.user_id,
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
                    user_id: this.user_id,
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
            user_id: 0,
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

pub enum ScrollMode {
    Disabled,
    Once{ delay: u8, speed: u8 },
    Infinite{ delay: u8, speed: u8 }
}

impl ScrollMode {
    fn to_wire_format(&self) -> (u8, u8) {
        let this = self.pic();
        let scroll_delay;
        let scroll_speed;
        match this {
            &ScrollMode::Disabled => {
                scroll_delay = 0;
                scroll_speed = 0;
            },
            &ScrollMode::Once{ delay: d, speed: s } => {
                scroll_delay = d | 0x80;
                scroll_speed = s;
            },
            &ScrollMode::Infinite{ delay: d, speed: s } => {
                scroll_delay = d;
                scroll_speed = s;
            },
        };
        (scroll_delay, scroll_speed)
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
            &TextHorizontalAlignment::Center => 0x4000,
            &TextHorizontalAlignment::Right => 0x8000,
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
            &TextVerticalAlignment::Middle => 0x1000,
            &TextVerticalAlignment::Bottom => 0x2000,
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
}

pub struct LabelLineView<'a> {
    pub user_id: u8,
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
    fn to_display_status(&self) -> ScreenDisplayStatus {
        let this = self.pic();

        let (scroll_delay, scroll_speed) = this.scroll.to_wire_format();
        let font_id = this.font.to_wire_format()
            | this.horizontal_alignment.to_wire_format()
            | this.vertical_alignment.to_wire_format();

        ScreenDisplayTextStatus{
            type_id: ScreenDisplayStatusTypeId::LabelLine,
            user_id: this.user_id,
            x: this.frame.x,
            y: this.frame.y,
            width: this.frame.width,
            height: this.frame.height,
            scroll_delay: scroll_delay,
            scroll_speed: scroll_speed,
            fill: this.fill.to_wire_format(),
            foreground_color: this.foreground.to_wire_format(),
            background_color: this.background.to_wire_format(),
            font_id: font_id,
            text: this.text,
        }.into()
    }
}

impl<'a> Default for LabelLineView<'a> {
    fn default() -> Self {
        Self{
            user_id: 0,
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
    fn to_display_status(&self) -> ScreenDisplayStatus {
        let this = self.pic();
        match this {
            &View::Rectangle(ref v) => v.to_display_status(),
            &View::Icon(ref v) => v.to_display_status(),
            &View::LabelLine(ref v) => v.to_display_status(),
        }
    }
}