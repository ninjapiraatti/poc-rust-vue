# Hula

This POC is made for an open source project which I will link here once it is launched. Idea is to have a web server that can manipulate a Postgres database and well, do whatever web servers do. Actix was chosen for the server and Vue for the frontend.

The web server part is essentially a direct clone of [Harry Gill's](https://gill.net.in) very good tutorial. It can be found [here.](https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/) At the time of writing (2021), Rust is a relatively young language so bear in mind that newer and better techniques might be around.

### Prerequisites

- [Diesel_cli](http://diesel.rs/guides/getting-started/).
- Postgres
- Rust
- Node.js
- Email service API key, I used sparkpost account for this

### Deployment

1. Create a postgres database
2. Clone this repo
3. Create .env file to the root:
```
DATABASE_URL=postgres://user:password@server/dbname
SPARKPOST_API_KEY='xxxxx'
SENDING_EMAIL_ADDRESS='your.sendingaddress@yourdomain.com'
SERVER_URL='localhost:8086'
PUBLIC_URL='http://localhost:8086'
RUST_LOG='trace'
USER_UPLOAD_PATH='/'
PROJECT_EXPIRY_SECS=2592000
```
4. From the root, `mkdir public`
5. Go to App folder and run `npm install`
6. Because of reasons, Flashmessages won't install as expected. Run `npm i @smartweb/vue-flash-message@1.0.0-alpha.12`
7. Bootstrap needs Popper but doesn't include it. Still in app folder, run `npm i @popperjs/core`
8. Still in the app folder, run `npm run dev`
9. From the root, setup diesel: `diesel migration run` and then `diesel setup`
10. From the root, run `cargo run`
11. App SHOULD be up at localhost:8086.
