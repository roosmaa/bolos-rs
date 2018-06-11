use core::convert::Into;

#[derive(Copy, Clone)]
pub enum FillMode {
    Fill,
    NoFill,
}

#[derive(Copy, Clone)]
pub struct Color(u32);

impl Color {
    fn white() -> Self {
        Color(0xFFFFFF)
    }

    fn black() -> Self {
        Color(0x000000)
    }
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub struct RectangleView {
    pub user_id: u8,
    pub frame: Frame,
    pub stroke: u8,
    pub radius: u8,
    pub fill: FillMode,
    pub foreground: Color,
    pub background: Color,
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

impl Into<View> for RectangleView {
    fn into(self) -> View {
        View::Rectangle(self)
    }
}

#[derive(Copy, Clone)]
pub struct CustomIcon {
    pub width: u32,
    pub height: u32,
    pub bits_per_pixel: u32,
    pub colors: &'static [Color],
    pub bitmap: &'static [u8],
}

impl Into<Icon> for CustomIcon {
    fn into(self) -> Icon {
        Icon::Custom(self)
    }
}

#[derive(Copy, Clone)]
pub enum Icon {
    Check,
    Cross,
    Custom(CustomIcon),
}

#[derive(Copy, Clone)]
pub struct IconView {
    pub user_id: u8,
    pub frame: Frame,
    pub icon: Icon,
}

impl Default for IconView {
    fn default() -> Self {
        Self{
            user_id: 0,
            frame: Default::default(),
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

impl Into<View> for IconView {
    fn into(self) -> View {
        View::Icon(self)
    }
}

#[derive(Copy, Clone)]
pub enum ScrollMode {
    Disabled,
    Once{ delay: u8, speed: u8 },
    Infinite{ delay: u8, speed: u8 }
}

#[derive(Copy, Clone)]
pub enum TextHorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Copy, Clone)]
pub enum TextVerticalAlignment {
    Top,
    Middle,
    Bottom,
}

#[derive(Copy, Clone)]
pub enum TextFont {
    OpenSansLight16px,
    OpenSansRegular11px,
    OpenSansExtraBold11px,
}

#[derive(Copy, Clone)]
pub struct LabelLineView {
    pub user_id: u8,
    pub frame: Frame,
    pub font: TextFont,
    pub horizontal_alignment: TextHorizontalAlignment,
    pub vertical_alignment: TextVerticalAlignment,
    pub scroll: ScrollMode,
    pub foreground: Color,
    pub background: Color,
    pub fill: FillMode,
    pub text: &'static str,
}

impl Default for LabelLineView {
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

impl Into<View> for LabelLineView {
    fn into(self) -> View {
        View::LabelLine(self)
    }
}

#[derive(Copy, Clone)]
pub enum View {
    None,
    Rectangle(RectangleView),
    Icon(IconView),
    LabelLine(LabelLineView),
}