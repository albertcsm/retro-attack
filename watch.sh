#!/bin/bash
(trap 'kill 0' SIGINT; \
 cargo watch -i .gitignore -i 'pkg/*' -s 'wasm-pack build --target web' & \
 live-server --port=8000 --no-browser)
