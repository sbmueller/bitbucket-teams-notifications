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
