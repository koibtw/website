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

build-data:
  cp --no-preserve=mode,ownership -r $(nix build --no-link --print-out-paths github:nixdle/nixdle#data) data

deploy:
  @just clean
  @just build-styles
  @just build-data
  @just test
  nix build
  nix copy --to ssh://seber "$(readlink -f result)"
  ssh seber "rm -f /var/website/website && ln -sf '$(readlink -f result)' /var/website/website"
  rsync -avz static/ seber:/var/website/static
  rsync -avz img/ seber:/var/website/img
  rsync -avz .env seber:/var/website/.env

clean:
  rm -rf static/main.css data
