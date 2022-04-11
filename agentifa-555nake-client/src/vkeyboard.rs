use std::fmt::{Display, Formatter};

use bevy::{
    input::Input,
    math::{Rect, Vec2},
    prelude::{
        App, Color, Commands, Component, DespawnRecursiveExt, Entity, EventReader, EventWriter,
        MouseButton, OrthographicCameraBundle, ParallelSystemDescriptorCoercion, Plugin, Query,
        Res, SystemSet, Transform, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
    window::Windows,
};

use crate::{AppState, InputState, SpriteSheetAssets};

const COLOR: Color = Color::rgba(1., 1., 1., 0.5);
const COLS: u8 = 10;
const PAD: f32 = 0.1;

#[derive(Clone, Component, Copy)]
pub struct Button {
    case: Case,
    pub key: Key,
    modifier: Modifier,
}

impl Display for Button {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.key {
            Key::A => match self.modifier {
                Modifier::AccentAcute => match self.case {
                    Case::Upper => write!(f, "Á"),
                    Case::Lower => write!(f, "á"),
                },
                Modifier::AccentGrave => match self.case {
                    Case::Upper => write!(f, "À"),
                    Case::Lower => write!(f, "à"),
                },
                Modifier::Caret => match self.case {
                    Case::Upper => write!(f, "Â"),
                    Case::Lower => write!(f, "â"),
                },
                Modifier::Umlaut => match self.case {
                    Case::Upper => write!(f, "Ä"),
                    Case::Lower => write!(f, "ä"),
                },
                Modifier::None => match self.case {
                    Case::Upper => write!(f, "A"),
                    Case::Lower => write!(f, "a"),
                },
            },
            Key::Ampersand => write!(f, "&"),
            Key::Apostrophe => write!(f, "'"),
            Key::Asterisk => write!(f, "*"),
            Key::At => write!(f, "@"),
            Key::B => match self.case {
                Case::Upper => write!(f, "B"),
                Case::Lower => write!(f, "b"),
            },
            Key::Backslash => write!(f, "\\"),
            Key::Bar => write!(f, "|"),
            Key::BracketClose => write!(f, ")"),
            Key::BracketOpen => write!(f, "("),
            Key::C => match self.case {
                Case::Upper => write!(f, "C"),
                Case::Lower => write!(f, "c"),
            },
            Key::Colon => write!(f, ":"),
            Key::Comma => write!(f, ","),
            Key::CurlyBracketClose => write!(f, "}}"),
            Key::CurlyBracketOpen => write!(f, "{{"),
            Key::D => match self.case {
                Case::Upper => write!(f, "D"),
                Case::Lower => write!(f, "d"),
            },
            Key::Degree => write!(f, "°"),
            Key::Dollar => write!(f, "$"),
            Key::Dot => write!(f, "."),
            Key::E => match self.modifier {
                Modifier::AccentAcute => match self.case {
                    Case::Upper => write!(f, "É"),
                    Case::Lower => write!(f, "é"),
                },
                Modifier::AccentGrave => match self.case {
                    Case::Upper => write!(f, "È"),
                    Case::Lower => write!(f, "è"),
                },
                Modifier::Caret => match self.case {
                    Case::Upper => write!(f, "Ê"),
                    Case::Lower => write!(f, "ê"),
                },
                _ => match self.case {
                    Case::Upper => write!(f, "E"),
                    Case::Lower => write!(f, "e"),
                },
            },
            Key::Eight => write!(f, "8"),
            Key::Equals => write!(f, "="),
            Key::Euro => write!(f, "€"),
            Key::ExclamationMark => write!(f, "!"),
            Key::F => match self.case {
                Case::Upper => write!(f, "F"),
                Case::Lower => write!(f, "f"),
            },
            Key::Five => write!(f, "5"),
            Key::Four => write!(f, "4"),
            Key::G => match self.case {
                Case::Upper => write!(f, "G"),
                Case::Lower => write!(f, "g"),
            },
            Key::GreaterThan => write!(f, ">"),
            Key::H => match self.case {
                Case::Upper => write!(f, "H"),
                Case::Lower => write!(f, "h"),
            },
            Key::Hash => write!(f, "#"),
            Key::I => match self.modifier {
                Modifier::AccentAcute => match self.case {
                    Case::Upper => write!(f, "Í"),
                    Case::Lower => write!(f, "í"),
                },
                Modifier::AccentGrave => match self.case {
                    Case::Upper => write!(f, "Ì"),
                    Case::Lower => write!(f, "ì"),
                },
                Modifier::Caret => match self.case {
                    Case::Upper => write!(f, "Î"),
                    Case::Lower => write!(f, "î"),
                },
                _ => match self.case {
                    Case::Upper => write!(f, "I"),
                    Case::Lower => write!(f, "i"),
                },
            },
            Key::J => match self.case {
                Case::Upper => write!(f, "J"),
                Case::Lower => write!(f, "j"),
            },
            Key::K => match self.case {
                Case::Upper => write!(f, "K"),
                Case::Lower => write!(f, "k"),
            },
            Key::L => match self.case {
                Case::Upper => write!(f, "L"),
                Case::Lower => write!(f, "l"),
            },
            Key::LessThan => write!(f, "<"),
            Key::M => match self.case {
                Case::Upper => write!(f, "M"),
                Case::Lower => write!(f, "m"),
            },
            Key::Micro => write!(f, "µ"),
            Key::Minus => write!(f, "-"),
            Key::N => match self.case {
                Case::Upper => write!(f, "N"),
                Case::Lower => write!(f, "n"),
            },
            Key::Nine => write!(f, "9"),
            Key::O => match self.modifier {
                Modifier::AccentAcute => match self.case {
                    Case::Upper => write!(f, "Ó"),
                    Case::Lower => write!(f, "ó"),
                },
                Modifier::AccentGrave => match self.case {
                    Case::Upper => write!(f, "Ò"),
                    Case::Lower => write!(f, "ò"),
                },
                Modifier::Caret => match self.case {
                    Case::Upper => write!(f, "Ô"),
                    Case::Lower => write!(f, "ô"),
                },
                Modifier::Umlaut => match self.case {
                    Case::Upper => write!(f, "Ö"),
                    Case::Lower => write!(f, "ö"),
                },
                Modifier::None => match self.case {
                    Case::Upper => write!(f, "O"),
                    Case::Lower => write!(f, "o"),
                },
            },
            Key::One => write!(f, "1"),
            Key::P => match self.case {
                Case::Upper => write!(f, "P"),
                Case::Lower => write!(f, "p"),
            },
            Key::Percent => write!(f, "%"),
            Key::Plus => write!(f, "+"),
            Key::PowerOfThree => write!(f, "³"),
            Key::PowerOfTwo => write!(f, "²"),
            Key::Q => match self.case {
                Case::Upper => write!(f, "Q"),
                Case::Lower => write!(f, "q"),
            },
            Key::QuestionMark => write!(f, "?"),
            Key::QuotationMark => write!(f, "\""),
            Key::R => match self.case {
                Case::Upper => write!(f, "R"),
                Case::Lower => write!(f, "r"),
            },
            Key::S => match self.case {
                Case::Upper => write!(f, "S"),
                Case::Lower => write!(f, "s"),
            },
            Key::Section => write!(f, "§"),
            Key::Semicolon => write!(f, ";"),
            Key::Seven => write!(f, "7"),
            Key::Six => write!(f, "6"),
            Key::Slash => write!(f, "/"),
            Key::Space => match self.modifier {
                Modifier::AccentAcute => write!(f, "´"),
                Modifier::AccentGrave => write!(f, "`"),
                Modifier::Caret => write!(f, "^"),
                _ => write!(f, " "),
            },
            Key::SquareBracketClose => write!(f, "]"),
            Key::SquareBracketOpen => write!(f, "["),
            Key::T => match self.case {
                Case::Upper => write!(f, "T"),
                Case::Lower => write!(f, "t"),
            },
            Key::Three => write!(f, "3"),
            Key::Tilde => write!(f, "~"),
            Key::Two => write!(f, "2"),
            Key::U => match self.modifier {
                Modifier::AccentAcute => match self.case {
                    Case::Upper => write!(f, "Ú"),
                    Case::Lower => write!(f, "ú"),
                },
                Modifier::AccentGrave => match self.case {
                    Case::Upper => write!(f, "Ù"),
                    Case::Lower => write!(f, "ù"),
                },
                Modifier::Caret => match self.case {
                    Case::Upper => write!(f, "Û"),
                    Case::Lower => write!(f, "û"),
                },
                Modifier::Umlaut => match self.case {
                    Case::Upper => write!(f, "Ü"),
                    Case::Lower => write!(f, "ü"),
                },
                Modifier::None => match self.case {
                    Case::Upper => write!(f, "U"),
                    Case::Lower => write!(f, "u"),
                },
            },
            Key::Underscore => write!(f, "_"),
            Key::V => match self.case {
                Case::Upper => write!(f, "V"),
                Case::Lower => write!(f, "v"),
            },
            Key::W => match self.case {
                Case::Upper => write!(f, "W"),
                Case::Lower => write!(f, "w"),
            },
            Key::X => match self.case {
                Case::Upper => write!(f, "X"),
                Case::Lower => write!(f, "x"),
            },
            Key::Y => match self.case {
                Case::Upper => write!(f, "Y"),
                Case::Lower => write!(f, "y"),
            },
            Key::Z => match self.case {
                Case::Upper => write!(f, "Z"),
                Case::Lower => write!(f, "z"),
            },
            Key::Zero => write!(f, "0"),
            _ => write!(f, ""),
        }
    }
}

