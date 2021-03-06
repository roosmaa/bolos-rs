use byteorder::{ByteOrder, LittleEndian};
use pic::Pic;
use super::packet::{Packet, FourByteIterator};

#[repr(u8)]
enum StatusTag {
    General = 0x60,
    ScreenDisplay = 0x65,
}

pub struct GeneralStatus {
}

impl Packet for GeneralStatus {
    impl_packet!(self, StatusTag::General, {
        [S] 2 => [0; 2],
    });
}

impl<'a> Into<Status<'a>> for GeneralStatus {
    fn into(self) -> Status<'a> {
        Status::General(self)
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ScreenDisplayStatusTypeId {
    Button = 1,
    Label = 2,
    Rectangle = 3,
    Line = 4,
    Icon = 5,
    Circle = 6,
    LabelLine = 7,
}

const SCREEN_DISPLAY_HEADER_SIZE: usize = 28;

fn make_screen_display_header(
    type_id: ScreenDisplayStatusTypeId,
    user_id: u8,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    stroke: u8,
    radius: u8,
    fill: u8,
    foreground_color: u32,
    background_color: u32,
    font_id: u16,
    icon_id: u8,
) -> [u8; SCREEN_DISPLAY_HEADER_SIZE] {
    let mut hdr: [u8; SCREEN_DISPLAY_HEADER_SIZE] = [
        type_id as u8,
        user_id,
        0, 0, // x
        0, 0, // y
        0, 0, // width
        0, 0, // height
        stroke,
        radius,
        fill,
        0, 0, 0, // <alignment gap>
        0, 0, 0, 0, // foreground_color
        0, 0, 0, 0, // background_color
        0, 0, // font_id
        icon_id,
        0, // <alignment gap>
    ];
    LittleEndian::write_i16(&mut hdr[2..4], x);
    LittleEndian::write_i16(&mut hdr[4..6], y);
    LittleEndian::write_u16(&mut hdr[6..8], width);
    LittleEndian::write_u16(&mut hdr[8..10], height);
    LittleEndian::write_u32(&mut hdr[16..20], foreground_color);
    LittleEndian::write_u32(&mut hdr[20..24], background_color);
    LittleEndian::write_u16(&mut hdr[24..26], font_id);
    hdr
}

pub struct ScreenDisplayShapeStatus {
    pub type_id: ScreenDisplayStatusTypeId,
    pub user_id: u8,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub stroke: u8,
    pub radius: u8,
    pub fill: u8,
    pub foreground_color: u32,
    pub background_color: u32,
}

impl Packet for ScreenDisplayShapeStatus {
    impl_packet!(self, StatusTag::ScreenDisplay, {
        [S] SCREEN_DISPLAY_HEADER_SIZE => make_screen_display_header(
            self.type_id,
            self.user_id,
            self.x,
            self.y,
            self.width,
            self.height,
            self.stroke,
            self.radius,
            self.fill,
            self.foreground_color,
            self.background_color,
            0,
            0,
        ),
    });
}

impl<'a> Into<ScreenDisplayStatus<'a>> for ScreenDisplayShapeStatus {
    fn into(self) -> ScreenDisplayStatus<'a> {
        ScreenDisplayStatus::Shape(self)
    }
}

impl<'a> Into<Status<'a>> for ScreenDisplayShapeStatus {
    fn into(self) -> Status<'a> {
        Status::ScreenDisplay(self.into())
    }
}

pub struct ScreenDisplayTextStatus<'a> {
    pub type_id: ScreenDisplayStatusTypeId,
    pub user_id: u8,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub scroll_delay: u8,
    pub scroll_speed: u8,
    pub fill: u8,
    pub foreground_color: u32,
    pub background_color: u32,
    pub font_id: u16,
    pub text: &'a str,
}

impl<'a> Packet for ScreenDisplayTextStatus<'a> {
    impl_packet!(self, StatusTag::ScreenDisplay, {
        [S] SCREEN_DISPLAY_HEADER_SIZE => make_screen_display_header(
            self.type_id,
            self.user_id,
            self.x,
            self.y,
            self.width,
            self.height,
            self.scroll_delay,
            0,
            self.fill,
            self.foreground_color,
            self.background_color,
            self.font_id,
            self.scroll_speed,
        ),
        [S] self.text.pic().len() => self.text.pic().as_bytes(),
    });
}

impl<'a> Into<ScreenDisplayStatus<'a>> for ScreenDisplayTextStatus<'a> {
    fn into(self) -> ScreenDisplayStatus<'a> {
        ScreenDisplayStatus::Text(self)
    }
}

impl<'a> Into<Status<'a>> for ScreenDisplayTextStatus<'a> {
    fn into(self) -> Status<'a> {
        Status::ScreenDisplay(self.into())
    }
}

