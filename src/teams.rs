//! https://learn.microsoft.com/en-us/microsoftteams/platform/webhooks-and-connectors/how-to/connectors-using

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
    pub content_url: Option<String>,
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
    pub fn new(message: String, url: Option<String>) -> Self {
        Payload {
            r#type: "message",
            attachments: vec![Card {
                content_type: "application/vnd.microsoft.card.adaptive",
                content_url: url,
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
}
