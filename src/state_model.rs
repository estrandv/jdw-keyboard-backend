use std::cmp;
use std::collections::HashMap;
use std::fmt::format;
use std::hash::Hash;

use rosc::{OscMessage, OscPacket, OscType};
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(EnumIter, PartialEq, Debug, Clone, Eq, Hash)]
pub enum Letter {
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Z,
    X,
    C,
    V,
    B,
    N,
    M
}

impl Letter {
    pub fn from(letter: char) -> Result<Letter, String>{
        match letter {
            'a' => Ok(Letter::A),
            // TODO: rest of the damn alphabet...
            _ => Err(format!("{} is no a supported letter", letter))
        }
    }
}

struct PlaySampleMessageData {
    number: i32,
    pack_name: String
}

impl PlaySampleMessageData {
    fn new() -> PlaySampleMessageData {

        PlaySampleMessageData {
            number: 0,
            pack_name: "Roland808".to_string()
        }
    }

    pub fn modify_number(&mut self, amount:i32) {
        let changed = self.number + amount;
        self.number = cmp::max(changed, 0);
    }

    pub fn as_packet(&self) -> OscPacket {
        OscPacket::Message(OscMessage {
            addr: "/play_sample".to_string(),
            args: vec![
                OscType::String("letter_sample".to_string()), // ext_id
                OscType::String(self.pack_name.to_string()),
                OscType::Int(self.number),
                OscType::String("".to_string()),
                OscType::String("ofs".to_string()), // NOTE: should be modular
                OscType::Float(0.0),
            ],
        })
    }
}

pub struct LetterPayloads {
    payloads: HashMap<Letter, PlaySampleMessageData>
}

impl LetterPayloads {
    pub fn new() -> LetterPayloads {
        LetterPayloads {
            payloads: Letter::iter().map(|letter|
                (letter, PlaySampleMessageData::new())
            ).collect()
        }
    }

    pub fn modify_number(&mut self, letter: &Letter, amount: i32) {
        self.payloads.get_mut(letter).unwrap().modify_number(amount);
    }

    pub fn get_letter_packet(&self, letter: &Letter) -> OscPacket {
        // NOTE: Unsafe unwrap, but missing get implies we've messed up the defaults...
        self.payloads.get(letter).map(|data| data.as_packet()).unwrap()
    }
}