pub struct ScreenDisplaySystemIconStatus {
    pub user_id: u8,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub icon_id: u8,
}

impl Packet for ScreenDisplaySystemIconStatus {
    impl_packet!(self, StatusTag::ScreenDisplay, {
        [S] SCREEN_DISPLAY_HEADER_SIZE => make_screen_display_header(
            ScreenDisplayStatusTypeId::Icon,
            self.user_id,
            self.x,
            self.y,
            self.width,
            self.height,
            0,
            0,
            0,
            0,
            0,
            0,
            self.icon_id,
        ),
    });
}

impl<'a> Into<ScreenDisplayStatus<'a>> for ScreenDisplaySystemIconStatus {
    fn into(self) -> ScreenDisplayStatus<'a> {
        ScreenDisplayStatus::SystemIcon(self)
    }
}

impl<'a> Into<Status<'a>> for ScreenDisplaySystemIconStatus {
    fn into(self) -> Status<'a> {
        Status::ScreenDisplay(self.into())
    }
}

pub struct ScreenDisplayCustomIconStatus<'a> {
    pub user_id: u8,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub bits_per_pixel: u8,
    pub colors: &'a [u32],
    pub bitmap: &'a [u8],
}

impl<'a> Packet for ScreenDisplayCustomIconStatus<'a> {
    impl_packet!(self, StatusTag::ScreenDisplay, {
        [S] SCREEN_DISPLAY_HEADER_SIZE => make_screen_display_header(
            ScreenDisplayStatusTypeId::Icon,
            self.user_id,
            self.x,
            self.y,
            self.width,
            self.height,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ),
        [S] 1 => [self.bits_per_pixel],
        [I] 4 * self.colors.len() => {
            self.colors.pic().iter().flat_map(|n| {
                let mut tmp = [0; 4];
                LittleEndian::write_u32(&mut tmp, *n);
                FourByteIterator::new(tmp)
            })
        },
        [S] self.bitmap.len() => self.bitmap.pic(),
    });
}

impl<'a> Into<ScreenDisplayStatus<'a>> for ScreenDisplayCustomIconStatus<'a> {
    fn into(self) -> ScreenDisplayStatus<'a> {
        ScreenDisplayStatus::CustomIcon(self)
    }
}

impl<'a> Into<Status<'a>> for ScreenDisplayCustomIconStatus<'a> {
    fn into(self) -> Status<'a> {
        Status::ScreenDisplay(self.into())
    }
}

pub enum ScreenDisplayStatus<'a> {
    Shape(ScreenDisplayShapeStatus),
    Text(ScreenDisplayTextStatus<'a>),
    SystemIcon(ScreenDisplaySystemIconStatus),
    CustomIcon(ScreenDisplayCustomIconStatus<'a>),
}

impl<'a> Packet for ScreenDisplayStatus<'a> {
    fn bytes_size(&self) -> u16 {
        match self {
            &ScreenDisplayStatus::Shape(ref s) => s.bytes_size(),
            &ScreenDisplayStatus::Text(ref s) => s.bytes_size(),
            &ScreenDisplayStatus::SystemIcon(ref s) => s.bytes_size(),
            &ScreenDisplayStatus::CustomIcon(ref s) => s.bytes_size(),
        }
    }

    fn to_bytes(&self, buf: &mut [u8], offset: usize) -> usize {
        match self {
            &ScreenDisplayStatus::Shape(ref s) => s.to_bytes(buf, offset),
            &ScreenDisplayStatus::Text(ref s) => s.to_bytes(buf, offset),
            &ScreenDisplayStatus::SystemIcon(ref s) => s.to_bytes(buf, offset),
            &ScreenDisplayStatus::CustomIcon(ref s) => s.to_bytes(buf, offset),
        }
    }
}

impl<'a> Into<Status<'a>> for ScreenDisplayStatus<'a> {
    fn into(self) -> Status<'a> {
        Status::ScreenDisplay(self)
    }
}

pub enum Status<'a> {
    General(GeneralStatus),
    ScreenDisplay(ScreenDisplayStatus<'a>),
}

impl<'a> Packet for Status<'a> {
    fn bytes_size(&self) -> u16 {
        match self {
            &Status::General(ref s) => s.bytes_size(),
            &Status::ScreenDisplay(ref s) => s.bytes_size(),
        }
    }

    fn to_bytes(&self, buf: &mut [u8], offset: usize) -> usize {
        match self {
            &Status::General(ref s) => s.to_bytes(buf, offset),
            &Status::ScreenDisplay(ref s) => s.to_bytes(buf, offset),
        }
    }
}
