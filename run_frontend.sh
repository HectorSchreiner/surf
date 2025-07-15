#!/usr/bin/env bash

echo "Compiling Frontend..."
cd frontend; 
pnpm install;
pnpm run serve;