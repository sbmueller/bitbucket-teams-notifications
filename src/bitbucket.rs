use rocket::serde::Deserialize;

/// Top level structure of incoming BitBucket payload.
///
/// Specification: https://confluence.atlassian.com/bitbucketserver/event-payload-938025882.html
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Payload<'r> {
    #[serde(rename = "eventKey")]
    pub event_key: &'r str,
    #[serde(rename = "pullRequest")]
    pub pull_request: PullRequest<'r>,
    pub actor: Actor<'r>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PullRequest<'r> {
    pub id: u64,
    pub title: &'r str,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Actor<'r> {
    #[serde(rename = "displayName")]
    pub display_name: &'r str,
}
