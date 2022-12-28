use rocket::serde::{Deserialize, Serialize};

/// Top level structure of incoming BitBucket payload.
///
/// Specification: https://confluence.atlassian.com/bitbucketserver/event-payload-938025882.html
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Payload<'r> {
    #[serde(rename = "eventKey")]
    pub event_key: &'r str,
    #[serde(rename = "pullRequest")]
    pub pull_request: PullRequest<'r>,
    pub actor: Actor<'r>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PullRequest<'r> {
    pub id: u64,
    pub title: &'r str,
    pub links: Vec<Href<'r>>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Actor<'r> {
    #[serde(rename = "displayName")]
    pub display_name: &'r str,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Href<'r> {
    pub href: &'r str,
}

impl<'r> Payload<'r> {
    #[allow(dead_code)]
    pub fn dummy(event_key: &'r str) -> Payload<'r> {
        Payload {
            event_key,
            pull_request: PullRequest {
                id: 123,
                title: "Refactor",
                links: vec![Href {
                    href: "http://test.url/",
                }],
            },
            actor: Actor {
                display_name: "John Doe",
            },
        }
    }
}
