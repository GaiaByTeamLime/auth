use rocket::{routes, Build, Rocket};
use rocket_firebase_auth::{
    auth::FirebaseAuth
};

struct ServerState {
    pub auth: FirebaseAuth
}

#[rocket::launch]
async fn rocket() -> Rocket<Build> {
    let firebase_auth = FirebaseAuth::try_from_json_file("firebase-credentials.json")
        .expect("Failed to read Firebase credentials");

    rocket::build()
        .mount("/", routes![hello_world])
        .manage(ServerState {
            auth: firebase_auth
        })
}