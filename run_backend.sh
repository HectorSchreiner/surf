<<<<<<< HEAD
#!/usr/bin/env bash

echo "Compiling Backend...";
cd backend;
cargo watch -x run --features=docs;
=======
#!/bin/bash

echo "Compiling Backend..."
cd backend; cargo watch -x run --features=docs
>>>>>>> main