impl Button {
    fn index(&self) -> usize {
        match self.key {
            Key::A => match self.modifier {
                Modifier::AccentAcute => match self.case {
                    Case::Upper => 60,
                    Case::Lower => 110,
                },
                Modifier::AccentGrave => match self.case {
                    Case::Upper => 65,
                    Case::Lower => 115,
                },
                Modifier::Caret => match self.case {
                    Case::Upper => 70,
                    Case::Lower => 120,
                },
                Modifier::Umlaut => match self.case {
                    Case::Upper => 56,
                    Case::Lower => 106,
                },
                Modifier::None => match self.case {
                    Case::Upper => 30,
                    Case::Lower => 80,
                },
            },
            Key::Ampersand => 135,
            Key::Apostrophe => 159,
            Key::Asterisk => 156,
            Key::At => 152,
            Key::B => match self.case {
                Case::Upper => 31,
                Case::Lower => 81,
            },
            Key::Backslash => 137,
            Key::Backspace => 3,
            Key::Bar => 162,
            Key::BracketClose => 147,
            Key::BracketOpen => 146,
            Key::C => match self.case {
                Case::Upper => 32,
                Case::Lower => 82,
            },
            Key::Colon => 167,
            Key::Comma => 164,
            Key::CurlyBracketClose => 149,
            Key::CurlyBracketOpen => 144,
            Key::D => match self.case {
                Case::Upper => 33,
                Case::Lower => 83,
            },
            Key::Degree => 141,
            Key::Dollar => 133,
            Key::Dot => 166,
            Key::E => match self.modifier {
                Modifier::AccentAcute => match self.case {
                    Case::Upper => 61,
                    Case::Lower => 111,
                },
                Modifier::AccentGrave => match self.case {
                    Case::Upper => 66,
                    Case::Lower => 116,
                },
                Modifier::Caret => match self.case {
                    Case::Upper => 71,
                    Case::Lower => 121,
                },
                _ => match self.case {
                    Case::Upper => 34,
                    Case::Lower => 84,
                },
            },
            Key::Eight => 27,
            Key::Equals => 138,
            Key::Escape => 0,
            Key::Euro => 153,
            Key::ExclamationMark => 130,
            Key::F => match self.case {
                Case::Upper => 35,
                Case::Lower => 85,
            },
            Key::Five => 24,
            Key::Four => 23,
            Key::G => match self.case {
                Case::Upper => 36,
                Case::Lower => 86,
            },
            Key::GreaterThan => 161,
            Key::H => match self.case {
                Case::Upper => 37,
                Case::Lower => 87,
            },
            Key::Hash => 158,
            Key::I => match self.modifier {
                Modifier::AccentAcute => match self.case {
                    Case::Upper => 62,
                    Case::Lower => 112,
                },
                Modifier::AccentGrave => match self.case {
                    Case::Upper => 67,
                    Case::Lower => 117,
                },
                Modifier::Caret => match self.case {
                    Case::Upper => 72,
                    Case::Lower => 122,
                },
                _ => match self.case {
                    Case::Upper => 38,
                    Case::Lower => 88,
                },
            },
            Key::J => match self.case {
                Case::Upper => 39,
                Case::Lower => 89,
            },
            Key::K => match self.case {
                Case::Upper => 40,
                Case::Lower => 90,
            },
            Key::L => match self.case {
                Case::Upper => 41,
                Case::Lower => 91,
            },
            Key::LessThan => 160,
            Key::Letters => match self.case {
                Case::Upper => 10,
                Case::Lower => 11,
            },
            Key::M => match self.case {
                Case::Upper => 42,
                Case::Lower => 92,
            },
            Key::Micro => 163,
            Key::Minus => 155,
            Key::Modifier => match self.modifier {
                Modifier::AccentAcute => 15,
                Modifier::AccentGrave => 16,
                Modifier::Caret => 17,
                Modifier::Umlaut => 14,
                Modifier::None => 13,
            },
            Key::N => match self.case {
                Case::Upper => 43,
                Case::Lower => 93,
            },
            Key::Nine => 28,
            Key::O => match self.modifier {
                Modifier::AccentAcute => match self.case {
                    Case::Upper => 63,
                    Case::Lower => 113,
                },
                Modifier::AccentGrave => match self.case {
                    Case::Upper => 68,
                    Case::Lower => 128,
                },
                Modifier::Caret => match self.case {
                    Case::Upper => 73,
                    Case::Lower => 123,
                },
                Modifier::Umlaut => match self.case {
                    Case::Upper => 57,
                    Case::Lower => 107,
                },
                Modifier::None => match self.case {
                    Case::Upper => 44,
                    Case::Lower => 94,
                },
            },
            Key::One => 20,
            Key::P => match self.case {
                Case::Upper => 45,
                Case::Lower => 95,
            },
            Key::Percent => 134,
            Key::Plus => 154,
            Key::PowerOfThree => 143,
            Key::PowerOfTwo => 142,
            Key::Q => match self.case {
                Case::Upper => 46,
                Case::Lower => 96,
            },
            Key::QuestionMark => 139,
            Key::QuotationMark => 131,
            Key::R => match self.case {
                Case::Upper => 47,
                Case::Lower => 97,
            },
            Key::Return => 4,
            Key::S => match self.case {
                Case::Upper => 48,
                Case::Lower => 98,
            },
            Key::Section => 132,
            Key::Semicolon => 165,
            Key::Seven => 26,
            Key::Six => 25,
            Key::Slash => 136,
            Key::Space => match self.modifier {
                Modifier::AccentAcute => 150,
                Modifier::AccentGrave => 151,
                Modifier::Caret => 140,
                _ => 2,
            },
            Key::SquareBracketClose => 148,
            Key::SquareBracketOpen => 145,
            Key::Symbols => 12,
            Key::T => match self.case {
                Case::Upper => 49,
                Case::Lower => 99,
            },
            Key::Three => 22,
            Key::Tilde => 157,
            Key::Two => 21,
            Key::U => match self.modifier {
                Modifier::AccentAcute => match self.case {
                    Case::Upper => 64,
                    Case::Lower => 114,
                },
                Modifier::AccentGrave => match self.case {
                    Case::Upper => 69,
                    Case::Lower => 129,
                },
                Modifier::Caret => match self.case {
                    Case::Upper => 74,
                    Case::Lower => 124,
                },
                Modifier::Umlaut => match self.case {
                    Case::Upper => 58,
                    Case::Lower => 108,
                },
                Modifier::None => match self.case {
                    Case::Upper => 50,
                    Case::Lower => 100,
                },
            },
            Key::Underscore => 168,
            Key::V => match self.case {
                Case::Upper => 51,
                Case::Lower => 101,
            },
            Key::W => match self.case {
                Case::Upper => 52,
                Case::Lower => 102,
            },
            Key::X => match self.case {
                Case::Upper => 53,
                Case::Lower => 103,
            },
            Key::Y => match self.case {
                Case::Upper => 54,
                Case::Lower => 104,
            },
            Key::Z => match self.case {
                Case::Upper => 55,
                Case::Lower => 105,
            },
            Key::Zero => 29,
        }
    }

