use crate::{bitbucket, teams};

pub fn bitbucket_to_teams<'r>(input: &'r bitbucket::Payload) -> teams::Payload<'r> {
    let actor = input.actor.display_name;
    let id = input.pull_request.id;
    let title = input.pull_request.title;
    let message = match input.event_key {
        "pr:opened" => format!("{actor} opened PR {id}: {title}."),
        "pr:modified" => format!("{actor} changed PR {id}: {title}."),
        "pr:reviewer:approved" => format!("{actor} approved PR {id}: {title}."),
        "pr:reviewer:needs_work" => {
            format!("{actor} requested work on PR {id}: {title}.")
        }
        "pr:merged" => format!("{actor} merged PR {id}: {title}."),
        _ => "Unknown event_key".to_string(),
    };
    teams::Payload::new(message, input.pull_request.links[0].href)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_pr_opened() {
        let bitbucket_data = bitbucket::Payload::dummy("pr:opened");
        let teams_data = bitbucket_to_teams(&bitbucket_data);
        assert_eq!(
            teams_data.attachments[0].content.body.text,
            "John Doe opened PR 123: Refactor."
        );
    }

    #[test]
    fn test_conversion_pr_modified() {
        let bitbucket_data = bitbucket::Payload::dummy("pr:modified");
        let teams_data = bitbucket_to_teams(&bitbucket_data);
        assert_eq!(
            teams_data.attachments[0].content.body.text,
            "John Doe changed PR 123: Refactor."
        );
    }

    #[test]
    fn test_conversion_pr_approved() {
        let bitbucket_data = bitbucket::Payload::dummy("pr:reviewer:approved");
        let teams_data = bitbucket_to_teams(&bitbucket_data);
        assert_eq!(
            teams_data.attachments[0].content.body.text,
            "John Doe approved PR 123: Refactor."
        );
    }

    #[test]
    fn test_conversion_pr_needs_work() {
        let bitbucket_data = bitbucket::Payload::dummy("pr:reviewer:needs_work");
        let teams_data = bitbucket_to_teams(&bitbucket_data);
        assert_eq!(
            teams_data.attachments[0].content.body.text,
            "John Doe requested work on PR 123: Refactor."
        );
    }

    #[test]
    fn test_conversion_pr_merged() {
        let bitbucket_data = bitbucket::Payload::dummy("pr:merged");
        let teams_data = bitbucket_to_teams(&bitbucket_data);
        assert_eq!(
            teams_data.attachments[0].content.body.text,
            "John Doe merged PR 123: Refactor."
        );
    }

    #[test]
    fn test_pr_link() {
        let bitbucket_data = bitbucket::Payload::dummy("pr:merged");
        let teams_data = bitbucket_to_teams(&bitbucket_data);
        assert_eq!(
            teams_data.attachments[0].content_url,
            Some("http://test.url/")
        );
    }
}
