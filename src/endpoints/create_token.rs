use rocket::futures::TryFutureExt;
use rocket::{get, State, Responder, serde::{Serialize, json::Json}};
use rocket_firebase_auth::bearer_token::BearerToken;
use crate::ServerState;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CreatedResponseBody {
    pub token: String,
    pub uid: String,
}

#[derive(Debug, Responder)]
pub enum CreateResponse {
    #[response(status = 200)]
    Created(Json<CreatedResponseBody>),
    #[response(status = 500)]
    Error(String),
    #[response(status = 403)]
    Forbidden(()),
}

#[get("/create")]
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
                Ok((sensor_token, sensor_uid)) => CreateResponse::Created(Json(CreatedResponseBody {
                    token: sensor_token,
                    uid: sensor_uid,
                })),
            }
        }
    }
}