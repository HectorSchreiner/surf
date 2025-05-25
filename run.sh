#!/bin/bash

SESSION="surf"

case "$OSTYPE" in
  linux*)   echo "Linux";;
  cygwin*)  echo "Windows (Cygwin)";;
  msys*)    echo "Windows (MSYS or Git Bash)";;
  win32*)   echo "Windows";;
  *)        echo "Unknown OS: $OSTYPE";;
esac

# Install tmux
if [ "$OS" = "linux" ]; then
  if ! command -v tmux >/dev/null 2>&1; then
    echo "tmux not found. Installing..."
    sudo apt-get install -y tmux
  fi

  if ! command -v node >/dev/null 2>&1; then
    echo "node not found. Installing..."
    sudo apt-get install -y node
  fi

  if ! command -v docker >/dev/null 2>&1; then
    echo "docker not found. Installing..."
    sudo apt-get install -y node
  fi
fi

if ! command -v tmux >/dev/null 2>&1; then
  echo "Error: tmux is not installed or not in PATH."
  echo "Please install tmux manually."
  exit 1
fi
# hvis above ikke lige virker
if ! command -v node >/dev/null 2>&1; then
  echo "Error: node is not installed or not in PATH."
  echo "Please install node manually."
  exit 1
fi
# hvis above ikke lige virker
if ! command -v docker >/dev/null 2>&1; then
  echo "Error: docker is not installed or not in PATH."
  echo "Please install docker manually."
  exit 1
fi

# Start tmux session
tmux new-session -d -s $SESSION
tmux send-keys -t $SESSION "doclker compose -f compose.dev.yaml up -d" C-m

tmux split-window -h -t $SESSION
tmux send-keys -t $SESSION:0.1 "cd backend && cargo watch -x run --features=docs" C-m

tmux split-window -v -t $SESSION:0.1
tmux send-keys -t $SESSION:0.2 "cd frontend && pnpm run dev" C-m

tmux select-layout -t $SESSION tiled
tmux select-pane -t $SESSION:0.0
tmux attach -t $SESSION