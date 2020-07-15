#!/bin/sh

if [ "$1" = "fe" ]; then
  if [ "$2" = "run" ]; then
    npm start --prefix ./frontend
  elif [ "$2" = "build:dev" ]; then
    npm run build:workspace --prefix ./frontend
  elif [ "$2" = "build:prod" ]; then
    npm run build:backend --prefix ./frontend
  fi
elif [ "$1" = "be" ]; then
  if [ "$2" = "build" ]; then
    cargo build -p backend
  elif [ "$2" = "run" ]; then
    cargo run -p backend workspace
  elif [ "$2" = "watch" ]; then
    cargo watch -x "run -p backend workspace"
  fi
elif [ "$1" = "help" ]; then
  echo "
  All commands:
  fe run -> run frontend with webpack (dev mode)
  fe build:dev -> build frontend in frontend directory for backend run in workspace (dev mode, build with webpack)
  fe build:prod -> build frontend in backend directory (prod mode, build with webpack)
  be build -> build backend (with cargo)
  be run -> run backend (with cargo)
  be watch -> run cargo watch to run backend and observe changes
  "
else
  echo "Command is not known, run help"
fi