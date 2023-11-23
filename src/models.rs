use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct urls {
    pub id: i32,
    pub url: Vec<String>,
    pub uid: String,
    pub pswd: String,
}