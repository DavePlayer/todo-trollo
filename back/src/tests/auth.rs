use crate::{
    models::user::UserToRegister, repository::sql::establish_connection,
    route_handlers::auth::post::register_new_user,
};

use actix_web::{http::header::ContentType, test, App};
use diesel::{ExpressionMethods, RunQueryDsl};

#[actix_web::test]
async fn register_new_user_no_body() {
    let app = test::init_service(App::new().service(register_new_user)).await;
    let req = test::TestRequest::post()
        .uri("/register")
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{:?}", resp.response().status());
    assert!(resp.status().is_client_error());
}
#[actix_web::test]
async fn register_new_user_success() {
    use crate::schema::users::dsl::*;

    let app = test::init_service(App::new().service(register_new_user)).await;
    let user_data: UserToRegister = UserToRegister {
        name: "testUser1".into(),
        login: "testuser".into(),
        password: "secret".into(),
    };
    let req = test::TestRequest::post()
        .uri("/register")
        .insert_header(ContentType::plaintext())
        .set_json(&user_data)
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{:?}", resp.response().status());

    let mut conn = establish_connection().unwrap();
    if let Ok(status) = diesel::delete(users)
        .filter(name.eq(user_data.name))
        .filter(login.eq(user_data.login))
        .filter(password.eq(user_data.password))
        .execute(&mut conn)
    {
        println!("removed user. status: {status}");
    } else {
        println!("error when removing user");
    }

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn register_new_user_2_times_conflict() {
    use crate::schema::users::dsl::*;

    let app = test::init_service(App::new().service(register_new_user)).await;
    let user_data: UserToRegister = UserToRegister {
        name: "testUser1".into(),
        login: "testuser".into(),
        password: "secret".into(),
    };
    let req1 = test::TestRequest::post()
        .uri("/register")
        .insert_header(ContentType::plaintext())
        .set_json(&user_data)
        .to_request();
    let req2 = test::TestRequest::post()
        .uri("/register")
        .insert_header(ContentType::plaintext())
        .set_json(&user_data)
        .to_request();
    let resp1 = test::call_service(&app, req1).await;
    let resp2 = test::call_service(&app, req2).await;

    let mut conn = establish_connection().unwrap();
    if let Ok(status) = diesel::delete(users)
        .filter(name.eq(user_data.name))
        .filter(login.eq(user_data.login))
        .filter(password.eq(user_data.password))
        .execute(&mut conn)
    {
        println!("removed user. status: {status}");
    } else {
        println!("error when removing user");
    }

    assert!(resp1.status().is_success());
    assert!(resp2.status().is_client_error());
}
