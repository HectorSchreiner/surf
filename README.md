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

## Tech Stack
| Language     | Files | Lines | Code | Comments | Blanks |
|--------------|-------|-------|------|----------|--------|
| CSS          | 2     | 2     | 2    | 0        | 0      |
| HTML         | 2     | 33    | 31   | 0        | 2      |
| JavaScript   | 1     | 1     | 1    | 0        | 0      |
| JSON         | 2     | 44    | 44   | 0        | 0      |
| Nix          | 1     | 31    | 17   | 7        | 7      |
| Shell        | 1     | 61    | 46   | 5        | 10     |
| SQL          | 3     | 27    | 25   | 0        | 2      |
| TOML         | 4     | 66    | 60   | 0        | 6      |
| TSX          | 3     | 63    | 47   | 1        | 15     |
| TypeScript   | 2     | 20    | 19   | 0        | 1      |
| YAML         | 2     | 1950  | 1530 | 0        | 420    |
| Markdown     | 1     | 46    | 0    | 33       | 13     |
| └─ BASH      | 1     | 3     | 3    | 0        | 0      |
| └─ PowerShell| 1     | 3     | 3    | 0        | 0      |
| (Subtotal)   |       | 52    | 6    | 33       | 13     |
| Rust         | 16    | 1020  | 836  | 13       | 171    |
| └─ Markdown  | 4     | 15    | 0    | 14       | 1      |
| (Subtotal)   |       | 1035  | 836  | 27       | 172    |
| **Total**    | 40    | 3364  | 2658 | 59       | 647    |


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
./run_backend 
./run_frontend; 
```

**Nix Specific**
Install the dependencies using the provided flake.nix. (This installs: postman nodejs, docker, cargo-watch, rustc & cargo)
```bash
nix develop
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
