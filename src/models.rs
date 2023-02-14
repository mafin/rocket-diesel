use diesel::prelude::*;
use diesel::{Insertable, Queryable};

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
// #[table_name = "blog_posts"]
#[diesel(table_name = blog_posts)]
struct BlogPost {
    id: i32,
    title: String,
    body: String,
    published: bool,
}
