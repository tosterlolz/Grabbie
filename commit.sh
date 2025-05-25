#!/usr/bin/env bash

if [ -z "$1" ]; then
  echo "Usage: $0 'commit msg'"
  exit 1
fi

git add .
git commit -m "$1"
git push origin HEAD