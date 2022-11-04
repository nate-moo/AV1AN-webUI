#[macro_use]
extern crate rocket;

use std::thread::sleep;

use rand::Rng;
use rocket::{Build, Request, Rocket};
use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, handlebars, Template};

#[derive(FromForm)]
struct Args {
    input: String,
    output: String,
    encoder: String,
    video: String,
    audio: String,
    submit: String,
}

#[get("/")]
fn index() -> Template {
    Template::render("hbs/index", context! {
        title: "Params",
    })
}

#[get("/api/<int>")]
fn api(int: i32) -> String {
    format!("number {}", int)
}

#[get("/api/data/<id>")]
fn data_api(id: u32) -> String {
    format!("{}", rand::thread_rng().gen::<u32>())
}

#[post("/api/start", data = "<args>")]
fn start(args: Option<Form<Args>>) -> Redirect {
    let mut rng = rand::thread_rng();
    let id: u32 = rng.gen();
    Redirect::to(format!("/progress/{}", id))
}

#[get("/progress/<id>")]
fn progress(id: u32) -> Template {
    Template::render("hbs/progress", context! {
        title: "Progress",
        identifier: id,
    })
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("hbs/error/404", context! {
        uri: req.uri()
    })
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, api, start, progress, data_api])
        .register("/", catchers![not_found])
        .mount("/public", FileServer::from("static"))
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("wow", Box::new(wow_helper));
        }))
}

fn wow_helper(
    _: &handlebars::Helper<'_, '_>,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_, '_>,
    _: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    Ok(())
}