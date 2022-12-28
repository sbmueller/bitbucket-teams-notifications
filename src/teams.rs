//! https://learn.microsoft.com/en-us/microsoftteams/platform/webhooks-and-connectors/how-to/connectors-using

use crate::bitbucket;
use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Payload<'r> {
    r#type: &'r str,
    attachments: Vec<Card<'r>>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Card<'r> {
    #[serde(rename = "contentType")]
    content_type: &'r str,
    #[serde(rename = "contentUrl")]
    content_url: Option<&'r str>,
    content: Content<'r>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Content<'r> {
    #[serde(rename = "$schema")]
    schema: &'r str,
    r#type: &'r str,
    version: &'r str,
    body: Body<'r>,
}

#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
struct Body<'r> {
    r#type: &'r str,
    text: String,
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
        // TODO: Write better message
        let message = format!("{} opened new PR.", bitbucket.actor.display_name);
        Payload::new(message)
    }
}
