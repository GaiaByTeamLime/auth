use rocket::futures::TryFutureExt;
use rocket::{get, State, Responder};
use rocket_firebase_auth::bearer_token::BearerToken;
use crate::ServerState;

#[derive(Debug, Responder)]
pub enum CreateResponse {
    #[response(status = 200)]
    Created(String),
    #[response(status = 500)]
    Error(String),
    #[response(status = 403)]
    Forbidden(()),
}

#[get("/create/from_firebase")]
pub async fn create_from_firebase(state: &State<ServerState>, token: BearerToken) -> CreateResponse {
    match state
        .firebase
        .verify(&token)
        .map_ok(|decoded_token| decoded_token.uid)
        .await
    {
        Err(_) => CreateResponse::Forbidden(()),
        Ok(uid) => {
            match state.token.new_token(uid).await {
                Err(e) => CreateResponse::Error(e.to_string()),
                Ok(t) => CreateResponse::Created(t),
            }
        }
    }
}