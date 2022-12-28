//! https://learn.microsoft.com/en-us/microsoftteams/platform/webhooks-and-connectors/how-to/connectors-using

use crate::bitbucket;
use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Payload<'r> {
    r#type: &'r str,
    pub attachments: Vec<Card<'r>>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Card<'r> {
    #[serde(rename = "contentType")]
    content_type: &'r str,
    #[serde(rename = "contentUrl")]
    content_url: Option<&'r str>,
    pub content: Content<'r>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Content<'r> {
    #[serde(rename = "$schema")]
    schema: &'r str,
    r#type: &'r str,
    version: &'r str,
    pub body: Body<'r>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Body<'r> {
    r#type: &'r str,
    pub text: String,
}

impl<'r> Payload<'r> {
    pub fn new(message: String) -> Self {
        Payload {
            r#type: "message",
            attachments: vec![Card {
                content_type: "application/vnd.microsoft.card.adaptive",
                content_url: None,
                content: Content {
                    schema: "http://adaptivecards.io/schemas/adaptive-card.json",
                    r#type: "AdaptiveCard",
                    version: "1.2",
                    body: Body {
                        r#type: "TextBlock",
                        text: message,
                    },
                },
            }],
        }
    }

    pub fn from_bitbucket(bitbucket: &'r bitbucket::Payload) -> Self {
        let actor = bitbucket.actor.display_name;
        let id = bitbucket.pull_request.id;
        let title = bitbucket.pull_request.title;
        let message = match bitbucket.event_key {
            "pr:opened" => format!("{actor} opened PR {id}: {title}."),
            "pr:modified" => format!("{actor} changed PR {id}: {title}."),
            "pr:reviewer:approved" => format!("{actor} approved PR {id}: {title}."),
            "pr:reviewer:needs_work" => {
                format!("{actor} requested work on PR {id}: {title}.")
            }
            "pr:merged" => format!("{actor} merged PR {id}: {title}."),
            _ => "Unknown event_key".to_string(),
        };
        Payload::new(message)
    }
}
