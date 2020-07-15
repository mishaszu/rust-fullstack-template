# Rust Fullstack Template
**Backend: actix-web**
**Frontend: yew**
## Generate
You can use [cargo generate tool](https://github.com/ashleygwilliams/cargo-generate) to generate this template as your project:
```sh
cargo generate --git https://github.com/mishaszu/rust-fullstack-template.git --branch main
```
## What
This template is an easy way to start developing production levle application with `actix-web` & `yew`.
### Actix-web
Features in template:
- serving static files
- support simple get route
- share api with frontend

**Port: 8081**

Note:
In developer version static folder is placed in frontend directory,
and in production version static folder is within backend folder.
You can use dev setup to run server from workspace root folder,
or prod's setup to run it from backend folder.

### Yew
Features in template:
- handles 2 simple routes
- passing simple props
- handling simple get request
- webpack configured to support SPA application
- webpack proxy connect to `actix-web` server
- webpack outputs css files

**Port: 8000**

Note:
In order to do get request backend server has to be turned on.

## How