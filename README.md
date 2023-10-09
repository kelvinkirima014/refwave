Refwave is a high-perfomant Referral System; Users can create an account, and invite their friends to join via their referral code.

The Backend service is built using Rust's [Axum framework](https://github.com/tokio-rs/axum), a PostgreSQL database, and sqlx as the database driver. I'm also using some Tower components for middleware. The frontend frontend components are built in Solidjs and styled with Tailwindcss.

Key Features

Async communication: We employ the tokio runtime for async rust, allowing our system to handle requests and operations in a non-blocking manner. This boosts the perfomance of our application by allowing tasks to run without waiting for others to complete.

Server-Sent Events(SSE) for Real-time Updates: Server sent events allow the server to push updates to the client in realtime without the need for a client to initiate a request. They're similar to websockets but don't require bidirectional communcation and run on plain http.

Channels for Concurrent Streaming of Events: We employ the broadcast::channel from tokio which allows multiple receivers. This means that multiple clients can subscribe and listen for updates concurrently. Serialized data is sent over one side of the channel, and then emmited over the other end as a server-sent event.

Reactive UI: The UI is built to take advantage of SolidJS's reactive system, allowing updates at the finest level. When data changes, only the parts of the UI that depend on that particular piece of data get re-rendered. This ensures efficient updates without unnecessary rendering, leading to a snappy UX.


Requirements

Postgres

You need access to a PostgreSQL database. If you don't have it already; You can download from here or you can launch a PostgreSQL server in a Docker container.  At the end of the day, you just need a database URL, which you can pass to your web service as an environment variable.

Rust installed

Install Rust

NodeJS
You'll need a recent version of nodejs installed on your machine


Running the Application

git clone refwave
cd refwave/backend
 change .env.sample to .env and update it with your database URL

cargo run

Run the client

cd refwave/frontend

npm install

npm run dev --open
