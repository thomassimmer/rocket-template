use std::env;

use rocket::local::asynchronous::Client;
use rocket_template::rocket;

// Notes:
// For our integration tests, we use an in-memory database so we don't have to
// deal with a sqlite file dedicated for testing. This is not very efficient,
// because every test using an instance of TestContext will create a database
// in memory, run the migrations, and then execute itself. It's the best
// solution I could come with at the time of writing.
// The positive effect of this is we can run test in parallel because they all
// act on a different database.

pub struct TestContext {
    pub client: Client,
}

impl TestContext {
    pub async fn new() -> Self {
        env::set_var(
            "ROCKET_DATABASES",
            "{rocket_template_db={url=\":memory:\",pool_size=1,timeout=5}}",
        );

        let client = Client::tracked(rocket())
            .await
            .expect("valid rocket instance");

        // Add other properties that you need here...

        Self { client }
    }
}
