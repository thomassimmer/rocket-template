#[rocket::main]
async fn main() {
    let _result = rocket_template::rocket().launch().await;

    println!("Rocket: deorbit.");
}
