use ormx::Table;
use chrono::NaiveDateTime;

#[derive(Debug, Table)]
#[ormx(table = "tokens", id = token_id, insertable, deletable)]
pub struct Token {
    #[ormx(column = "id")]
    #[ormx(get_optional(i32))]
    #[ormx(default)]
    pub token_id: i32,

    #[ormx(get_optional(&str))]
    pub token: String,

    #[ormx(get_optional(&str))]
    pub sensor_mac: String,

    #[ormx(get_optional(&str))]
    pub firebase_uid: String,

    #[ormx(default)]
    pub created: NaiveDateTime,

    #[ormx(default, set)]
    pub last_used: NaiveDateTime,
}