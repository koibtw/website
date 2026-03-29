run:
  @just styles
  touch website.db
  cargo run

watch:
  cargo watch -s 'just run'

styles:
  whiskers theme.css.tera -f fall

check:
  cargo fmt --all --check
  cargo clippy -- -D warnings

format:
  cargo fmt --all

sync:
  rsync -rltzv --delete img/ seber:/var/website/img
  rsync -rltzv --delete static/ seber:/var/website/static
  rsync -rltzv --delete styles/ seber:/var/website/styles
  rsync -rltzv --delete keys/ seber:/var/website/keys
  rsync -rltzv --delete .env seber:/var/website/.env

deploy:
  @just clean
  @just styles
  nix build
  nix copy --to ssh://seber "$(readlink -f result)"
  ssh seber "rm -f /var/website/website && ln -sf '$(readlink -f result)' /var/website/website"
  @just sync

clean:
  rm -rf styles/theme.css
