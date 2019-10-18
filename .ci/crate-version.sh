#!/usr/bin/env sh
# shellcheck shell=sh disable=SC2039

print_usage() {
  local program="$1"

  echo "$program

    Prints latest version of a Cargo crate

    USAGE:
        $program [FLAGS] [--] <CRATE>

    FLAGS:
        -h, --help      Prints help information

    ARGS:
        <CRATE>  Name of the Cargo crate
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
  local crate="$1"
  shift

  report_version "$crate"
}

report_version() {
  local crate="$1"

  cargo search --limit 1 --quiet "$crate" | head -n 1 | awk -F'"' '{print $2}'
}

fail() {
  echo "" >&2
  echo "xxx $1" >&2
  echo "" >&2
  return 1
}

main "$@"
