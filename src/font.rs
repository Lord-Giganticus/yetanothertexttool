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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SoundId {
    None,
    SeSvKinopioTalkHey,	
    SeSvKinopioTalkYahoo,	
    SeSvKinopioTalkAngry,	
    SeSvKinopioTalkSad,	
    SeSvKinopioTalkHappy,	
    SeSvKinopioTalkSleep,	
    SeSvRabbitTalkNormal,	
    SeSvRabbitTalkCaught,	
    SeSvRabbitTalkThats,	
    SeSvRabbitTalkHelp,	
    SeSvRabbitTalkThanks,	
    SeSvPenguinLTalkNormal,	
    SeSvPenguinLTalkPleased,	
    SeSvPenguinLTalkNg,	
    SeSvPenguinLTalkQuestion,	
    SeSvPenguinLTalkDistant,	
    SeSvPenguinLTalkNormalL,	
    SeSvPenguinLTalkPleasedL,	
    SeSvPenguinLTalkNgL,	
    SeSvPenguinLTalkQuestionL,	
    SeSvPenguinLTalkOh,	
    SeSvPenguinSTalkNormal,	
    SeSvPenguinSTalkGlad,	
    SeSvPenguinSTalkGladHigh,	
    SeSvPenguinSTalkAngry,	
    SeSvPenguinSTalkSad,	
    SeSvPenguinSTalkHappy,	
    SeSvPenguinSTalkStrong,	
    SeSvPenguinSTalkNormalW,	
    SeSvPenguinSTalkGreet,	
    SeSvPenguinSTalkWin,	
    SeSvPenguinSTalkLose,	
    SeSvPenguinSTalkOuch,	
    SeSvPenguinAceTalkNormal,	
    SeSvPenguinAceTalkGreet,	
    SeSvPenguinAceTalkWin,	
    SeSvPenguinAceTalkLose,	
    SeSvSyatiTalkNormal,	
    SeSvSyatiTalkRingMeet,	
    SeSvSyatiTalkRingWin,	
    SeSvSyatiTalkRingLose,	
    SeSvSyatiTalkPhanMeet,	
    SeSvSyatiTalkPhanWin,	
    SeSvSyatiTalkPhanLose,	
    SeSvPenguinSsHappy,	
    SeSvPenguinSsGreet,	
    SeSvPenguinSsDamage,	
    SeSvPenguinSsDisappointed,	
    SeSvPenguinSsPleased,	
    SeSvPenguinSsAngry,	
    SeSvHoneybeeTalkNormal,	
    SeSvHoneybeeTalkConfusion,	
    SeSvHoneybeeTalkQuestion,	
    SeSvHoneybeeTalkSurprise,	
    SeSvHoneybeeTalkOrder,	
    SeSvHoneybeeTalkLaugh,	
    SeSvHoneybeeTalkGlad,	
    SeSvTicoTalkNormal,	
    SeSvTicoTalkGlad,	
    SeSvTicoTalkAngry,	
    SeSvTicoTalkSad,	
    SeSvTicoTalkHappy,	
    SeSvTicoTico = 64,	
    SeSvTicoTalkConfusion,	
    SeSvTicoTalkThanks,	
    SeSvLuigiMario,	
    SeSvLuigiFrightened,	
    SeSvLuigiSorry,	
    SeSvLuigiThanks,	
    SeSvLuigiHey,	
    SeDmKinopioChief,	
    SeSvTereracerTalkNormal,	
    SeSvTereracerTalkLaugh,	
    SeSvTereracerTalkRegret,	
    SeSvCaretakerShort,	
    SeSvCaretakerNormal,	
    SeSvCaretakerLong,	
    SeSvCaretakerRepeat,	
    SeSvHoneyqueenTalkSurprise = 85,	
    SeSvHoneyqueenTalkThanks,	
    SeSvHoneyqueenTalkWorry,	
    SeSvHoneyqueenTalkAa,	
    SeSvHoneyqueenTalkAn,	
    SeSvHoneyqueenTalkUfufu,	
    SeSvKinopioTalkWelcome,	
    SeSvKinopioTalkBeautiful,	
    SeSvKinopioTalkSurprise,	
    SeSvKinopioPuha,	
    SeSvKinopioTalkHelp,	
    SeSvKinopioTalkTremble,	
    SeSvKinopioTalkStrong,	
    SeSvKinopioTalkLookOut,	
    SeSvKinopioTalkWow,	
    SeSvPeachTalkHelp,	
    SeSvRosettaTalkNormal,	
    SeSvRosettaTalkSurprise,	
    SeSvRosettaTalkSmile,	
    SeSvRosettaTalkWaiting,	
    SeSvRosettaTalkWorried,	
    SeSvTicofatTalkNormal,	
    SeSvTicofatTalkKita,	
    SeSvTicofatMeta,	
    SeSvKinopiochiefTalkHey,	
    SeSvKinopiochiefTalkLaugh,	
    SeSvKinopiochiefTalkYahoo,	
    SeSvTicofatTalkGiveMe,	
    SeSvTicofatTalkWaku,	
    SeSvButlerTalkSurprise,	
    SeSvButlerTalkAgree,	
    SeSvButlerTalkWorried,	
    SeSvButlerTalkNormal,	
    SeSvPenguinOldGreet,	
    SeSvPenguinOldGrad,	
    SeSvPenguinOldSad,	
    SeSvPenguinOldNormal,	
    SeSvLuigiTalkTire,	
    SeSvLuigiTalkYah,	
    SeSvLuigiTalkHelp,	
    SeSvLuigiTalkOhYeah,	
    SeSvButlerTalkQuestion,	
    SeSvRosettaTalkLetsStart,	
    SeSvRosettaTalkSomuchToday,	
    SeSvPenguinOldScared,	
    SeSvTicocometTalkPururin,	
    SeSvTicocometTalkDon,	
    SeSvKinopioTalkWater,	
    SeSvKinopioTalkHeySnor,	
    SeSvKinopioTalkSadSnor,	
    SeSvTicoshopTalkPikarin,	
    SeSvTicoshopTalkKita,	
    SeBvKoopajrTlkProvoke,	
    SeBvKoopaTlkLaugh,	
    SeBvKoopaTlkNormal,	
    SeBvKoopaTlkRegret,	
    SeBvKoopaTlkCalm,	
    SeBvKoopaTlkExcited,	
    SeSvTicofatTalkYeah,	
    SeSvRosettaTalkThatsAll,	
    SeSvCareTakerTrample,	
    SeSvKinopioNoMail,	
    SeSvKinopioLookMail,	
    SeSvKinopioTalkShout,	
    SeSvKinopiochiefTalkEvasive,	
    SeSvRosettaTalkSo,	
    SeSvRosettaTalkLook,	
    SeSmSignboardHey,	
    SeSvRosettaTalkPlease,	
    SeSvRosettaTalkFu,	
    SeSvRosettaTalkTrouble,	
    SeSvRosettaTalkSigh,	
    SeSvRosettaTalkQuestion,	
    SeSvRosettaTalkSmileEpB,	
    SeSvHoneybeeTalkSleep,	
    SeSvKinopioTalkTired,	
    SeSvCaretakerAngryFast	
}

impl SoundId {
    pub fn as_string(num: u8) -> String {
        if num <= 62 || (num >= 64 && num <= 79) || (num >= 85 && num <= 161) {
            let sound: SoundId = unsafe {std::mem::transmute(num)};
            format!("{:?}", sound)
        } else {
            format!("{num}")
        }
    }
}