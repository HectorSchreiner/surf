<p align="center">
  <img src="assets/logo_white.png" alt="Logo" style="width:100%; max-width:800px; display:block; margin:auto;">
</p>

<p align="center">
  <a href="/LICENSE">
    <img src="https://cdn.prod.website-files.com/5e0f1144930a8bc8aace526c/65dd9eb5aaca434fac4f1c34_License-MIT-blue.svg" alt="License: MIT">
  </a>
    <img src="https://img.shields.io/badge/status-active-brightgreen.svg" alt="Project Status: Active">
    <img src="https://img.shields.io/github/languages/top/HectorSchreiner/surf.svg" alt="Top Language">
    <img src="https://img.shields.io/github/contributors/HectorSchreiner/surf.svg" alt="Contributors">
</p>

---

# 🚀 About

**Surf** is a blazingly fast and safe **attack surface management application**.

---

# Getting Started


## For Development

1. Start Postgres using Docker Compose:

```bash
docker compose -f compose.dev.yaml up -d
```

2. Start the backend (on port 4000):
   
```bash
cd backend && cargo watch -x run --features=docs
```

3. Start the frontend (on port 3000):

```bash
cd frontend && pnpm start
```

Requests to `localhost:3000/api` are proxied to `localhost:4000/api`

```powershell
docker compose -f compose.dev.yaml up -d
cd backend; cargo watch -x run --features=docs
cd frontend; pnpm run dev
```

# 🔥 Features

- **CVE Management Tool**  
    CVE management tool: For fast indexing an monitoring of the latest security vulnerabilities.



# 📄 License

This project is licensed under the MIT [LICENSE](./LICENSE).
