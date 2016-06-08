#include <stdio.h>
#include <string.h>
#include <math.h>
#include <tgmath.h>
#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

struct Robot {
    int robot_id;
    float x, y, w, vx, vy, vw;
};

struct Ball {
    float x, y, z, vx, vy, vz;
};

enum Action {
    None,
    Goto,
    GotoAndKick,
    GotoAndChip,
};

float distance(float ax, float ay, float bx, float by);
float distance2(float ax, float ay, float bx, float by);

int main() {
    fprintf(stderr, "started\n");

    // Version check I/O

    const int compat_version = 1;
    char magic_string[32];
    int version;
    scanf("%s %i", magic_string, &version);
    if (strcmp(magic_string, "ROBOIME_AI_PROTOCOL") == 0 && version == compat_version) {
        printf("COMPATIBLE %i\n", compat_version);
    } else {
        printf("NOT_COMPATIBLE %i\n", compat_version);
        return 0;
    }
    fflush(stdout);

    fprintf(stderr, "compatible\n");

    // Geometry input

    float field_length;
    float field_width;
    float goal_width;
    float center_circle_radius;
    float defense_radius;
    float defense_stretch;

    scanf("%f %f %f %f %f %f",
        &field_length,
        &field_width,
        &goal_width,
        &center_circle_radius,
        &defense_radius,
        &defense_stretch
    );

    fprintf(stderr, "initialized\n");

    // Game state I/O
    struct Robot robots_player[12];
    struct Robot robots_opponent[12];
    struct Ball ball;
    // stabilize the solution
    int previous_first_to_ball = -2;

    while (1) {
        int ids_player[6], ids_opponent[6];
        int ids_count_player = 0, ids_count_opponent = 0;

        int counter;
        float timestamp;
        char referee_state;
        float referee_time_left;
        int score_player, score_opponent;
        int goalie_id_player, goalie_id_opponent;
        scanf("%i %f %c %f %i %i %i %i",
            &counter,
            &timestamp,
            &referee_state,
            &referee_time_left,
            &score_player,
            &score_opponent,
            &goalie_id_player,
            &goalie_id_opponent
        );
        scanf("%f %f %f %f", &ball.x, &ball.y, &ball.vx, &ball.vy);

        int robot_count_player;
        scanf("%i", &robot_count_player);
        for (int i = 0; i < robot_count_player; ++i) {
            int robot_id;
            float robot_x, robot_y, robot_w, robot_vx, robot_vy, robot_vw;
            scanf("%i %f %f %f %f %f %f", &robot_id, &robot_x, &robot_y, &robot_w, &robot_vx, &robot_vy, &robot_vw);
            robots_player[robot_id].x = robot_x;
            robots_player[robot_id].y = robot_y;
            robots_player[robot_id].w = robot_w;
            robots_player[robot_id].vx = robot_vx;
            robots_player[robot_id].vy = robot_vy;
            robots_player[robot_id].vw = robot_vw;
            ids_player[ids_count_player++] = robot_id;
        }

        int robot_count_opponent;
        scanf("%i", &robot_count_opponent);
        for (int i = 0; i < robot_count_opponent; ++i) {
            int robot_id;
            float robot_x, robot_y, robot_w, robot_vx, robot_vy, robot_vw;
            scanf("%i %f %f %f %f %f %f", &robot_id, &robot_x, &robot_y, &robot_w, &robot_vx, &robot_vy, &robot_vw);
            robots_opponent[robot_id].x = robot_x;
            robots_opponent[robot_id].y = robot_y;
            robots_opponent[robot_id].w = robot_w;
            robots_opponent[robot_id].vx = robot_vx;
            robots_opponent[robot_id].vy = robot_vy;
            robots_opponent[robot_id].vw = robot_vw;
            ids_opponent[ids_count_opponent++] = robot_id;
        }

        int first_to_ball = ids_player[0];
        int second_to_ball = -1;
        {
            float distance2_to_ball = 1.0e99;
            float distance2_to_ball2 = 1.0e99;
            // find the closest robot to the ball that is not the goalie
            for (int i = 1; i < ids_count_player; ++i) {
                int id = ids_player[i];
                if (id == goalie_id_player) continue;
                float d2 = distance2(robots_player[id].x, robots_player[id].y, ball.x, ball.y);
                if (d2 < distance2_to_ball) {
                    if (distance2_to_ball < distance2_to_ball2) {
                        distance2_to_ball2 = distance2_to_ball;
                        second_to_ball = first_to_ball;
                    }
                    distance2_to_ball = d2;
                    first_to_ball = id;
                } else if (d2 < distance2_to_ball2) {
                    distance2_to_ball2 = d2;
                    second_to_ball = id;
                }
            }
            if (fabs(distance2_to_ball - distance2_to_ball2) < 0.150) {
                if (second_to_ball == previous_first_to_ball) {
                    second_to_ball = first_to_ball;
                    first_to_ball = previous_first_to_ball;
                }
            }
            previous_first_to_ball = first_to_ball;
        }
        //fprintf(stderr, "f: %i\ts: %i\n", first_to_ball, second_to_ball);

        printf("%i\n", counter);
        for (int i = 0; i < ids_count_player; ++i) {
            int id = ids_player[i];
            struct Robot *robot = &robots_player[id];
            float tx, ty, tw;
            enum Action action = None;

            if (id == goalie_id_player) {
                tx = -field_length / 2 + 0.090;
                ty = 0.0;
                tw = atan2(ball.y - robot->y, ball.x - robot->x);
                action = GotoAndKick;
            } else switch (referee_state) {
                case 'k': // KICKOFF
                case 'i': // INDIRECT
                case 'd': // DIRECT
                case 'N': // NORMAL
                    if (id == first_to_ball) {
                        float kx = field_length / 2;
                        float ky = 0.0;
                        float r = 0.080 / sqrt((kx - ball.x) * (kx - ball.x) + (ky - ball.y) * (ky - ball.y));
                        tx = ball.x - r * (kx - ball.x);
                        ty = ball.y - r * (ky - ball.y);
                        tw = atan2(ky - ball.y, kx - ball.x);
                        action = GotoAndKick;
                        break;
                    } else if (id == second_to_ball) {
                        float px = -field_length / 2;
                        float py = 0.0;
                        float r = 1.090 / sqrt((px - ball.x) * (px - ball.x) + (py - ball.y) * (py - ball.y));
                        tx = ball.x + r * (px - ball.x);
                        ty = ball.y + r * (py - ball.y);
                        tw = atan2(ball.y - py, ball.x - px);
                        action = Goto;
                        break;
                    }
                case 'p': // PRE_KICKOFF
                case 'P': // OPPONENT_PRE_KICKOFF
                case 'K': // OPPONENT_KICKOFF
                case 'I': // OPPONENT_INDIRECT
                case 'D': // OPPONENT_DIRECT
                case 'S': // STOP
                    if (id == first_to_ball || id == second_to_ball) {
                        float px = -field_length / 2;
                        float py = 0.0;
                        float pd = sqrt((px - ball.x) * (px - ball.x) + (py - ball.y) * (py - ball.y));
                        float r = 0.590 / pd;
                        float r2 = 0.095 / pd;
                        tx = ball.x + r * (px - ball.x);
                        ty = ball.y + r * (py - ball.y);
                        if (second_to_ball != -1) {
                            float txa = tx + r2 * (py - ball.y);
                            float tya = ty - r2 * (px - ball.x);
                            float txb = tx - r2 * (py - ball.y);
                            float tyb = ty + r2 * (px - ball.x);
                            int id_other = id == first_to_ball? second_to_ball : first_to_ball;
                            struct Robot *other = &robots_player[id_other];
                            // dra: distance from robot to a, ...
                            float dra = distance2(robot->x, robot->y, txa, tya);
                            float doa = distance2(other->x, other->y, txa, tya);
                            float drb = distance2(robot->x, robot->y, txb, tyb);
                            float dob = distance2(other->x, other->y, txb, tyb);
                            // minimize distance walked by both robots
                            if (dra + dob < drb + doa) {
                                tx = txa;
                                ty = tya;
                            } else {
                                tx = txb;
                                ty = tyb;
                            }
                        }
                        //tw = atan2(ty - py, tx - px);
                        tw = atan2(ball.y - ty, ball.x - tx);
                        action = Goto;
                    } else {
                        float txa = -field_length / 2 + defense_radius + 0.090;
                        float tya = 0.000;
                        float txb = -field_length / 2 + defense_radius + 0.090;
                        float tyb = 0.180;
                        float txc = -field_length / 2 + defense_radius + 0.090;
                        float tyc = -0.180;
                        int id0 = id;
                        int id1 = -1;
                        int id2 = -1;
                        for (int i = 0; i < ids_count_player; i++) {
                            if (ids_player[i] == id) continue;
                            if (ids_player[i] == first_to_ball) continue;
                            if (ids_player[i] == second_to_ball) continue;
                            if (ids_player[i] == goalie_id_player) continue;
                            if (id1 == -1) { id1 = ids_player[i]; continue; }
                            if (id2 == -1) { id2 = ids_player[i]; continue; }
                        }
                        //fprintf(stderr, "ids: %i %i %i ", id0, id1, id2);
                        float d0a = distance2(robots_player[id0].x, robots_player[id0].y, txa, tya);
                        float d0b = distance2(robots_player[id0].x, robots_player[id0].y, txb, tyb);
                        float d0c = distance2(robots_player[id0].x, robots_player[id0].y, txc, tyc);
                        float d1a = distance2(robots_player[id1].x, robots_player[id1].y, txa, tya);
                        float d1b = distance2(robots_player[id1].x, robots_player[id1].y, txb, tyb);
                        float d1c = distance2(robots_player[id1].x, robots_player[id1].y, txc, tyc);
                        float d2a = distance2(robots_player[id2].x, robots_player[id2].y, txa, tya);
                        float d2b = distance2(robots_player[id2].x, robots_player[id2].y, txb, tyb);
                        float d2c = distance2(robots_player[id2].x, robots_player[id2].y, txc, tyc);
                        float s0 = d0a + d1b + d2c;
                        float s1 = d0a + d1c + d2b;
                        float s2 = d0b + d1a + d2c;
                        float s3 = d0b + d1c + d2a;
                        float s4 = d0c + d1a + d2b;
                        float s5 = d0c + d1b + d2a;
                        float best = fmin(s0, fmin(s1, fmin(s2, fmin(s3, fmin(s4, s5)))));
                        if (best == s0 || best == s1) {
                            tx = txa;
                            ty = tya;
                        } else if (best == s2 || best == s3) {
                            tx = txb;
                            ty = tyb;
                        } else {
                            tx = txc;
                            ty = tyc;
                        }
                        action = Goto;
                    }
                    break;
                case 'x': // PRE_PENALTY
                case 'y': // PENALTY
                case 'X': // OPPONENT_PRE_PENALTY
                case 'Y': // OPPONENT_PENALTY
                default:
                    break;
            }

            float v_tangent = 0.0f;
            float v_normal = 0.0f;
            float v_angular = 0.0f;
            float kick_force = 0.0f;
            float chip_force = 0.0f;
            int dribble = 0;
            const float PL = 1.50f;
            const float PW = 1.80f;
            switch (action) {
                case GotoAndKick:
                    kick_force = 7.0f;
                case GotoAndChip:
                    if (kick_force == 0.0)
                        chip_force = 7.0f;
                case Goto:
                    //fprintf(stderr, "w: %f\n", w);
                    v_tangent = ((tx - robot->x) * cos(robot->w) + (ty - robot->y) * sin(robot->w)) * PL;
                    v_normal = (-(tx - robot->x) * sin(robot->w) + (ty - robot->y) * cos(robot->w)) * PL;
                    v_angular = (fmod(tw - robot->w + 5 * M_PI, 2 * M_PI) - M_PI) * PW;
                    break;
                case None:
                default:
                    break;
            }
            printf("%f %f %f %f %f %i\n", v_tangent, v_normal, v_angular, kick_force, chip_force, dribble);
        }

        fflush(stdout);
    }
}

float distance(float ax, float ay, float bx, float by) {
    return sqrt(distance2(ax, ay, bx, by));
}

float distance2(float ax, float ay, float bx, float by) {
    return (ax - bx) * (ax - bx) + (ay - by) * (ay - by);
}
