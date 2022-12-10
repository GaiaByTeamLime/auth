#![feature(associated_type_bounds)]

use rocket::{Build, Rocket};
use rocket_firebase_auth::auth::FirebaseAuth;
use token_auth::TokenAuth;


mod endpoints;
mod token_auth;

pub struct ServerState {
    pub firebase: FirebaseAuth,
    pub token: TokenAuth,
}

#[rocket::launch]
async fn rocket() -> Rocket<Build> {    
    dotenv::dotenv().ok();
    
    let firebase_auth = FirebaseAuth::try_from_json_file("firebase-credentials.json")
        .expect("Failed to read Firebase credentials");

    let token_auth: TokenAuth = TokenAuth::new(&dotenv::var("DATABASE_URL").unwrap()).await
        .expect("Failed to connect to the database!");

    rocket::build()
        .mount("/", endpoints::get_routes())
        .manage(ServerState {
            firebase: firebase_auth,
            token: token_auth,
        })
}