use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Fupn {
    pub creation_date: NaiveDateTime,
    pub id: i32,
    pub description: String,
    pub update_date: NaiveDateTime,
    pub phone_number: String,
}
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct NewFupn {
    pub fupn: Fupn,
    pub img_url: String,
}
#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct Fupns {
    pub fupns: Vec<Fupn>,
}
