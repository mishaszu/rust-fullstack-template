#!/bin/sh

if [ "$1" = "fe" ]; then
  if [ "$2" = "run" ]; then
    npm start --prefix ./frontend
  elif [ "$2" = "build" ]; then
    npm run build:workspace --prefix ./frontend
  fi
elif [ "$1" = "be" ]; then
  if [ "$2" = "build" ]; then
    cargo build -p backend
  elif [ "$2" = "run" ]; then
    cargo run -p backend workspace
  elif [ "$2" = "watch" ]; then
    cargo watch -x "run -p backend workspace"
  fi
fi