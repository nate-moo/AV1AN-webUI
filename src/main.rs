#[macro_use] extern crate rocket;

use rocket::response::Redirect;
use rocket_dyn_templates::{Template, handlebars, context};

#[get("/")]
fn index() -> Template {
    Template::render("hbs/index", context! {
        title: "Hello",
        name: Some("Beans"),
        items: vec!["One", "Two", "Three"],
        Title: "Index",
    })
}

#[get("/api/<int>")]
fn api(int: i32) -> String {
    format!("number {}", int)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, api])
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

    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write("</b></i>")?;
    }

    Ok(())
}