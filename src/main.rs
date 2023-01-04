#[macro_use]
extern crate rocket;
extern crate html_escape;
extern crate reqwest;
use rocket::serde::json::Json;

mod bitbucket;
mod mapper;
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
    let teams_payload = mapper::bitbucket_to_teams(&bitbucket_payload);
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
    fn test_request_with_dummy() {
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

    #[test]
    fn test_request_with_doc_example() {
        // Example from bitbucket docs: https://confluence.atlassian.com/bitbucketserver/event-payload-938025882.html
        let payload = r#"{
  "eventKey":"pr:opened",
  "date":"2017-09-19T09:58:11+1000",
  "actor":{
    "name":"admin",
    "emailAddress":"admin@example.com",
    "id":1,
    "displayName":"Administrator",
    "active":true,
    "slug":"admin",
    "type":"NORMAL"
  },
  "pullRequest":{
    "id":1,
    "version":0,
    "title":"a new file added",
    "state":"OPEN",
    "open":true,
    "closed":false,
    "createdDate":1505779091796,
    "updatedDate":1505779091796,
    "fromRef":{
      "id":"refs/heads/a-branch",
      "displayId":"a-branch",
      "latestCommit":"ef8755f06ee4b28c96a847a95cb8ec8ed6ddd1ca",
      "repository":{
        "slug":"repository",
        "id":84,
        "name":"repository",
        "scmId":"git",
        "state":"AVAILABLE",
        "statusMessage":"Available",
        "forkable":true,
        "project":{
          "key":"PROJ",
          "id":84,
          "name":"project",
          "public":false,
          "type":"NORMAL"
        },
        "public":false
      }
    },
    "toRef":{
      "id":"refs/heads/master",
      "displayId":"master",
      "latestCommit":"178864a7d521b6f5e720b386b2c2b0ef8563e0dc",
      "repository":{
        "slug":"repository",
        "id":84,
        "name":"repository",
        "scmId":"git",
        "state":"AVAILABLE",
        "statusMessage":"Available",
        "forkable":true,
        "project":{
          "key":"PROJ",
          "id":84,
          "name":"project",
          "public":false,
          "type":"NORMAL"
        },
        "public":false
      }
    },
    "locked":false,
    "author":{
      "user":{
        "name":"admin",
        "emailAddress":"admin@example.com",
        "id":1,
        "displayName":"Administrator",
        "active":true,
        "slug":"admin",
        "type":"NORMAL"
      },
      "role":"AUTHOR",
      "approved":false,
      "status":"UNAPPROVED"
    },
    "reviewers":[

    ],
    "participants":[

    ],
    "links":{
      "self":[
        null
      ]
    }
  }
}"#;
        let target_url = html_escape::encode_safe("https://httpbin.org/post").to_string();
        let rocket = super::rocket();
        let client = Client::tracked(rocket).unwrap();
        let req = client
            .post(rocket::uri!(prupdate(target_url)))
            .header(rocket::http::ContentType::JSON)
            .body(payload);
        let response = req.dispatch();
        assert_eq!(response.status(), rocket::http::Status::Ok);
    }
}