    fn position(&self) -> Position {
        match self.key {
            Key::A => Position { x: 0, y: 3 },
            Key::Ampersand => Position { x: 5, y: 4 },
            Key::Apostrophe => Position { x: 0, y: 1 },
            Key::Asterisk => Position { x: 8, y: 3 },
            Key::At => Position { x: 4, y: 3 },
            Key::B => Position { x: 1, y: 3 },
            Key::Backslash => Position { x: 7, y: 4 },
            Key::Backspace => Position { x: 6, y: 0 },
            Key::Bar => Position { x: 3, y: 1 },
            Key::BracketClose => Position { x: 4, y: 2 },
            Key::BracketOpen => Position { x: 3, y: 2 },
            Key::C => Position { x: 2, y: 3 },
            Key::Colon => Position { x: 8, y: 1 },
            Key::Comma => Position { x: 5, y: 1 },
            Key::CurlyBracketClose => Position { x: 6, y: 2 },
            Key::CurlyBracketOpen => Position { x: 1, y: 2 },
            Key::D => Position { x: 3, y: 3 },
            Key::Degree => Position { x: 1, y: 3 },
            Key::Dollar => Position { x: 3, y: 4 },
            Key::Dot => Position { x: 7, y: 1 },
            Key::E => Position { x: 4, y: 3 },
            Key::Eight => Position { x: 7, y: 4 },
            Key::Equals => Position { x: 8, y: 4 },
            Key::Escape => Position { x: 2, y: 0 },
            Key::Euro => Position { x: 5, y: 3 },
            Key::ExclamationMark => Position { x: 0, y: 4 },
            Key::F => Position { x: 5, y: 3 },
            Key::Five => Position { x: 4, y: 4 },
            Key::Four => Position { x: 3, y: 4 },
            Key::G => Position { x: 6, y: 3 },
            Key::GreaterThan => Position { x: 2, y: 1 },
            Key::H => Position { x: 7, y: 3 },
            Key::Hash => Position { x: 8, y: 2 },
            Key::I => Position { x: 8, y: 3 },
            Key::J => Position { x: 9, y: 3 },
            Key::K => Position { x: 0, y: 2 },
            Key::L => Position { x: 1, y: 2 },
            Key::LessThan => Position { x: 1, y: 1 },
            Key::Letters => Position { x: 3, y: 0 },
            Key::M => Position { x: 2, y: 2 },
            Key::Micro => Position { x: 4, y: 1 },
            Key::Minus => Position { x: 7, y: 3 },
            Key::Modifier => Position { x: 4, y: 0 },
            Key::N => Position { x: 3, y: 2 },
            Key::Nine => Position { x: 8, y: 4 },
            Key::O => Position { x: 4, y: 2 },
            Key::One => Position { x: 0, y: 4 },
            Key::P => Position { x: 5, y: 2 },
            Key::Percent => Position { x: 4, y: 4 },
            Key::Plus => Position { x: 6, y: 3 },
            Key::PowerOfThree => Position { x: 3, y: 3 },
            Key::PowerOfTwo => Position { x: 2, y: 3 },
            Key::Q => Position { x: 6, y: 2 },
            Key::QuestionMark => Position { x: 9, y: 4 },
            Key::QuotationMark => Position { x: 1, y: 4 },
            Key::R => Position { x: 7, y: 2 },
            Key::Return => Position { x: 7, y: 0 },
            Key::S => Position { x: 8, y: 2 },
            Key::Section => Position { x: 2, y: 4 },
            Key::Semicolon => Position { x: 6, y: 1 },
            Key::Seven => Position { x: 6, y: 4 },
            Key::Six => Position { x: 5, y: 4 },
            Key::Slash => Position { x: 6, y: 4 },
            Key::Space => Position { x: 5, y: 0 },
            Key::SquareBracketClose => Position { x: 5, y: 2 },
            Key::SquareBracketOpen => Position { x: 2, y: 2 },
            Key::Symbols => Position { x: 3, y: 0 },
            Key::T => Position { x: 9, y: 2 },
            Key::Three => Position { x: 2, y: 4 },
            Key::Tilde => Position { x: 7, y: 2 },
            Key::Two => Position { x: 1, y: 4 },
            Key::U => Position { x: 2, y: 1 },
            Key::Underscore => Position { x: 9, y: 1 },
            Key::V => Position { x: 3, y: 1 },
            Key::W => Position { x: 4, y: 1 },
            Key::X => Position { x: 5, y: 1 },
            Key::Y => Position { x: 6, y: 1 },
            Key::Z => Position { x: 7, y: 1 },
            Key::Zero => Position { x: 9, y: 4 },
        }
    }
}

