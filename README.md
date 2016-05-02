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

### Initialization output

Currently the following line is expected:

- `COMPATIBLE 1`.

In the future, the following may be used to state explicit incompatibility:

- `NOT_COMPATIBLE 1`, given that `1` is the highest compatible version.

### Initialization input, just after the initialization output

__Line 2__ field data:

- `FIELD_WIDTH`: an float.

__Line 3__ field data:

- `FIELD_HIGH`: an float.

__Line 4__ field data:

- `GOAL_WIDTH`: an float.

__Line 5__ field data:

- `CENTER_CIRCLE_RADIUS`: an float.

__Line 6__ field data:

- `DEFENSE RADIUS`: an float.

__Line 7__ field data:

- `DEFENSE STRETCH`: an float.

__Line 8__ field data:

- `FREE_KICK_FROM_DEFENSE_DIST`: an float.

__Line 9__ field data:

- `PENALTY_SPOT_FROM_FIELD_LINE_DIST`: an float.

__Line 10__ field data:

- `PENALTY_LINE_FROM_SPOT_DIST`: an float.

### Input for one game turn

__Line 1__  general play data:

- `COUNTER`: an integrer, the number of the packet received from the vision system.

__Line 2__ general play data:

- `TIMESTAMP`: an float, the time elapsed since the play started.

__Line 3__ referee data:

- `OUR SCORE`: an integrer, not implemented yet currently always__0__;
- `OPPONENT SCORE`: an integrer, not implemented yet currently always __0__.

__Line 4__ general play data:
- `REFEREE_STATE`, a char, not implemented yet currently always N;
- `REFEREE_TIME_LEFT`, a float.

__Line 5__ ball status data:

- `BALL_X`: an float;
- `BALL_Y`: an float;
- `BALL_VX`: an float, not implemented yet currently always __0__;
- `BALL_VY`: an float, not implemented yet currently always __0__;

__Line 6__ robots status data:

- `GOALKEEPER ID`, an integrer.

__Line 7__ robots general data:

- `NUM_ROBOTS`, an integrer.

__Line 8__ robots general data:

- `OPPONENT_NUM_ROBOTS`, an integrer.

__Next NUM_ROBOTS lines__, robots data:

- `ROBOT_ID`, an integrer;
- `ROBOT_X`, a float;
- `ROBOT_Y`, a float;
- `ROBOT_W`, a float;
- `ROBOT_VX`, a float, not implement yet currently always __0__;
- `ROBOT_VY`, a float, not implement yet currently always __0__;
- `ROBOT_VW`, a float, not implement yet currently always __0__;


__Next OPPONENT_NUM_ROBOTS lines__, robots data:

- `ROBOT_ID`, an integrer;
- `ROBOT_X`, a float;
- `ROBOT_Y`, a float;
- `ROBOT_W`, a float;
- `ROBOT_VX`, a float, not implement yet currently always __0__;
- `ROBOT_VY`, a float, not implement yet currently always __0__;
- `ROBOT_VW`, a float, not implement yet currently always __0__;

### Output for one game turn

__Line 1__, command counter:

- `IDENTIFIER`, a char, expects the char __C__;
- `COUNTER`, an integrer;

__Next NUM_ROBOTS lines__, robots commands:

- `V_TAN`, an float;
- `V_NORM`, an float;
- `V_ANG`, an float; 
- `KICK_X`, an float;
- `KICK_Z`, an float;
- `SPIN`, a bool;

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
