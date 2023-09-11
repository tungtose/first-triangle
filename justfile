dev:
  WINIT_UNIX_BACKEND="x11" cargo watch -x 'run'

build:
  cargo build

clean:
  rm -rf /target
