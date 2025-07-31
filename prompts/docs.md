Update documents to match the existing API implementation.
You can overwrite the existing documentation.

1. Read the implementation of `crates/homunculus_http_server/src/**` to understand the HTTP endpoints.
2. Read the implementation of `sdk/typescript/src/**` to understand the SDK interfaces.
3. Update the `docs/api/open-api.yml` file with the correct API documentation.
4. Update the `docs/mod-manual/src/sdk/**` files to match the SDK interfaces.
5. Run `make build-openapi` to regenerate the documentation.

## Constraints

- The documentation must be in English.
- The documentation must match the existing HTTP API implementation.
- The documentation must be clear and concise.
- Tag APIs appropriately.
