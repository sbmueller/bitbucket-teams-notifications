# BitBucket Teams Notifications

This is a small web service that can be used as target for [BitBucket server
webhooks][1]. The goal is to transform the BitBucket request in a request that
is accepted by a [Microsoft Teams webhook][2], enabling a bot that updates a
certain channel with updates on pull requests that happen in one repository.

Requires Rust nightly.

## Webhook Format

Since BitBucket decodes URLs entered as webhook, a custom escaping is required
to distinguish between URL intended for the webservice and URL that belongs to
a Teams channel incoming webhook. For that, `+` is used in the Teams incoming
webhook URL as replacement character for a slash `/`:

```
https://url.to.this.service/prupdate/https:++teams.url+some-identifier
```

Note the URL up to `prupdate/` is intended to be processed by this service
while everything afterwards is the outgoing Teams URL, where `/` is replaced by
`+`.

[1]: https://confluence.atlassian.com/bitbucketserver/event-payload-938025882.html
[2]: https://learn.microsoft.com/en-us/microsoftteams/platform/webhooks-and-connectors/how-to/connectors-using?tabs=cURL
