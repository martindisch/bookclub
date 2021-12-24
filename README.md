# bookclub

A book proposal and voting system for book clubs. It's brutally simple and does
the absolute minimum to allow adding books with some additional information and
collecting votes on them in as few clicks as possible. No signup or accounts,
no hassle.

## Run with Docker

`docker compose up` will spin up a MongoDB instance, the API on localhost:8080
and the client on localhost:3000. Visit the client and you're good to go.

## Run for local development

1. Start a MongoDB instance, for example with `docker run mongo:5`.
2. Create `bookclub-api/.env` based on `bookclub-api/.env.TEMPLATE` and set the
   variable to the IP address of your MongoDB instance.
3. Start the API with `cd bookclub-client && cargo run`.
4. Start the client with `cd bookclub-client && npm install && npm run dev`.

## License

[MIT license](LICENSE)
