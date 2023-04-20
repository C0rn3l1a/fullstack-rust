# Nacho's Full Stack Rust Setup

Example setup of fullstack rust app using axum + leptos.

The sample website is a contact form that sends an email to the configured email in the backend. Designed to be used with gmail using App Password and using Cloudflare Turnstile to prevent bot submission.

# Env Setup
## Requirements
- [Rust](https://www.rust-lang.org/learn/get-started)
- [Trunk](https://trunkrs.dev/) (make sure to add was target with `rustup target add wasm32-unknown-unknown`)
- [Node](https://nodejs.org/en)
- [Tailwind](https://tailwindcss.com/docs/installation)
- [cargo-cmd](https://crates.io/crates/cargo-cmd)
- You may need to install libssl-dev `sudo apt install libssl-dev`

## Environment

The following env variables are required for the backend to run:
```
# backend/.env
SMTP_USERNAME={{ your email }}
SMTP_PASSWORD={{ your app password }}
SMTP_PROVIDER=smtp.gmail.com
RECEIVER_EMAIL={{ your email }}
SENDER_EMAIL={{ your email }}
LOG_LEVEL={{ DEBUG | INFO | WARN | ERROR }}
CLOUDFLARE_SECRET_KEY={{ cf turnstile secret key }}
```

## 3rd Parties

Setup **Cloudflare Turnstile** to prevent bot submissions following [the documentation](https://developers.cloudflare.com/turnstile/).

Setup **A Google App Password** for you gmail account following [this link](https://support.google.com/accounts/answer/185833?hl=en).

# Run the Project

## Backend
```
> cd backend
> cargo run
```
And you should see this
```
Finished dev [unoptimized + debuginfo] target(s) in x.xxs
    Running `target/debug/backend`

╓---- Axum Server Startup ----
║
╟- listening on: 0.0.0.0:3000
╙- log level: {{LOG_LEVEL}}
```

## Frontend
```
> cd frontend
> cargo cmd serve
```
And you should see this
```
[preserve]
> cargo cmd build-css
> NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./static/tailwind.css

Rebuilding...

Done in xxxms.

[serve]
> trunk serve
YYYY-MM-DDTHH:mm:ss.SSSSSSZ  INFO 📦 starting build
...
...
YYYY-MM-DDTHH:mm:ss.SSSSSSZ  INFO ✅ success
YYYY-MM-DDTHH:mm:ss.SSSSSSZ  INFO 📡 serving static assets at -> /
YYYY-MM-DDTHH:mm:ss.SSSSSSZ  INFO 📡 proxying /api -> http://localhost:3000/api
YYYY-MM-DDTHH:mm:ss.SSSSSSZ  INFO 📡 server listening at http://127.0.0.1:8080
```