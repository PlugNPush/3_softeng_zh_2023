_default:
    @just --list

# run the server, watching for changes
watch:
    @killall server &> /dev/null || true
    cd server && cargo watch -x run

# run the server
run *args:
    @cd server && cargo run -q -- {{ args }}

# run the web app dev server, watching for changes
app-watch:
    @killall trunk &> /dev/null || true
    cd app && trunk serve --open

# render the documentation book, watching for changes
book-watch:
    @killall mdbook &> /dev/null || true
    cd docs && mdbook serve --port 5000

# render d2 diagrams, watching for changes
diagrams-watch:
    watchexec --debounce 1000 \
        --emit-events-to file \
        --watch docs/diagrams \
        --restart ./dev/render_diagrams.sh

# start a terminal workspace for development
zellij:
    zellij --layout dev/zellij.kdl
    @killall server &> /dev/null || true
    @killall trunk &> /dev/null || true
    @killall mdbook &> /dev/null || true

podman-build:
    podman build -t softw-eng .

podman-run:
    podman run -it --rm -p 4000:4000 softw-eng

dive:
    # dive is a great container image inspection tool
    dive podman://softw-eng

# app must be running for this to work
playwright-test:
    (cd e2e && npx playwright test)
