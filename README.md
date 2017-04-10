# Webfinger
[![Build Status](https://travis-ci.org/NullHubProject/webfinger-rs.svg)](https://travis-ci.org/NullHubProject/webfinger-rs)
[![Coverage Status](https://coveralls.io/repos/NullHubProject/webfinger-rs/badge.svg?branch=master&service=github)](https://coveralls.io/github/NullHubProject/webfinger-rs?branch=master)

## Try it out
`cargo run <identifier> [<host>]`

Identifier is a resource URI. If no scheme is given, `acct:` is assumed.

Host is optional; if not given, it will be parsed from the identifier.
