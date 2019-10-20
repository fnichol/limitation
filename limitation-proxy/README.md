# limitation-proxy

|                  |                                                         |
| ---------------: | ------------------------------------------------------- |
|               CI | [![CI Status][badge-ci-overall]][ci]                    |
|   Latest Version | [![Latest version][badge-version]][crate]               |
|    Documentation | [![Documentation][badge-docs]][docs]                    |
|  Crate Downloads | [![Crate downloads][badge-crate-dl]][crate]             |
| GitHub Downloads | [![GitHub downloads][badge-github-dl]][github-releases] |
|          License | [![Crate license][badge-license]][github]               |

**Table of Contents**

<!-- toc -->

- [About](#about)
- [Installation](#installation)
  - [Cargo Install](#cargo-install)
  - [From Source](#from-source)
- [Usage](#usage)
- [Ideas and Future Work](#ideas-and-future-work)
- [CI Status](#ci-status)
  - [Build (master branch)](#build-master-branch)
  - [Test (master branch)](#test-master-branch)
  - [Check (master branch)](#check-master-branch)
- [Code of Conduct](#code-of-conduct)
- [Issues](#issues)
- [Contributing](#contributing)
- [Release History](#release-history)
- [Authors](#authors)
- [License](#license)

<!-- tocstop -->

A reverse proxy service with configurable rate limiting

## About

The `limitation-proxy` service is an HTTP reverse proxy which sits in front of
another HTTP service and will perform rate limiting on all requests that pass
through it. The rate limiting is a variant of a fixed window rate limiting
strategy and Redis is used for its persistence.

```text
        +------------------+            +-----------------------+
        |                  |            |                       |
        |                  |            |                       |
    --->+ limitation-proxy +----------->+        proxied        |
        |                  +<-----------+        back end       |
        |                  |            |                       |
        |                  |            |                       |
        +--------+---------+            +-----------------------+
                 |
                 v
        +--------+---------+
        |                  |
        |                  |
        |      Redis       |
        |                  |
        |                  |
        +------------------+
```

## Installation

### Cargo Install

If [Rust](https://rustup.rs/) is installed, then installing with Cargo is
straight forward:

```console
$ cargo install limitation-proxy
```

### From Source

To install from source, you can clone the Git repository, build with Cargo and
copy the binary into a destination directory. This will build the project from
the latest commit on the master branch, which may not correspond to the latest
stable release:

```console
$ git clone https://github.com/fnichol/limitation.git
$ cd limitation
$ cargo build --bin limitation-proxy --release
$ cp ./target/release/limitation-proxy /dest/path/
```

## Usage

You can use the `-h`/`--help` flag to get:

```console
$ limitation-proxy
limitation-proxy 0.1.1
Fletcher Nichol <fnichol@nichol.ca>

A reverse proxy service with configurable rate limiting

Project home page: https://github.com/fnichol/limitation

USAGE:
    limitation-proxy

OPTIONS:
    -b, --bind <BIND>        Bind address for the service [env: BIND_ADDR]
                             [default: 0.0.0.0:8080]
    -H, --header <HEADER>    Header to be used as the key for rate-limiting
                             [default: authorization]
    -l, --limit <LIMIT>      Maximum number of requests per key in the period
                             [default: 5000]
    -P, --period <PERIOD>    Duration of period window in seconds [default:
                             3600]
    -p, --proxy <PROXY>      Backend proxy URL target [env: PROXY_URL]
                             [default: http://127.0.0.1:8000]
    -r, --redis <REDIS>      Redis URL for persistence [env: REDIS_URL]
                             [default: redis://127.0.0.1/]
    -h, --help               Prints help information
    -V, --version            Prints version information
```

A running Redis instance is required for `limitation-proxy`, so we'll assume one
is running locally and reachable at `127.0.0.1:6379` which happens to be this
service's default as well. Starting the service without any arguments runs with
the default settings:

```console
$ limitation-proxy
```

You can also override the defaults with the arguments. For example, this will
run the service limiting up to 100 requests in a 60 second window:

```console
$ limitation-proxy --limit 100 --period 60
```

## Ideas and Future Work

These are some ideas and potential future work for this project. If you're
reading this, then maybe you're curious or interested in helping out? Great! Be
sure to check out the [Contributing][#contributing] section and dig in!

- Allowing configurable rules to match particular HTTP requests, possibly by
  HTTP verb, path, headers, etc.
- Allow more granular matching inside the header value. Currently, an
  `Authorization` header will only key on the full value string which may
  include both basic authentication and token based authentication.
- Support incoming TLS and/or TLS to the proxied back end.
- Add an API path to check a user's current limit status. Note that there would
  likely be some small feature work in the `limitation` crate to support this.
- Allow this service to cover a sub-path on the proxied back end. Currently a
  request to `/a/b` will be proxied to the back end's `/a/b` path. It might be
  useful if the service could take `/a/b` and send it to `/api/v1/a/b` on the
  back end

## CI Status

### Build (master branch)

| Operating System | Stable Rust                                                             | Nightly Rust                                                              | <abbr title="Minimum Supported Rust Version">MSRV</abbr>                |
| ---------------: | ----------------------------------------------------------------------- | ------------------------------------------------------------------------- | ----------------------------------------------------------------------- |
|          FreeBSD | [![FreeBSD Stable Build Status][badge-stable_freebsd-build]][ci-master] | [![FreeBSD Nightly Build Status][badge-nightly_freebsd-build]][ci-master] | [![FreeBSD Oldest Build Status][badge-oldest_freebsd-build]][ci-master] |
|            Linux | [![Linux Stable Build Status][badge-stable_linux-build]][ci-master]     | [![Linux Nightly Build Status][badge-nightly_linux-build]][ci-master]     | [![Linux Oldest Build Status][badge-oldest_linux-build]][ci-master]     |
|            macOS | [![macOS Stable Build Status][badge-stable_macos-build]][ci-master]     | [![macOS Nightly Build Status][badge-nightly_macos-build]][ci-master]     | [![macOS Oldest Build Status][badge-oldest_macos-build]][ci-master]     |
|          Windows | [![Windows Stable Build Status][badge-stable_windows-build]][ci-master] | [![Windows Nightly Build Status][badge-nightly_windows-build]][ci-master] | [![Windows Oldest Build Status][badge-oldest_windows-build]][ci-master] |

### Test (master branch)

| Operating System | Stable Rust                                                           | Nightly Rust                                                            | <abbr title="Minimum Supported Rust Version">MSRV</abbr>              |
| ---------------: | --------------------------------------------------------------------- | ----------------------------------------------------------------------- | --------------------------------------------------------------------- |
|          FreeBSD | [![FreeBSD Stable Test Status][badge-stable_freebsd-test]][ci-master] | [![FreeBSD Nightly Test Status][badge-nightly_freebsd-test]][ci-master] | [![FreeBSD Oldest Test Status][badge-oldest_freebsd-test]][ci-master] |
|            Linux | [![Linux Stable Test Status][badge-stable_linux-test]][ci-master]     | [![Linux Nightly Test Status][badge-nightly_linux-test]][ci-master]     | [![Linux Oldest Test Status][badge-oldest_linux-test]][ci-master]     |
|            macOS | [![macOS Stable Test Status][badge-stable_macos-test]][ci-master]     | [![macOS Nightly Test Status][badge-nightly_macos-test]][ci-master]     | [![macOS Oldest Test Status][badge-oldest_macos-test]][ci-master]     |
|          Windows | [![Windows Stable Test Status][badge-stable_windows-test]][ci-master] | [![Windows Nightly Test Status][badge-nightly_windows-test]][ci-master] | [![Windows Oldest Test Status][badge-oldest_windows-test]][ci-master] |

### Check (master branch)

|        | Status                                            |
| ------ | ------------------------------------------------- |
| Lint   | [![Lint Status][badge-check-lint]][ci-master]     |
| Format | [![Format Status][badge-check-format]][ci-master] |

## Code of Conduct

This project adheres to the Contributor Covenant [code of
conduct][code-of-conduct]. By participating, you are expected to uphold this
code. Please report unacceptable behavior to fnichol@nichol.ca.

## Issues

If you have any problems with or questions about this project, please contact us
through a [GitHub issue][issues].

## Contributing

You are invited to contribute to new features, fixes, or updates, large or
small; we are always thrilled to receive pull requests, and do our best to
process them as fast as we can.

Before you start to code, we recommend discussing your plans through a [GitHub
issue][issues], especially for more ambitious contributions. This gives other
contributors a chance to point you in the right direction, give you feedback on
your design, and help you find out if someone else is working on the same thing.

## Release History

See the [changelog] for a full release history.

## Authors

Created and maintained by [Fletcher Nichol][fnichol] (<fnichol@nichol.ca>).

## License

Licensed under the Mozilla Public License Version 2.0 ([LICENSE.txt][license]).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the MPL-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[badge-check-format]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=check&script=format
[badge-check-lint]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=check&script=lint
[badge-ci-overall]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square
[badge-crate-dl]:
  https://img.shields.io/crates/d/limitation-proxy.svg?style=flat-square
[badge-docs]: https://docs.rs/limitation-proxy/badge.svg?style=flat-square
[badge-github-dl]:
  https://img.shields.io/github/downloads/fnichol/limitation/total.svg?style=flat-square
[badge-license]:
  https://img.shields.io/crates/l/limitation-proxy.svg?style=flat-square
[badge-nightly_freebsd-build]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_nightly_freebsd&script=build
[badge-nightly_freebsd-test]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_nightly_freebsd&script=test
[badge-nightly_linux-build]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_nightly_linux&script=build
[badge-nightly_linux-test]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_nightly_linux&script=test
[badge-nightly_macos-build]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_nightly_macos&script=build
[badge-nightly_macos-test]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_nightly_macos&script=test
[badge-nightly_windows-build]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_nightly_windows&script=build
[badge-nightly_windows-test]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_nightly_windows&script=test
[badge-oldest_freebsd-build]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_1.35.0_freebsd&script=build
[badge-oldest_freebsd-test]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_1.35.0_freebsd&script=test
[badge-oldest_linux-build]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_1.35.0_linux&script=build
[badge-oldest_linux-test]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_1.35.0_linux&script=test
[badge-oldest_macos-build]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_1.35.0_macos&script=build
[badge-oldest_macos-test]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_1.35.0_macos&script=test
[badge-oldest_windows-build]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_1.35.0_windows&script=build
[badge-oldest_windows-test]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_1.35.0_windows&script=test
[badge-stable_freebsd-build]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_stable_freebsd&script=build
[badge-stable_freebsd-test]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_stable_freebsd&script=test
[badge-stable_linux-build]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_stable_linux&script=build
[badge-stable_linux-test]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_stable_linux&script=test
[badge-stable_macos-build]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_stable_macos&script=build
[badge-stable_macos-test]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_stable_macos&script=test
[badge-stable_windows-build]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_stable_windows&script=build
[badge-stable_windows-test]:
  https://img.shields.io/cirrus/github/fnichol/limitation.svg?style=flat-square&task=test_stable_windows&script=test
[badge-version]:
  https://img.shields.io/crates/v/limitation-proxy.svg?style=flat-square
[changelog]:
  https://github.com/fnichol/limitation/blob/master/limitation-proxy/CHANGELOG.md
[ci]: https://cirrus-ci.com/github/fnichol/limitation
[ci-master]: https://cirrus-ci.com/github/fnichol/limitation/master
[code-of-conduct]:
  https://github.com/fnichol/limitation/blob/master/limitation-proxy/CODE_OF_CONDUCT.md
[commonmark]: https://commonmark.org/
[crate]: https://crates.io/crates/limitation-proxy
[docs]: https://docs.rs/limitation-proxy
[fnichol]: https://github.com/fnichol
[github]: https://github.com/fnichol/limitation
[github-releases]: https://github.com/fnichol/limitation/releases
[issues]: https://github.com/fnichol/limitation/issues
[license]:
  https://github.com/fnichol/limitation/blob/master/limitation-proxy/LICENSE.txt
