use rocket::{Route, routes};

pub mod create_token;
pub mod verify_token;

pub fn get_routes() -> Vec<Route> {
    routes![
        create_token::create_from_firebase, 
        verify_token::verify_firebase, verify_token::verify_token
    ]
}