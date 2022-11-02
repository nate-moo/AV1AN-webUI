#[macro_use] extern crate rocket;

use rocket::fs::FileServer;
use rocket::Request;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, handlebars, Template};

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

#[post("/api/start")]
fn start() -> Redirect {
    let id = 12345;
    Redirect::to(format!("/progress/{}", id))
}

#[get("/progress/<id>")]
fn progress(id: i32) -> Template {
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
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, api, start, progress])
        .register("/", catchers![not_found])
        .mount("/public", FileServer::from("static"))
        .attach(Template::custom(|engines| {
            engines.handlebars.register_helper("wow", Box::new(wow_helper));
        }))
}

fn wow_helper(
    h: &handlebars::Helper<'_, '_>,
    _: &handlebars::Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext<'_, '_>,
    out: &mut dyn handlebars::Output
) -> handlebars::HelperResult {
    Ok(())
}