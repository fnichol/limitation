#!/usr/bin/env sh
# shellcheck shell=sh disable=SC2039

print_usage() {
  local program="$1"

  echo "$program

    Installs a Cargo plugin into a dedicated directory for later caching

    USAGE:
        $program [FLAGS] [--] <PLUGIN>

    FLAGS:
        -h, --help      Prints help information

    ARGS:
        <PLUGIN>  Name of the Cargo plugin
    " | sed 's/^ \{1,4\}//g'
}

main() {
  set -eu
  if [ -n "${DEBUG:-}" ]; then set -v; fi
  if [ -n "${TRACE:-}" ]; then set -xv; fi

  local program
  program="$(basename "$0")"

  OPTIND=1
  while getopts "h-:" arg; do
    case "$arg" in
      h)
        print_usage "$program"
        return 0
        ;;
      -)
        case "$OPTARG" in
          help)
            print_usage "$program"
            return 0
            ;;
          '')
            # "--" terminates argument processing
            break
            ;;
          *)
            print_usage "$program" >&2
            fail "invalid argument --$OPTARG"
            ;;
        esac
        ;;
      \?)
        print_usage "$program" >&2
        fail "invalid argument; arg=-$OPTARG"
        ;;
    esac
  done
  shift "$((OPTIND - 1))"

  if [ -z "${1:-}" ]; then
    print_usage "$program" >&2
    fail "missing <PLUGIN> argument; arg=-$OPTARG"
  fi
  local plugin="$1"
  shift

  local root="$CARGO_HOME/opt/$plugin"

  install_plugin "$plugin" "$root"
}

install_plugin() {
  local plugin="$1"
  local root="$2"

  mkdir -p "$root"
  rustup install stable
  cargo +stable install --root "$root" --force --verbose "$plugin"

  # Create symbolic links for all execuatbles into $CARGO_HOME/bin
  ln -snf "$root/bin"/* "$CARGO_HOME/bin/"
}

fail() {
  echo "" >&2
  echo "xxx $1" >&2
  echo "" >&2
  return 1
}

main "$@"
