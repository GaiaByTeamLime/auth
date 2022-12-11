use rocket_firebase_auth::bearer_token::BearerToken;
use sqlx::PgPool;
use ormx::Insert;
use token::{Token, InsertToken};

mod token;

pub struct TokenAuth {
    database: PgPool,
}

pub enum Error {
    Sql(sqlx::Error),
    Verify(String),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        Self::Sql(error)
    }
}
impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Self::Sql(e) => e.to_string(),
            Self::Verify(e) => e.to_string(),
        }
    }
}

impl TokenAuth {
    pub async fn new(connection: &str) -> Result<Self, sqlx::Error> {
        let db = PgPool::connect(connection).await?;

        Ok(Self {
            database: db,
        })
    }

    pub async fn verify(&self, token: &BearerToken) -> Result<Token, Error> {
        match Token::by_token(&self.database, token.as_str()).await? {
            Some(t) => Ok(t),
            None => Err(Error::Verify("Token does not exist".to_string()))
        }
    }

    pub async fn new_token(&self, uid: String) -> Result<String, Error> {
        // this generates a 36-char long string with a secure random number generator
        let rng = rand::random::<[u8; 27]>(); 
        let token = base64::encode(rng);
        let mut db = self.database.acquire().await?;

        let _new = InsertToken {
            token: token.clone(),
            firebase_uid: uid.clone(),
        }
        .insert(&mut db)
        .await?;

        Ok(token.to_string())
    }
}