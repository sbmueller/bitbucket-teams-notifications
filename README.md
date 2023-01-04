# BitBucket Teams Notifications

This is a small web service that can be used as target for [BitBucket server
webhooks][1]. The goal is to transform the BitBucket request in a request that
is accepted by a [Microsoft Teams webhook][2], enabling a bot that updates a
certain channel with updates on pull requests that happen in one repository.

Requires Rust nightly.

[1]: https://confluence.atlassian.com/bitbucketserver/event-payload-938025882.html
[2]: https://learn.microsoft.com/en-us/microsoftteams/platform/webhooks-and-connectors/how-to/connectors-using?tabs=cURL