#[derive(Clone, Copy)]
pub enum Key {
    A,
    Ampersand,
    Apostrophe,
    Asterisk,
    At,
    B,
    Backslash,
    Backspace,
    Bar,
    BracketClose,
    BracketOpen,
    C,
    Colon,
    Comma,
    CurlyBracketClose,
    CurlyBracketOpen,
    D,
    Degree,
    Dollar,
    Dot,
    E,
    Eight,
    Equals,
    Escape,
    Euro,
    ExclamationMark,
    F,
    Five,
    Four,
    G,
    GreaterThan,
    H,
    Hash,
    I,
    J,
    K,
    L,
    LessThan,
    Letters,
    M,
    Micro,
    Minus,
    Modifier,
    N,
    Nine,
    O,
    One,
    P,
    Percent,
    Plus,
    PowerOfThree,
    PowerOfTwo,
    Q,
    QuestionMark,
    QuotationMark,
    R,
    Return,
    S,
    Section,
    Semicolon,
    Seven,
    Six,
    Slash,
    Space,
    SquareBracketClose,
    SquareBracketOpen,
    Symbols,
    T,
    Three,
    Tilde,
    Two,
    U,
    Underscore,
    V,
    W,
    X,
    Y,
    Z,
    Zero,
}

pub struct VKeyboardPlugin;
impl Plugin for VKeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Button>()
            .add_system_set(SystemSet::on_enter(AppState::Register).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Register).with_system(cleanup))
            .add_system_set(
                SystemSet::on_update(AppState::Register)
                    .with_system(
                        input_mouse
                            .after(InputState::Keyboard)
                            .after(InputState::Mouse),
                    )
                    .with_system(listen_buttons)
                    .with_system(
                        update_buttons
                            .after(InputState::Keyboard)
                            .after(InputState::Mouse),
                    ),
            );
    }
}

