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
    teams::Payload::new(message)
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
    fn test_conversion_doc_example() {
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
        let bitbucket_data = rocket::serde::json::from_str(payload).unwrap();
        let teams_data = bitbucket_to_teams(&bitbucket_data);
        assert_eq!(
            teams_data.attachments[0].content.body.text,
            "Administrator opened PR 1: a new file added."
        );
    }

    // #[test]
    // fn test_pr_link() {
    //     let bitbucket_data = bitbucket::Payload::dummy("pr:merged");
    //     let teams_data = bitbucket_to_teams(&bitbucket_data);
    //     assert_eq!(
    //         teams_data.attachments[0].content_url,
    //         Some("http://test.url/")
    //     );
    // }
}
