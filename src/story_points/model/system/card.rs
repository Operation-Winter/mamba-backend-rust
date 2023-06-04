use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Card {
    Zero,
    One,
    Two,
    Three,
    Five,
    Eight,
    Thirteen,
    Twenty,
    Fourty,
    Hundred,
    Question,
}