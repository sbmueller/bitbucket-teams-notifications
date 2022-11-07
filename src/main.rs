#[macro_use]
extern crate rocket;
extern crate html_escape;
extern crate reqwest;
use rocket::serde::json::Json;

mod bitbucket;
mod teams;

/// Event handler for incoming BitBucket webhooks.
#[post(
    "/prupdate/<teams_url>",
    format = "application/json",
    data = "<payload>"
)]
async fn prupdate(teams_url: &str, payload: Json<bitbucket::Payload<'_>>) -> rocket::http::Status {
    // Map payload to teams structure
    let bitbucket_payload = payload.into_inner();
    let teams_payload = teams::Payload::from_bitbucket(&bitbucket_payload);
    // Make request to teams url
    let client = reqwest::Client::new();
    let decoded_url = html_escape::decode_html_entities(teams_url);
    match client
        .post(decoded_url.as_ref())
        .json(&teams_payload)
        .send()
        .await
    {
        Ok(_) => rocket::http::Status::Ok,
        Err(_) => rocket::http::Status::InternalServerError,
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![prupdate])
}
