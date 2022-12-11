use rocket::{Build, Rocket, Config, figment::Figment};
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
    openssl_probe::init_ssl_cert_env_vars();
    
    let firebase_auth = FirebaseAuth::try_from_json_file("firebase-credentials.json")
        .expect("Failed to read Firebase credentials");

    let token_auth: TokenAuth = TokenAuth::new(&dotenv::var("DATABASE_URL").unwrap()).await
        .expect("Failed to connect to the database!");

    println!("Starting rocket on 0.0.0.0:5000");

    let figment = Figment::from(Config::default())
        .merge((Config::PORT, 5000))
        .merge((Config::ADDRESS, "0.0.0.0"));

    rocket::custom(figment)
        .mount("/", endpoints::get_routes())
        .manage(ServerState {
            firebase: firebase_auth,
            token: token_auth,
        })
}