#[derive(Clone, Copy)]
enum Case {
    Lower,
    Upper,
}

enum Layout {
    Letters,
    Symbols,
}

impl Layout {
    fn buttons(&self, case: Case, modifier: Modifier) -> Vec<Button> {
        let mut layout = vec![
            Button {
                case,
                key: Key::Escape,
                modifier,
            },
            Button {
                case,
                key: Key::Space,
                modifier,
            },
            Button {
                case,
                key: Key::Backspace,
                modifier,
            },
            Button {
                case,
                key: Key::Return,
                modifier,
            },
        ];

        match self {
            Layout::Letters => {
                match case {
                    Case::Upper => layout.push(Button {
                        case: Case::Lower,
                        key: Key::Letters,
                        modifier,
                    }),
                    Case::Lower => layout.push(Button {
                        case,
                        key: Key::Symbols,
                        modifier,
                    }),
                }

                layout.append(&mut vec![
                    Button {
                        case,
                        key: Key::Modifier,
                        modifier: modifier.next(),
                    },
                    Button {
                        case,
                        key: Key::One,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::Two,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::Three,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::Four,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::Five,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::Six,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::Seven,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::Eight,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::Nine,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::Zero,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::A,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::B,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::C,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::D,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::E,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::F,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::G,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::H,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::I,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::J,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::K,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::L,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::M,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::N,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::O,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::P,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::Q,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::R,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::S,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::T,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::U,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::V,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::W,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::X,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::Y,
                        modifier,
                    },
                    Button {
                        case,
                        key: Key::Z,
                        modifier,
                    },
                ]);
            }
            Layout::Symbols => layout.append(&mut vec![
                Button {
                    case: Case::Upper,
                    key: Key::Letters,
                    modifier,
                },
                Button {
                    case,
                    key: Key::ExclamationMark,
                    modifier,
                },
                Button {
                    case,
                    key: Key::QuotationMark,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Section,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Dollar,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Percent,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Ampersand,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Slash,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Backslash,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Equals,
                    modifier,
                },
                Button {
                    case,
                    key: Key::QuestionMark,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Degree,
                    modifier,
                },
                Button {
                    case,
                    key: Key::PowerOfTwo,
                    modifier,
                },
                Button {
                    case,
                    key: Key::PowerOfThree,
                    modifier,
                },
                Button {
                    case,
                    key: Key::CurlyBracketOpen,
                    modifier,
                },
                Button {
                    case,
                    key: Key::SquareBracketOpen,
                    modifier,
                },
                Button {
                    case,
                    key: Key::BracketOpen,
                    modifier,
                },
                Button {
                    case,
                    key: Key::BracketClose,
                    modifier,
                },
                Button {
                    case,
                    key: Key::SquareBracketClose,
                    modifier,
                },
                Button {
                    case,
                    key: Key::CurlyBracketClose,
                    modifier,
                },
                Button {
                    case,
                    key: Key::At,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Euro,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Plus,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Minus,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Asterisk,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Tilde,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Hash,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Apostrophe,
                    modifier,
                },
                Button {
                    case,
                    key: Key::LessThan,
                    modifier,
                },
                Button {
                    case,
                    key: Key::GreaterThan,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Bar,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Micro,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Comma,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Semicolon,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Dot,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Colon,
                    modifier,
                },
                Button {
                    case,
                    key: Key::Underscore,
                    modifier,
                },
            ]),
        }

        layout
    }
}

