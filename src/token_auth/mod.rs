use rocket_firebase_auth::bearer_token::BearerToken;
use sqlx::PgPool;
use ormx::Insert;
use token::{Token, InsertToken};
use ring::{rand::{SystemRandom, SecureRandom}, digest};

const URL_SAFE_ENGINE: base64::engine::fast_portable::FastPortable =
    base64::engine::fast_portable::FastPortable::from(
        &base64::alphabet::URL_SAFE,
        base64::engine::fast_portable::NO_PAD);

mod token;

pub struct TokenAuth {
    database: PgPool,
    rand: SystemRandom,
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
    // Create a new instance of TokenAuth with database connection string `connection`.
    pub async fn new(connection: &str) -> Result<Self, sqlx::Error> {
        let db = PgPool::connect(connection).await?;
        let rng = SystemRandom::new();

        // Prime the secure random number generator by getting some bytes
        let mut buf: [u8; 100] = [0; 100];
        rng.fill(&mut buf)
            .expect("Secure random generator was unavailable!");

        Ok(Self {
            database: db,
            rand: rng,
        })
    }

    fn new_rand(&self) -> String {
        let mut buf: [u8; 27] = [0; 27];
        self.rand.fill(&mut buf)
            .expect("Secure random generator was unavailable!");

        base64::encode_engine(buf, &URL_SAFE_ENGINE)
    }

    /// returns `data` hashed using SHA256 encoded using base64
    fn hash(&self, data: &[u8]) -> String {
        let hash = digest::digest(&digest::SHA256, data);
        base64::encode_engine(hash.as_ref(), &URL_SAFE_ENGINE)
    }

    /// Verify a sensor token
    pub async fn verify(&self, sensor_token: &BearerToken) -> Result<Token, Error> {
        let hashed_token = self.hash(sensor_token.as_str().as_bytes());
        match Token::by_token(&self.database, hashed_token.as_str()).await? {
            Some(t) => Ok(t),
            None => Err(Error::Verify("Token does not exist".to_string()))
        }
    }

    /// Create a new sensor token and uid pair. Returned token cannot be recovered.
    pub async fn new_token(&self, uid: String) -> Result<(String, String), Error> {
        let sensor_uid = self.new_rand();
        let sensor_token = self.new_rand();
        let hashed_token = self.hash(sensor_token.as_bytes());

        let mut db = self.database.acquire().await?;

        // Store the hashed token value
        let _new = InsertToken {
            token: hashed_token,
            sensor_uid: sensor_uid.clone(),
            firebase_uid: uid,
        }
        .insert(&mut db)
        .await?;

        // And make sure we only respond with the original token,
        // and don't keep a copy
        Ok((sensor_token, sensor_uid))
    }
}