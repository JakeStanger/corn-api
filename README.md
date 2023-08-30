# Corn API

Small API for parsing [Corn](https://github.com/jakestanger/corn) as a service.

Send requests on `/parse`. The request body should be fully formed Corn.
Request responses will be the parsed output. 

You can optionally specify the `Accept` header as one of 
`application/json`, `application/yaml` or `application/toml`.
This defaults to JSON.

Configure the host and port with `HOST` and `PORT` environment variables.
These default to `127.0.0.1` and `5050`.

Requests are rate-limited. Control this with the `REQUESTS_PER_MINUTE` env var.
This defaults to `20`.

A Dockerfile is included.