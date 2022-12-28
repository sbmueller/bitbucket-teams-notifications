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
    println!("{decoded_url}");
    match client
        .post(decoded_url.as_ref())
        .json(&teams_payload)
        .send()
        .await
    {
        Ok(_) => rocket::http::Status::Ok,
        Err(e) => {
            println!("{e}");
            rocket::http::Status::InternalServerError
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![prupdate])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;

    #[test]
    fn test_conversion_pr_opened() {
        let bitbucket_data = bitbucket::Payload::dummy("pr:opened");
        let teams_data = teams::Payload::from_bitbucket(&bitbucket_data);
        assert_eq!(
            teams_data.attachments[0].content.body.text,
            "John Doe opened PR 123: Refactor."
        );
    }

    #[test]
    fn test_conversion_pr_modified() {
        let bitbucket_data = bitbucket::Payload::dummy("pr:modified");
        let teams_data = teams::Payload::from_bitbucket(&bitbucket_data);
        assert_eq!(
            teams_data.attachments[0].content.body.text,
            "John Doe changed PR 123: Refactor."
        );
    }

    #[test]
    fn test_conversion_pr_approved() {
        let bitbucket_data = bitbucket::Payload::dummy("pr:reviewer:approved");
        let teams_data = teams::Payload::from_bitbucket(&bitbucket_data);
        assert_eq!(
            teams_data.attachments[0].content.body.text,
            "John Doe approved PR 123: Refactor."
        );
    }

    #[test]
    fn test_conversion_pr_needs_work() {
        let bitbucket_data = bitbucket::Payload::dummy("pr:reviewer:needs_work");
        let teams_data = teams::Payload::from_bitbucket(&bitbucket_data);
        assert_eq!(
            teams_data.attachments[0].content.body.text,
            "John Doe requested work on PR 123: Refactor."
        );
    }

    #[test]
    fn test_conversion_pr_merged() {
        let bitbucket_data = bitbucket::Payload::dummy("pr:merged");
        let teams_data = teams::Payload::from_bitbucket(&bitbucket_data);
        assert_eq!(
            teams_data.attachments[0].content.body.text,
            "John Doe merged PR 123: Refactor."
        );
    }

    #[test]
    fn test_request() {
        let target_url = html_escape::encode_safe("https://httpbin.org/post").to_string();
        let rocket = super::rocket();
        let client = Client::tracked(rocket).unwrap();
        let req = client
            .post(rocket::uri!(prupdate(target_url)))
            .header(rocket::http::ContentType::JSON)
            .json(&bitbucket::Payload::dummy("pr:opened"));
        let response = req.dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
    }
}
