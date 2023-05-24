use actix_web::{dev::ServiceRequest, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

use crate::{errors::AuthErrors, models::user::UserClaims};

// this is copied from the internets
// it checks if your token is valid
// can't convert my error type to standard error for this case, so i left it like this
// i know it sucks but middlewares on actix web are overly complicated
// and i don't know i want to paly with ot
pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::error::Error, ServiceRequest)> {
    println!("{:?}", std::env::vars());
    let secret = match std::env::var("JWT_SECRET") {
        Ok(s) => s,
        Err(err) => {
            // if env is not set the entire server is not going to work, so it should panic
            let sth: actix_web::error::Error =
                AuthErrors::EnvError(err.to_string() + " JWT_SECRET").into();
            return Err((sth, req));
        }
    };
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();

    let token_string = credentials.token();

    let claims: UserClaims = match token_string.verify_with_key(&key) {
        Ok(o) => o,
        Err(err) => {
            log::info!("error when decoding token: {:?}", err);
            // this scratches my bananna
            //     let config = req
            //         .app_data::<actix_web_httpauth::extractors::bearer::Config>()
            //         .cloned()
            //         .unwrap_or_default()
            //         .scope("");
            let sth: actix_web::error::Error = AuthErrors::InvalidToken(err.to_string()).into();
            return Err((sth, req));
        }
    };

    req.extensions_mut().insert(claims);
    Ok(req)
}
