roboime-next
============

Next iteration of [RoboIME][roboime]'s software stack, coded in [Rust][rust].

[![Build Status](https://travis-ci.org/roboime/roboime-next.svg?branch=master)](https://travis-ci.org/roboime/roboime-next)
[![Build Status](https://ci.appveyor.com/api/projects/status/ukyaep1cl4r4v3al?svg=true)](https://ci.appveyor.com/project/jansegre/roboime-next)
[![Coverage Status](https://coveralls.io/repos/github/roboime/roboime-next/badge.svg?branch=master)](https://coveralls.io/github/roboime/roboime-next?branch=master)
[![MPL License](https://img.shields.io/badge/license-MPL--2.0-blue.svg)][mpl2]

[Documentation][docs]

Developing
----------

    cargo run

This will resolve, download and compile dependencies, and compile the project in debug mode and run it. That's it. Really!

> TODO: add guidelines about coding conventions, link to the project blueprints and whatnot

### Optimized builds

    cargo run --release

Game I/O
--------

### Initialization input

__Line 1__ version data:

- `"ROBOIME_INTEL_PROTOCOL_VERSION"`: literal string;
- `VERSION_NUMBER`: an integer, currently __1__.

__Line 2__ field data:

...

### Initialization output

Currently the following line is expected:

- `COMPATIBLE 1`.

In the future, the following may be used to state explicit incompatibility:

- `NOT_COMPATIBLE 1`, given that `1` is the highest compatible version.

### Input for one game turn

...

### Output for one game turn

...

### Constraints

- `0 <= COUNTER < 1000000`;
- `0 <= OUR_SCORE, OPPONENT_SCORE, <= 10`;
- ...

License
-------

This code is licensed under the [Mozilla Public License 2.0][mpl2], of which a textual copy is available at [LICENSE.txt](LICENSE.txt).

You are allowed and encouraged to use this software on the RoboCup competitions.  If you do, please let us know.

Although not required, we think it's best for all if improvements are shared.

[roboime]: http://www.roboime.com/
[rust]: https://www.rust-lang.org/
[mpl2]: https://www.mozilla.org/MPL/2.0/
[docs]: http://www.roboime.com/roboime-next/
