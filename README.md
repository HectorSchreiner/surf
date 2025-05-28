<p align="center">
  <img src="assets/logo_white.png" alt="Logo" style="width:400px; display:block; margin:auto;">
</p>

<p align="center">
  <a href="/LICENSE">
    <img src="https://cdn.prod.website-files.com/5e0f1144930a8bc8aace526c/65dd9eb5aaca434fac4f1c34_License-MIT-blue.svg" alt="License: MIT">
  </a>
    <img src="https://img.shields.io/badge/status-active-brightgreen.svg" alt="Project Status: Active">
    <img src="https://img.shields.io/github/languages/top/HectorSchreiner/surf.svg" alt="Top Language">
    <img src="https://img.shields.io/github/contributors/HectorSchreiner/surf.svg" alt="Contributors">
</p>

<h2 align="center">A blazingly fast and safe attack surface management application.</h2>

Surf is a security tool, that allows you to scan your internal surface for the latest vulnerabilities, and give you updates whenever one is found. 

* [Getting Started](#getting-started)
* [For Development](#for-development)
* [Features](#features)
* [License](#license)

## Getting Started


## For Development
Make sure you have installed `Docker, Rust & npm` on your machine

1. Start Postgres using Docker Compose:
2. Start the backend (on port 4000):
3. Start the frontend (on port 3000):

Requests to `localhost:3000/api` are proxied to `localhost:4000/api`

**Linux**
```bash
docker compose -f compose.dev.yaml up -d
cd backend && cargo watch -x run --features=docs
cd frontend && pnpm start
```

**Windows**
```powershell
docker compose -f compose.dev.yaml up -d
cd backend; cargo watch -x run --features=docs;
cd frontend; pnpm run dev
```

## Features

## License
This project is licensed under the MIT [LICENSE](./LICENSE).
