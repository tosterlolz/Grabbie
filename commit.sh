#!/usr/bin/env bash

if [ -z "$1" ]; then
  echo "Usage: $0 'commit msg'"
  exit 1
fi

makepkg --printsrcinfo > .SRCINFO
git add .
git commit -m "$1"
git push origin HEAD

if git remote | grep -q "^aur$"; then
  current_branch=$(git rev-parse --abbrev-ref HEAD)
  git push aur "$current_branch":master
else
  echo "Remote 'aur' isn't configured."
fi