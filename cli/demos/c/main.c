#include <stdio.h>
#include <string.h>
#include <math.h>

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
    float free_kick_from_defense_distance;
    float penalty_spot_from_field_line_dist;
    float penalty_line_from_spot_dist;

    scanf("%f %f %f %f %f %f %f %f %f",
        &field_length,
        &field_width,
        &goal_width,
        &center_circle_radius,
        &defense_radius,
        &defense_stretch,
        &free_kick_from_defense_distance,
        &penalty_spot_from_field_line_dist,
        &penalty_line_from_spot_dist
    );

    fprintf(stderr, "initialized\n");

    // Game state I/O

    while (1) {

        // State

        int ids[6];
        int ids_count = 0;
        float x = 0.0f, y = 0.0f, w = 0.0f;
        float tx = 0.0f, ty = 0.0f, tw = 0.0f;

        // Input

        int counter;
        float timestamp;
        char referee_state;
        float referee_time_left;
        int score_player, score_opponent;
        int goalie_id_player, goalie_id_opponent;
        int robot_count_player, robot_count_opponent;

        scanf("%i %f %c %f %i %i %i %i %i %i",
            &counter,
            &timestamp,
            &referee_state,
            &referee_time_left,
            &score_player,
            &score_opponent,
            &goalie_id_player,
            &goalie_id_opponent,
            &robot_count_player,
            &robot_count_opponent
        );

        float ball_x, ball_y, ball_vx, ball_vy;

        scanf("%f %f %f %f",
            &ball_x,
            &ball_y,
            &ball_vx,
            &ball_vy
        );

        for (int i = 0; i < robot_count_player; ++i) {
            int robot_id;
            float robot_x, robot_y, robot_w, robot_vx, robot_vy, robot_vw;

            scanf("%i %f %f %f %f %f %f",
                &robot_id,
                &robot_x,
                &robot_y,
                &robot_w,
                &robot_vx,
                &robot_vy,
                &robot_vw
            );

            ids[ids_count++] = robot_id;
            if (robot_id == 0) {
                x = robot_x;
                y = robot_y;
                w = robot_w;
            }
        }

        for (int i = 0; i < robot_count_opponent; ++i) {
            int robot_id;
            float robot_x, robot_y, robot_w, robot_vx, robot_vy, robot_vw;

            scanf("%i %f %f %f %f %f %f",
                &robot_id,
                &robot_x,
                &robot_y,
                &robot_w,
                &robot_vx,
                &robot_vy,
                &robot_vw
            );
        }

        tx = ball_x;
        ty = ball_y;
        tw = 0;

        printf("%i\n", counter);

        for (int i = 0; i < ids_count; ++i) {
            float v_tangent = 0.0f;
            float v_normal = 0.0f;
            float v_angular = 0.0f;
            float kick_force = 0.0f;
            float chip_force = 0.0f;
            int dribble = 0;

            if (ids[i] == 0) {
                const float PL = 0.40f;
                const float PW = 0.80f;
                v_tangent  = PL * ((tx - x) * cos(w) + (ty - y) * sin(w));
                v_normal   = PL * ((ty - y) * cos(w) + (tx - x) * sin(w));
                v_angular  = PW * (tw - w);
                kick_force = 4.0f;
                chip_force = 0.0f;
                dribble = 1;
            }

            //fprintf(stderr, "%f %f %f %f %f %i\n", v_tangent, v_normal, v_angular, kick_force, chip_force, dribble);
            printf("%f %f %f %f %f %i\n", v_tangent, v_normal, v_angular, kick_force, chip_force, dribble);
        }

        fflush(stdout);
    }
}
