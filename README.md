# Rust Fullstack Template :electric_plug: :battery:

**Backend: actix-web**

**Frontend: yew**

## Generate :sparkler: :sparkler: :floppy_disk:
You can use [cargo generate tool](https://crates.io/crates/cargo-generate) to generate this template as your workspace:
```sh
cargo generate --git https://github.com/mishaszu/rust-fullstack-template.git --branch main
```
---
## What :green_book: :mega:
This template is an easy way to start developing production-level applications with `actix-web` & `yew`.
### Actix-web
Features in the template:
- serving static files
- support simple get route
- share API with Frontend
- serve Frontend as SPA

**Port: 8081**

*Note:
In the developer version, the static folder is placed in frontend directory,
and in the production version, the static folder is within the backend folder.
You can use dev setup to run the server from the workspace root folder,
or prod's set up to run it from backend folder.*

#### actix-web examples
[actix-web examples](https://github.com/actix/examples)

### Yew
**Remember to install Frontend packages first!**
```sh
cd frontend && npm install
```

Features in the template:
- handles two simple routes
- passing simple props
- handling simple get request
- webpack configured to support SPA application
- webpack proxy connect to `actix-web` server
- webpack outputs CSS files

**Port: 8000**

*Note:
To get request backend server has to be turned on.*

#### yew examples:
[yew examples](https://github.com/yewstack/yew/tree/v0.17/examples)

[yew router examples](https://github.com/yewstack/yew/tree/v0.17/yew-router/examples)

---
## How :blue_book: :satellite:
You can manually run all commands with `cargo run` or `cargo run -p project_name` for BE and `npm start` etc for FE, or you can use `run.sh` the file that provides all commands.

Remember to install Frontend packages with npm first (you should do this only once).

```sh
cd frontend && npm install
```


Run Frontend in developer mode. The frontend will be served by webpack and proxied to `actix-web` server.
```sh
  ./run.sh fe run
```
Build Frontend with webpack and output files to `frontend/public`
```sh
  ./run.sh fe build:dev
```
Build Frontend with webpack and output file to `backend/static`
```sh
  ./run.sh fe build:prod
```
Build backend
```sh
  ./run.sh be build
```
Run backend with workspace folder as root.
```sh
  ./run.sh be run
```
Run backend with backend folder as root.
```sh
	cd ./backend && cargo run
```
Run & watch backend with workspace folder as root. You will need [cargo-watch](https://crates.io/crates/cargo-watch/3.1.1)
```sh
  ./run.sh be watch
```
Run & watch backend with backend folder as root. You will need [cargo-watch](https://crates.io/crates/cargo-watch/3.1.1)
```sh
  cd backend && cargo watch -x run
```

  