#[derive(Clone, Copy)]
enum Modifier {
    AccentAcute,
    AccentGrave,
    Caret,
    None,
    Umlaut,
}

impl Modifier {
    fn next(&self) -> Modifier {
        match self {
            Modifier::AccentAcute => Modifier::AccentGrave,
            Modifier::AccentGrave => Modifier::Caret,
            Modifier::Caret => Modifier::Umlaut,
            Modifier::None => Modifier::AccentAcute,
            Modifier::Umlaut => Modifier::None,
        }
    }
}

struct Position {
    x: usize,
    y: usize,
}

#[derive(Component)]
struct VKeyboardComponent;

fn cleanup(mut commands: Commands, query: Query<Entity, With<VKeyboardComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn input_mouse(
    mut event_writer: EventWriter<Button>,
    input: Res<Input<MouseButton>>,
    input_state: Res<InputState>,
    query: Query<(&Button, &Transform)>,
    windows: Res<Windows>,
) {
    if !vec![InputState::Mouse].contains(&input_state) {
        return;
    }

    if input.just_pressed(MouseButton::Left) {
        let wnd = windows.get_primary().unwrap();
        if let Some(mut cursor) = wnd.cursor_position() {
            cursor -= 0.5 * Vec2::new(wnd.width(), wnd.height());
            let contains = |p: Vec2, a: Rect<f32>| {
                p.x > a.left && p.x < a.right && p.y > a.bottom && p.y < a.top
            };

            for (btn, tf) in query.iter() {
                let offs = 0.5 * tf.scale;
                let rect = Rect {
                    bottom: tf.translation.y - offs.y,
                    left: tf.translation.x - offs.x,
                    right: tf.translation.x + offs.x,
                    top: tf.translation.y + offs.y,
                };

                if contains(cursor, rect) {
                    event_writer.send(*btn);
                }
            }
        }
    }
}

fn listen_buttons(
    mut commands: Commands,
    mut event_reader: EventReader<Button>,
    query: Query<Entity, With<Button>>,
    sheets: Res<SpriteSheetAssets>,
) {
    let mut spawn_layout = |l: Vec<Button>| {
        for e in query.iter() {
            commands.entity(e).despawn();
        }

        for btn in l.iter() {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(Vec2::ONE),
                        index: btn.index(),
                        ..Default::default()
                    },
                    texture_atlas: sheets.keys.clone(),
                    ..Default::default()
                })
                .insert(VKeyboardComponent)
                .insert(*btn);
        }
    };

    for btn in event_reader.iter() {
        match btn.key {
            Key::Letters | Key::Modifier => {
                spawn_layout(Layout::Letters.buttons(btn.case, btn.modifier))
            }
            Key::Symbols => spawn_layout(Layout::Symbols.buttons(btn.case, Modifier::None)),
            _ => (),
        }
    }
}

fn setup(mut commands: Commands, mut event_writer: EventWriter<Button>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(VKeyboardComponent);

    event_writer.send(Button {
        case: Case::Upper,
        key: Key::Letters,
        modifier: Modifier::None,
    });
}

fn update_buttons(
    input_state: Res<InputState>,
    mut query: Query<(&Button, &mut TextureAtlasSprite, &mut Transform)>,
    windows: Res<Windows>,
) {
    let wnd = windows.get_primary().unwrap();
    let height = wnd.height();
    let width = wnd.width();
    let blk = height.min(width) / COLS as f32;
    let blk_pad = blk - PAD * blk;
    let offs_blk = 0.5 * blk;
    let offs_x = 0.5 * COLS as f32 * blk;
    let offs_y = 0.5 * height - PAD * blk;
    for (btn, mut tex, mut tf) in query.iter_mut() {
        tex.color = match *input_state {
            InputState::Keyboard => Color::NONE,
            InputState::Mouse => COLOR,
        };

        tf.scale.x = blk_pad;
        tf.scale.y = blk_pad;
        tf.translation.x = btn.position().x as f32 * blk + offs_blk - offs_x;
        tf.translation.y = btn.position().y as f32 * blk + offs_blk - offs_y;
    }
}
