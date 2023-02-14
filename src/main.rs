#[macro_use] extern crate rocket;
#[macro_use] extern crate serde;

use rocket::serde::json::Json;
use rocket::State;
use rocket::fairing::AdHoc;
use rocket_diesel::models::BlogPost;
use rocket_diesel::{Db, schema};
use diesel::RunQueryDsl;

#[derive(Deserialize)]
struct Config {
    name: String,
    age: u8,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/random")]
fn get_random_blog_post() -> Json<BlogPost> {
    Json(
        BlogPost {
            id: 1,
            title: "My first post".to_string(),
            body: "This is my first post".to_string(),
            published: true,
        }
    )
}

#[get("/<id>")]
fn get_blog_post(id: i32) -> Json<BlogPost> {
    Json(
        BlogPost {
            id,
            title: "Some title".to_string(),
            body: "Some body".to_string(),
            published: true,
        }
    )
}

// #[get("/")]
// fn get_all_blog_posts() -> Json<Vec<BlogPost>> {
//     Json(vec![
//         BlogPost {
//             id: 0,
//             title: "Harry Potter".to_string(),
//             body: "There once lived a boy".to_string(),
//             published: true,
//         },
//         BlogPost {
//             id: 1,
//             title: "Fantastic Beast".to_string(),
//             body: "There once lived a beast".to_string(),
//             published: true,
//         }
//     ]
//     )
// }
#[get("/")]
async fn get_all_blog_posts(connection: Db) -> Json<Vec<BlogPost>> {
    connection
        .run(|c| schema::blog_posts::table.load(c))
        .await
        .map(Json)
        .expect("Failed to fetch blog posts")
}

// #[post("/", data = "<blog_post>")]
// fn create_blog_post(blog_post: Json<BlogPost>) -> Json<BlogPost> {
//     blog_post
// }
#[post("/", data = "<blog_post>")]
async fn create_blog_post(
    connection: Db,
    blog_post: Json<BlogPost>,
) -> Json<BlogPost> {
    connection
        .run(move |c| {
            diesel::insert_into(schema::blog_posts::table)
                .values(&blog_post.into_inner())
                .get_result(c)
        })
        .await
        .map(Json)
        .expect("boo")
}

#[get("/config")]
fn get_config(config: &State<Config>) -> String {
    format!(
        "Hello, {}! You are {} years old.", config.name, config.age
    )
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    rocket
        .attach(Db::fairing())
        .attach(AdHoc::config::<Config>())
        .mount("/",
               routes![
                   index,
                   get_config
               ]
        )
        .mount("/blog-posts",
               routes![
                   get_random_blog_post,
                   get_blog_post,
                   get_all_blog_posts,
                   create_blog_post,
               ],
        )
}
