use byteorder::{ByteOrder, BigEndian};

#[repr(u8)]
enum EventTag {
    ButtonPush = 0x05,
    DisplayProcessed = 0x0D,
    Ticker = 0x0E,
}

impl EventTag {
    fn from_u8(value: u8) -> Option<Self> {
        if value == EventTag::ButtonPush as u8 {
            Some(EventTag::ButtonPush)
        } else if value == EventTag::DisplayProcessed as u8 {
            Some(EventTag::DisplayProcessed)
        } else if value == EventTag::Ticker as u8 {
            Some(EventTag::Ticker)
        } else {
            None
        }
    }
}

pub struct ButtonPushEvent {
    pub button_id: u8,
}

impl ButtonPushEvent {
    fn from_bytes(raw: &[u8]) -> Option<Self> {
        if raw.len() != 1 {
            None
        } else {
            // TODO: Unpack the button_id
            Some(Self{
                button_id: raw[0],
            })
        }
    }
}

pub struct DisplayProcessedEvent {
}

impl DisplayProcessedEvent {
    fn from_bytes(raw: &[u8]) -> Option<Self> {
        if raw.len() != 0 {
            None
        } else {
            Some(Self{})
        }
    }
}

pub struct TickerEvent {
}

impl TickerEvent {
    fn from_bytes(raw: &[u8]) -> Option<Self> {
        if raw.len() != 0 {
            None
        } else {
            Some(Self{})
        }
    }
}

pub enum Event {
    StartLoop,
    ButtonPush(ButtonPushEvent),
    DisplayProcessed(DisplayProcessedEvent),
    Ticker(TickerEvent),
}

impl Event {
    pub fn from_bytes(raw: &[u8]) -> Option<Self> {
        if raw.len() < 3 {
            return None;
        }

        let tag = EventTag::from_u8(raw[0]);
        let data_len = BigEndian::read_u16(&raw[1..3]);
        let data = &raw[3..3+(data_len as usize)];

        match tag {
            Some(EventTag::ButtonPush) =>
                ButtonPushEvent::from_bytes(data)
                    .map(|e| Event::ButtonPush(e)),
            Some(EventTag::DisplayProcessed) =>
                DisplayProcessedEvent::from_bytes(data)
                    .map(|e| Event::DisplayProcessed(e)),
            Some(EventTag::Ticker) =>
                TickerEvent::from_bytes(data)
                    .map(|e| Event::Ticker(e)),
            None => None,
        }
    }
}
