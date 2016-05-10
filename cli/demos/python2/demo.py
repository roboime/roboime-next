import sys
from math import sin, cos

print >>sys.stderr, 'started'

# Version check I/O

magic_string, version = raw_input().split()
if magic_string == 'ROBOIME_AI_PROTOCOL' and int(version) == 1:
    print 'COMPATIBLE 1'
else:
    print 'NOT_COMPATIBLE 1'
    sys.exit(0)
sys.stdout.flush()

print >>sys.stderr, 'compatible'

# Geometry raw_input

i = iter(raw_input().split())
field_width = float(next(i))
field_height = float(next(i))
goal_width = float(next(i))
center_circle_radius = float(next(i))
defense_radius = float(next(i))
defense_stretch = float(next(i))
free_kick_from_defense_dist = float(next(i))
penalty_spot_from_field_line_dist = float(next(i))
penalty_line_from_spot_dist = float(next(i))

print >>sys.stderr, 'initialized'

# Game state I/O

while True:

    # State

    ids = []
    x, y, w = 0, 0, 0
    tx, ty, tw = 0, 0, 0

    # Input

    i = iter(raw_input().split())
    counter = int(next(i))
    timestamp = float(next(i))
    referee_state = next(i)
    referee_time_left = float(next(i))
    score_player = int(next(i))
    score_opponent = int(next(i))
    goalie_id_player = int(next(i))
    goalie_id_opponent = int(next(i))
    robot_count_player = int(next(i))
    robot_count_opponent = int(next(i))

    ball_x, ball_y, ball_vx, ball_vy = map(float, raw_input().split())

    for _ in range(robot_count_player):
        robot_id, robot_x, robot_y, robot_w, robot_vx, robot_vy, robot_vw = map(float, raw_input().split())
        robot_id = int(robot_id)
        ids.append(robot_id)
        if robot_id == 0:
            x, y, w = robot_x, robot_y, robot_w

    for _ in range(robot_count_opponent):
        robot_id, robot_x, robot_y, robot_w, robot_vx, robot_vy, robot_vw = map(float, raw_input().split())
        robot_id = int(robot_id)

    tx, ty, tw = ball_x, ball_y, 0

    print counter

    for robot_id in ids:
        v_tan = 0.0
        v_norm = 0.0
        v_ang = 0.0
        kick_x = 0.0
        kick_z = 0.0
        spin = 0

        if robot_id == 0:
            PL = 0.40
            PW = 0.80
            v_tan  = PL * ((tx - x) * cos(w) + (ty - y) * sin(w))
            v_norm = PL * ((ty - y) * cos(w) - (tx - x) * sin(w))
            v_ang  = PW * (tw - w)
            kick_x = 4.0
            kick_z = 0.0
            spin = 1

        print v_tan, v_norm, v_ang, kick_x, kick_z, spin

    sys.stdout.flush()
