use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub enum Potency {
    #[default]
    Low = 1,
    Mid = 2,
    High = 3,
}
impl From<Potency> for i32 {
    fn from(v: Potency) -> i32 {
        match v {
            Potency::Low => 1,
            Potency::Mid => 2,
            Potency::High => 3,
        }
    }
}
impl fmt::Display for Potency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Potency::Low => "Low",
            Potency::Mid => "Mid",
            Potency::High => "High",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Copy, Clone, Serialize, PartialEq, Eq, Hash, Default, Deserialize)]
#[serde(try_from = "IntermediateData")]
pub enum Modifier {
    AttackUp,
    DefenseUp,
    ResistCold,
    ResistHot,
    ResistElectric,
    Fireproof,
    MovingSpeed,
    Quietness,
    LifeMaxUp,
    GutsRecover,
    ExGutsMaxUp,
    LifeRecover,
    #[default]
    None,
}

impl Modifier {
    pub fn threshold(&self) -> [i32; 2] {
        match self {
            Modifier::AttackUp => [5, 7],
            Modifier::DefenseUp => [5, 7],
            Modifier::ResistCold => [6, 999],
            Modifier::ResistHot => [6, 999],
            Modifier::ResistElectric => [4, 6],
            Modifier::Fireproof => [7, 999],
            Modifier::MovingSpeed => [5, 7],
            Modifier::Quietness => [6, 9],
            Modifier::LifeMaxUp => [999, 999],
            Modifier::GutsRecover => [999, 999],
            Modifier::ExGutsMaxUp => [999, 999],
            Modifier::LifeRecover => [999, 999],
            Modifier::None => [999, 999],
        }
    }
    pub fn elixir(&self) -> &str {
        match self {
            Modifier::AttackUp => "Mighty Elixir",
            Modifier::DefenseUp => "Tough Elixir",
            Modifier::ResistCold => "Spicy Elixir",
            Modifier::ResistHot => "Chilly Elixir",
            Modifier::ResistElectric => "Electro Elixir",
            Modifier::Fireproof => "Fireproof Elixir",
            Modifier::MovingSpeed => "Hasty Elixir",
            Modifier::Quietness => "Sneaky Elixir",
            Modifier::ExGutsMaxUp => "Enduring Elixir",
            Modifier::GutsRecover => "Energizing Elixir",
            Modifier::LifeMaxUp => "Hearty Elixir",
            Modifier::LifeRecover => "",
            Modifier::None => "",
        }
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Modifier::AttackUp => "AttackUp",
            Modifier::DefenseUp => "DefenseUp",
            Modifier::ResistCold => "ResistCold",
            Modifier::ResistHot => "ResistHot",
            Modifier::ResistElectric => "ResistElectric",
            Modifier::Fireproof => "Fireproof",
            Modifier::MovingSpeed => "MovingSpeed",
            Modifier::Quietness => "Quietness",
            Modifier::ExGutsMaxUp => "ExGutsMaxUp",
            Modifier::GutsRecover => "GutsRecover",
            Modifier::LifeMaxUp => "LifeMaxUp",
            Modifier::LifeRecover => "LifeRecover",
            Modifier::None => "None",
        };
        write!(f, "{}", s)
    }
}

#[derive(Deserialize, Debug, Default)]
enum IntermediateData {
    AttackUp,
    DefenseUp,
    ResistCold,
    ResistHot,
    ResistElectric,
    Fireproof,
    MovingSpeed,
    Quietness,
    LifeMaxUp,
    GutsRecover,
    ExGutsMaxUp,
    LifeRecover,
    #[default]
    None,
    #[serde(alias = "")]
    Empty,
}
use std::convert::From;
impl From<IntermediateData> for Modifier {
    fn from(data: IntermediateData) -> Self {
        match data {
            IntermediateData::AttackUp => Modifier::AttackUp,
            IntermediateData::DefenseUp => Modifier::DefenseUp,
            IntermediateData::ResistCold => Modifier::ResistCold,
            IntermediateData::ResistHot => Modifier::ResistHot,
            IntermediateData::ResistElectric => Modifier::ResistElectric,
            IntermediateData::Fireproof => Modifier::Fireproof,
            IntermediateData::MovingSpeed => Modifier::MovingSpeed,
            IntermediateData::Quietness => Modifier::Quietness,
            IntermediateData::LifeMaxUp => Modifier::LifeMaxUp,
            IntermediateData::GutsRecover => Modifier::GutsRecover,
            IntermediateData::ExGutsMaxUp => Modifier::ExGutsMaxUp,
            IntermediateData::LifeRecover => Modifier::LifeRecover,
            IntermediateData::None => Modifier::None,
            IntermediateData::Empty => Modifier::None,
        }
    }
}
