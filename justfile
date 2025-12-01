run:
  @just build-styles
  cargo run

watch:
  cargo watch -s 'just run'

check:
  cargo fmt --all --check
  cargo clippy -- -D warnings

test:
  cargo test

format:
  cargo fmt --all

build-styles:
  sass --no-source-map --style=compressed -q styles/main.scss static/styles.css

deploy:
  @just clean
  @just build-styles
  @just test
  nix build
  STORE_PATH="$(readlink -f result)"
  nix copy --to ssh://seber "$STORE_PATH"
  ssh seber "rm -f /var/website/website && ln -sf '$STORE_PATH' /var/website/website"
  rsync -avz static/ seber:/var/website/static
  rsync -avz img/ seber:/var/website/img
  rsync -avz .env seber:/var/website/.env

clean:
  rm -f static/main.css
