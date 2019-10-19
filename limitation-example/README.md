# limitation-example

|         |                                      |
| ------: | ------------------------------------ |
| License | [![License][badge-license]][license] |

**Table of Contents**

<!-- toc -->

- [Usage](#usage)
  - [From Source](#from-source)
- [Authors](#authors)
- [License](#license)

<!-- tocstop -->

An example command line application to demonstrate the `Limiter` in isolation.

## Usage

### From Source

To run this program you need the [Rust](https://rustup.rs/) programming language
install and a checkout of the project source:

```console
$ git clone https://github.com/fnichol/limitation.git
$ cd limitation-example
$ cargo run --bin limitation-example -- MYKEY
```

## Authors

Created and maintained by [Fletcher Nichol][fnichol] (<fnichol@nichol.ca>).

## License

Licensed under the Mozilla Public License Version 2.0 ([LICENSE.txt][license]).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the MPL-2.0 license, shall be
licensed as above, without any additional terms or conditions.

[badge-license]:
  https://img.shields.io/badge/License-MPL%202.0-blue.svg?style=flat-square
[fnichol]: https://github.com/fnichol
[license]:
  https://github.com/fnichol/limitation/blob/master/limitation-proxy/LICENSE.txt
