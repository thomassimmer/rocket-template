use rocket::fs::NamedFile;
use rocket::Request;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![index, favicon]
}

#[get("/")]
fn index() -> &'static str {
    "Rocket-template is running."
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("src/static/favicon.ico").await.ok()
}

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}
