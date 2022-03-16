# Deployment

## Environment Variables

- `ROCKET_SITE_HOST`
- `ROCKET_AUTHTOKEN_COOKIE_NAME`
- `ROCKET_AUTHTOKEN_COOKIE_DOMAIN`
- `ROCKET_JWT_PUBLIC_KEY`
- `ROCKET_JWT_PRIVATE_KEY`
- `ROCKET_ALLOWED_NEXT_HOSTS`

See the `scripts/run_dev_server.sh` for more details.

## Proxied Headers

`flux-web-auth` is designed to only run behind a reverse proxy and these headers are expected to run properly:

- `X-Scheme`
