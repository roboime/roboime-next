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

### Editor/IDE

Please, setup [EditorConfig](http://editorconfig.org/) on your editor/IDE. Also, when writing code, aim to respect the surrounding style
conventions.  In the future code lints will be added to warn about style that deviates from our preference, for now that's not a priority.

### Optimized builds

    cargo run --release


Game I/O
--------

__Notes:__ linear (`x`, `y`, field measures) units are in meters, angular (`w`) are in radians, linear velocities (`vx`, `vy`) are in
meters per second, and angular velocity in radians per second.

### Initialization input

__Line 1__ version data:

- `"ROBOIME_INTEL_PROTOCOL_VERSION"`: literal string;
- `VERSION_NUMBER`: an integer, currently __1__.

Before the next line, it will wait for the correct initialization output.

__Line 2__ field data:

- `FIELD_LENGTH`: a float.
- `FIELD_WIDTH`: a float.
- `GOAL_WIDTH`: a float.
- `CENTER_CIRCLE_RADIUS`: a float.
- `DEFENSE RADIUS`: a float.
- `DEFENSE STRETCH`: a float.
- `FREE_KICK_FROM_DEFENSE_DIST`: a float.
- `PENALTY_SPOT_FROM_FIELD_LINE_DIST`: a float.
- `PENALTY_LINE_FROM_SPOT_DIST`: a float.

### Initialization output

Currently the following line is expected:

- `COMPATIBLE 1`

In the future, the following may be used to state explicit incompatibility:

- `NOT_COMPATIBLE 1`, given that `1` is the highest compatible version

### Input for one game turn

__Line 1__ general play data:

- `COUNTER`: an integrer, counter for the number of received packets
- `TIMESTAMP`: a float, the time elapsed since the play started
- `OUR SCORE`: an integrer, your team score - not implemented yet currently always `0`
- `OPPONENT SCORE`: an integrer, your opponent team score - not implemented yet currently always `0`
- `REFEREE_STATE`, a char, indicates the referee state in a list of possible states - not implemented yet currently always `'N'`
- `REFEREE_TIME_LEFT`, a float, time left to finish the round - not implemented yet always `-1`
- `GOALKEEPER ID`, an integrer, goalkeeper robot identifier
- `NUM_ROBOTS`, an integrer, number of robots in your team
- `OPPONENT_NUM_ROBOTS`, an integrer, number of robots in your opponent team

__Line 2__ ball status data:

- `BALL_X`: a float, ball x position
- `BALL_Y`: a float, ball y position
- `BALL_VX`: a float, ball x velocity
- `BALL_VY`: a float, ball y velocity

__Next `NUM_ROBOTS` lines__, robots data:

- `ROBOT_ID`, an integrer, robot identifier
- `ROBOT_X`, a float, robot x position
- `ROBOT_Y`, a float, robot y position
- `ROBOT_W`, a float, robot angular position
- `ROBOT_VX`, a float, robot x velocity
- `ROBOT_VY`, a float, robot y velocity
- `ROBOT_VW`, a float, robot angular velocity

__Next OPPONENT_NUM_ROBOTS lines__, robots data:

- `ROBOT_ID`, an integrer, robot identifier
- `ROBOT_X`, a float, robot x position
- `ROBOT_Y`, a float, robot y position
- `ROBOT_W`, a float, robot angular position
- `ROBOT_VX`, a float, robot x velocity
- `ROBOT_VY`, a float, robot y velocity
- `ROBOT_VW`, a float, robot angular velocity

### Output for one game turn

__Line 1__, command counter:

- `COUNTER`, an integrer, of counter for the number of sent packages;

__Next `NUM_ROBOTS` lines__, robots commands:

- `V_TAN`, a float, robot tangencial velocity
- `V_NORM`, a float, robot normal velocity
- `V_ANG`, a float, robot angular velocity
- `KICK_X`, a float, robot x kick velocity
- `KICK_Z`, a float, robot z kick velocity
- `SPIN`, a bool, true (`1`) if the spin is to be turned or false (`0`) else

These actions will be applied on the robots in the order they were given.

### Constraints

The robot diameter is always `0.180`, will call it `ROBOT_DIAM` here.

We'll call π `PI` here.

- `0 <= COUNTER < 1000000`;
- `0 <= OUR_SCORE, OPPONENT_SCORE, <= 10`;
- `0 <= ROBOT_ID <= 12`;
- `|ROBOT_X|, |BALL_X| <= FIELD_LENGTH / 2 + ROBOT_DIAM`;
- `|ROBOT_Y|, |BALL_Y| <= FIELD_WIDTH / 2 + ROBOT_DIAM`;
- `|ROBOT_W| <= PI`;
- `||ROBOT_VX, ROBOT_VY||, ||BALL_VX, BALL_VY|| <= 20.0`;
- `|ROBOT_VW| <= 10.0 * PI`;


License
-------

This code is licensed under the [Mozilla Public License 2.0][mpl2], of which a textual copy is available at [LICENSE.txt](LICENSE.txt).

You are allowed and encouraged to use this software on the RoboCup competitions.  If you do, please let us know.

Although not required, we think it's best for all if improvements are shared.

[roboime]: http://www.roboime.com/
[rust]: https://www.rust-lang.org/
[mpl2]: https://www.mozilla.org/MPL/2.0/
[docs]: http://www.roboime.com/roboime-next/
