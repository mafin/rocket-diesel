use rocket::serde::{Serialize, Deserialize};
use diesel::{Insertable, Queryable};

use crate::schema::blog_posts;

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "blog_posts"]
pub struct BlogPost {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
