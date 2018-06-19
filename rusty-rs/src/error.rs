#[derive(Debug)]
#[repr(u16)]
pub enum SystemError {
    Exception = 1,
    InvalidParameter = 2,
    Overflow = 3,
    Security = 4,
    InvalidCrc = 5,
    InvalidChecksum = 6,
    InvalidCounter = 7,
    NotSupported = 8,
    InvalidState = 9,
    Timeout = 10,
    Pic = 11,
    AppExit = 12,
    IoOverflow = 13,
    IoHeader = 14,
    IoState = 15,
    IoReset = 16,
    CxPort = 17,
    System = 18,
}

impl SystemError {
    pub fn from_u16(value: u16) -> Option<Self> {
        if value == SystemError::Exception as u16 {
            Some(SystemError::Exception)
        } else if value == SystemError::InvalidParameter as u16 {
            Some(SystemError::InvalidParameter)
        } else if value == SystemError::Overflow as u16 {
            Some(SystemError::Overflow)
        } else if value == SystemError::Security as u16 {
            Some(SystemError::Security)
        } else if value == SystemError::InvalidCrc as u16 {
            Some(SystemError::InvalidCrc)
        } else if value == SystemError::InvalidChecksum as u16 {
            Some(SystemError::InvalidChecksum)
        } else if value == SystemError::InvalidCounter as u16 {
            Some(SystemError::InvalidCounter)
        } else if value == SystemError::NotSupported as u16 {
            Some(SystemError::NotSupported)
        } else if value == SystemError::InvalidState as u16 {
            Some(SystemError::InvalidState)
        } else if value == SystemError::Timeout as u16 {
            Some(SystemError::Timeout)
        } else if value == SystemError::Pic as u16 {
            Some(SystemError::Pic)
        } else if value == SystemError::AppExit as u16 {
            Some(SystemError::AppExit)
        } else if value == SystemError::IoOverflow as u16 {
            Some(SystemError::IoOverflow)
        } else if value == SystemError::IoHeader as u16 {
            Some(SystemError::IoHeader)
        } else if value == SystemError::IoState as u16 {
            Some(SystemError::IoState)
        } else if value == SystemError::IoReset as u16 {
            Some(SystemError::IoReset)
        } else if value == SystemError::CxPort as u16 {
            Some(SystemError::CxPort)
        } else if value == SystemError::System as u16 {
            Some(SystemError::System)
        } else {
            None
        }
    }
}