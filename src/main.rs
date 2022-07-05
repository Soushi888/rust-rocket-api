#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct MyForm {
    #[field(default = "Sacha")]
    name: String,

    #[field(default = 42)]
    age: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    name: String,
    age: u8,
}

#[get("/json")]
fn json() -> Json<User> {
    Json(User {
        name: "John".to_string(),
        age: 30,
    })
}

#[get("/form")]
fn get_form() -> Json<MyForm> {
    Json(MyForm {
        name: "Sacha".to_string(),
        age: 42,
    })
}

#[post("/form", data = "<form>")]
fn form(form: Form<MyForm>) -> String {
    format!("Hello, {}! You are {} years old.", form.name, form.age)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, hello, json, form, get_form])
        .launch()
        .await?;

    Ok(())
}