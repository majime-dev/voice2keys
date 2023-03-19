#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Vk {
    Backspace = 0x08,
    Tab = 0x09,
    Clear = 0x0c,
    Enter = 0x0d,
    Shift = 0x10,
    Control = 0x11,
    Alt = 0x12,
    Pause = 0x13,
    CapsLock = 0x14,
    Escape = 0x1b,
    Space = 0x20,
    PageUp = 0x21,
    PageDown = 0x22,
    End = 0x23,
    Home = 0x24,
    #[cfg_attr(feature = "serde", serde(alias = "Left"))]
    LeftArrow = 0x25,
    #[cfg_attr(feature = "serde", serde(alias = "Up"))]
    UpArrow = 0x26,
    #[cfg_attr(feature = "serde", serde(alias = "Right"))]
    RightArrow = 0x27,
    #[cfg_attr(feature = "serde", serde(alias = "Down"))]
    DownArrow = 0x28,
    Select = 0x29,
    Print = 0x2a,
    PrintScreen = 0x2c,
    Insert = 0x2d,
    Delete = 0x2e,
    #[cfg_attr(feature = "serde", serde(rename = "0"))]
    _0 = b'0',
    #[cfg_attr(feature = "serde", serde(rename = "1"))]
    _1 = b'1',
    #[cfg_attr(feature = "serde", serde(rename = "2"))]
    _2 = b'2',
    #[cfg_attr(feature = "serde", serde(rename = "3"))]
    _3 = b'3',
    #[cfg_attr(feature = "serde", serde(rename = "4"))]
    _4 = b'4',
    #[cfg_attr(feature = "serde", serde(rename = "5"))]
    _5 = b'5',
    #[cfg_attr(feature = "serde", serde(rename = "6"))]
    _6 = b'6',
    #[cfg_attr(feature = "serde", serde(rename = "7"))]
    _7 = b'7',
    #[cfg_attr(feature = "serde", serde(rename = "8"))]
    _8 = b'8',
    #[cfg_attr(feature = "serde", serde(rename = "9"))]
    _9 = b'9',
    #[cfg_attr(feature = "serde", serde(alias = "a"))]
    A = b'A',
    #[cfg_attr(feature = "serde", serde(alias = "b"))]
    B = b'B',
    #[cfg_attr(feature = "serde", serde(alias = "c"))]
    C = b'C',
    #[cfg_attr(feature = "serde", serde(alias = "d"))]
    D = b'D',
    #[cfg_attr(feature = "serde", serde(alias = "e"))]
    E = b'E',
    #[cfg_attr(feature = "serde", serde(alias = "f"))]
    F = b'F',
    #[cfg_attr(feature = "serde", serde(alias = "g"))]
    G = b'G',
    #[cfg_attr(feature = "serde", serde(alias = "h"))]
    H = b'H',
    #[cfg_attr(feature = "serde", serde(alias = "i"))]
    I = b'I',
    #[cfg_attr(feature = "serde", serde(alias = "j"))]
    J = b'J',
    #[cfg_attr(feature = "serde", serde(alias = "k"))]
    K = b'K',
    #[cfg_attr(feature = "serde", serde(alias = "l"))]
    L = b'L',
    #[cfg_attr(feature = "serde", serde(alias = "m"))]
    M = b'M',
    #[cfg_attr(feature = "serde", serde(alias = "n"))]
    N = b'N',
    #[cfg_attr(feature = "serde", serde(alias = "o"))]
    O = b'O',
    #[cfg_attr(feature = "serde", serde(alias = "p"))]
    P = b'P',
    #[cfg_attr(feature = "serde", serde(alias = "q"))]
    Q = b'Q',
    #[cfg_attr(feature = "serde", serde(alias = "r"))]
    R = b'R',
    #[cfg_attr(feature = "serde", serde(alias = "s"))]
    S = b'S',
    #[cfg_attr(feature = "serde", serde(alias = "t"))]
    T = b'T',
    #[cfg_attr(feature = "serde", serde(alias = "u"))]
    U = b'U',
    #[cfg_attr(feature = "serde", serde(alias = "v"))]
    V = b'V',
    #[cfg_attr(feature = "serde", serde(alias = "w"))]
    W = b'W',
    #[cfg_attr(feature = "serde", serde(alias = "x"))]
    X = b'X',
    #[cfg_attr(feature = "serde", serde(alias = "y"))]
    Y = b'Y',
    #[cfg_attr(feature = "serde", serde(alias = "z"))]
    Z = b'Z',
    LeftWin = 0x5b,
    RightWin = 0x5c,
    Numpad0 = 0x60,
    Numpad1 = 0x61,
    Numpad2 = 0x62,
    Numpad3 = 0x63,
    Numpad4 = 0x64,
    Numpad5 = 0x65,
    Numpad6 = 0x66,
    Numpad7 = 0x67,
    Numpad8 = 0x68,
    Numpad9 = 0x69,
    Multiply = 0x6a,
    Add = 0x6b,
    Separator = 0x6c,
    Subtract = 0x6d,
    Decimal = 0x6e,
    Divide = 0x6f,
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7a,
    F12 = 0x7b,
    F13 = 0x7c,
    F14 = 0x7d,
    F15 = 0x7e,
    F16 = 0x7f,
    F17 = 0x80,
    F18 = 0x81,
    F19 = 0x82,
    F20 = 0x83,
    F21 = 0x84,
    F22 = 0x85,
    F23 = 0x86,
    F24 = 0x87,
    Numlock = 0x90,
    ScrollLock = 0x91,
    LeftShift = 0xa0,
    RightShift = 0xa1,
    LeftControl = 0xa2,
    RightControl = 0xa3,
    LeftAlt = 0xa4,
    RightAlt = 0xa5,
    Plus = 0xbb,
    Comma = 0xbc,
    Minus = 0xbd,
    Period = 0xbe,
}
