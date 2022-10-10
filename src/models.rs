use crate::schema::items;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub time_deleted: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub time_deleted: Option<DateTime<Utc>>,
}
