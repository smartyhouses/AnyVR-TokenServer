use livekit_api::access_token;
use serde::{Deserialize, Serialize};
use std::env;
use warp::Filter;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env").ok();

    let create_token_route = warp::path("requestToken")
        .and(warp::query::<QueryParams>())
        .map(|params: QueryParams| {
            let token = create_token(&params.room_name, &params.user_name).unwrap();
            warp::reply::json(&TokenResponse { token })
        });

    warp::serve(create_token_route)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

fn create_token(
    room_name: &String,
    user_name: &String,
) -> Result<String, access_token::AccessTokenError> {
    let api_key = env::var("LIVEKIT_API_KEY").expect("LIVEKIT_API_KEY is not set");
    let api_secret = env::var("LIVEKIT_API_SECRET").expect("LIVEKIT_API_SECRET is not set");

    let id = generate_unique_id();
    let token = access_token::AccessToken::with_api_key(&api_key, &api_secret)
        .with_identity(&id)
        .with_name(&user_name)
        .with_grants(access_token::VideoGrants {
            room_join: true,
            room: room_name.to_string(),
            ..Default::default()
        })
        .to_jwt();

    println!("Token created: name = {}, id = {}, room = {}", &user_name, id, &room_name);
    return token;
}

fn generate_unique_id() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    return rand_string;
}

#[derive(Debug, Deserialize)]
struct QueryParams {
    room_name: String,
    user_name: String,
}

// Response structure
#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}
