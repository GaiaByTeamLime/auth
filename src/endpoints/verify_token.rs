use rocket::{get, State, response::Responder};
use rocket::futures::TryFutureExt;
use rocket_firebase_auth::bearer_token::BearerToken;
use crate::ServerState;

#[derive(Debug, Responder)]
pub enum VerifyResponse {
    #[response(status = 200)]
    Valid(String),
    #[response(status = 403)]
    Forbidden(()),
}

#[get("/verify/firebase")]
pub async fn verify_firebase(state: &State<ServerState>, token: BearerToken) -> VerifyResponse {
    match state
        .firebase
        .verify(&token)
        .map_ok(|decoded_token| decoded_token.uid)
        .await
    {
        Ok(uid) => VerifyResponse::Valid(uid),
        Err(_) => VerifyResponse::Forbidden(()),
    }
}

#[get("/verify/token")]
pub async fn verify_token(state: &State<ServerState>, token: BearerToken) -> VerifyResponse {
    match state
        .token
        .verify(&token)
        .map_ok(|decoded_token| decoded_token.sensor_uid)
        .await
    {
        Ok(sensor_uid) => VerifyResponse::Valid(sensor_uid),
        Err(_) => VerifyResponse::Forbidden(()),
    }
}