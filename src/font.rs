#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u16)]
pub enum PicIcon {
    AButton,
    BButton,
    CButton,
    WiiRemote,
    NumChuck,
    OneButton,
    TwoButton,
    Star,
    LaunchStar,
    PullStar,
    PullStarIcon,
    StarBit,
    Coconut,
    Bell,
    StarBunny,
    NumChuckPad,
    XMark,
    Coin,
    Mario,
    DPad,
    PullStarChip,
    LaunchStarChip,
    HomeButton,
    MinusButton,
    PlusButton,
    ZButton,
    SilverStar,
    GrandStar,
    Luigi,
    LaunchStarIcon,
    PurpleCoin,
    GreenStar, // Evanbowl!!
    Crown,
    CannonPoint,
    Space,
    RedStar,
    HandClosed,
    HandPointer,
    HandOpen,
    RainbowStarBit,
    Peach,
    Mail,
    QuestionMark,
    MarioOrLuigi,
    OneUp = 49,
    LifeShroom,
    HungryLuma,
    Polari,
    Comet,
    GreenQuestionMark
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    LightRed,
    LightGreen,
    LightBlue,
    LightYellow,
    BrightPink,
    Gray
}

impl Color {
    #[inline]
    pub const fn new(num: u8) -> Option<Self> {
        if num <= 11 {
            Some(unsafe {std::mem::transmute(num)})
        } else {
            None
        }
    }
    #[inline]
    pub fn as_utf16(self) -> Vec<u16> {
        format!("[color:{:?}]", self).to_lowercase().encode_utf16().collect()
    }
}

impl PicIcon {
    pub fn as_utf16(self) -> Vec<u16> {
        format!("[icon:{:?}]", self).to_lowercase().encode_utf16().collect()
    }
    #[inline]
    pub const fn new(num: u16) -> Option<Self> {
        if num <= 43 || (num >= 49 && num <= 54) {
            Some(unsafe {std::mem::transmute(num)})
        }
        else {
            None
        }
    }
}

