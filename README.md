Refwave is a referral code generation system that allows users to create an account and invite their friends to join via a referral code. It's not really useful for anything besides showcasing how one can architect a Rust backend and communicate with it from a Javascript environment.

The Backend service is built using Rust's [Axum framework](https://github.com/tokio-rs/axum), a PostgreSQL database, and [sqlx](https://docs.rs/sqlx/latest/sqlx/) as the database driver. I'm also using some Tower's [ServiceBuilder](https://docs.rs/tower/latest/tower/struct.ServiceBuilder.html) for middleware. The frontend components are built in [Solidjs](https://www.solidjs.com/) and styled with [Tailwindcss](https://tailwindcss.com/).

## Key Features

**Server-Sent Events(SSE) for Real-time Updates**: [Server sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) allow the server to push updates to the client in realtime without the need for a client to initiate a request. They're similar to websockets but are unidirectional; that is, data messages are delivered in one direction, from the server to the client. This makes them an excellent choice when we only need to send data from the server to the client in message form. The client subscribes to updates via the [EventSource](https://developer.mozilla.org/en-US/docs/Web/API/EventSource) API by opening a persistent connection to the server, which in turn sends events in the form of `text/event-stream`. 

**Async communication**: We employ the [tokio runtime](https://tokio.rs/) for async rust, allowing our system to handle requests and operations in a non-blocking manner. This boosts the perfomance of our application by allowing tasks to run without waiting for others to complete.

**Channels for Concurrent Streaming of Events**: We employ the [broadcast::channel](https://docs.rs/tokio/latest/tokio/sync/broadcast/) from tokio which allows multiple receivers. This means that multiple clients can subscribe and listen for updates concurrently. Serialized data is sent over one side of the channel, and then emmited over the other end as a server-sent event.

**Reactive UI**: The UI is built to take advantage of [SolidJS's reactive system](https://www.solidjs.com/guides/reactivity), allowing updates at the finest level. When data changes, only the parts of the UI that depend on that particular piece of data get re-rendered. This ensures efficient updates without unnecessary rendering, leading to a snappy UX.


## Requirements

Here's what you need to run the code locally.


### Rust

You need to have Rust installed on your system. If you haven't installed it yet, follow the official [installation guide](https://www.rust-lang.org/tools/install).

### Postgres

You need access to a PostgreSQL database. If you don't have it already; You can download from [here](https://www.postgresql.org/download/) or you can launch a PostgreSQL server in a Docker container.  At the end of the day, you just need a database URL, which you can pass to your web service as an environment variable.

### sqlx-cli

We'll leverage the sqlx cli to handle database migrations. Install it by running:

```bash
 cargo install sqlx-cli 
```

### NodeJS

You'll need a recent(18+) version of nodejs installed on your machine. [Download here](https://nodejs.org/en/download).


## Running the Application

To run the application, clone the repo:
```bash
git clone https://github.com/kelvinkirima014/refwave.git
```
change into the backend directory to start the server:
```bash
cd refwave/backend
```
Update the DATABASE_URL in the `.env.sample` with your database url. Also remember to change the name of the file to just `.env`.

Then run the database migrations using sqlx-cli:
```bash
sqlx migrate run
```
And finally start the backend service by running:
```bash
cargo run
```

Now you can interact with the application by starting the frontend:

```bash
cd refwave/frontend

npm install

npm run dev --open

```