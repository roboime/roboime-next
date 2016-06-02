#include <iostream>
#include <vector>
#include <cmath>
#include "joystick.hh"
#include <unistd.h>

using namespace std;

void init_sim(void);
void receive_state(int &counter, int &robot_count_player);
void send_commands(int counter, int robot_counter, float v_ang0, float v_norm0, float v_tan0, float kick_x0, float kick_z0, bool spin0);

int main() {
    init_sim();

    // Create an instance of Joystick
    Joystick joystick("/dev/input/js0");

    // Ensure that it was found and that we can use it
    if (!joystick.isFound()) {
        cerr << "no joystick found" << endl;
        return 1;
    }

    // Game state I/O

    while (true) {
        int axis0_pos, axis3_pos, axis4_pos;
        float kick_x, kick_z;
        bool spin;
        JoystickEvent event;

        if (joystick.sample(&event)) {
            if ((event.isButton())&&(event.number==0)) {
                kick_x = (event.value == 0 ? 0 : 4);
            }
            if ((event.isButton())&&(event.number==1)) {
                kick_z = (event.value == 0 ? 0 : 4);
            }
            if ((event.isButton())&&(event.number==2)) {
                spin = (event.value == 0 ? false : true);
            }
            if ((event.isAxis()) && (event.number == 0)) {
                axis0_pos = event.value;
            }
            if ((event.isAxis()) && (event.number == 3)) {
                axis3_pos = event.value;
            }
            if ((event.isAxis()) && (event.number == 4)) {
                axis4_pos = event.value;
            }
        }
        int robot_count, counter;
        receive_state(counter, robot_count);
        float v_ang, v_tan, v_norm;
        if (axis0_pos > 4000 || axis0_pos < -4000) {
          v_ang = ((float)axis0_pos / 10000.0);
        } else {
          v_ang = 0;
        }
        if ((axis3_pos > 7000) || (axis3_pos < -10000)) {
          v_norm = ((float)axis3_pos / 10000.0);
        } else {
          v_norm = 0;
        }
        if ((axis4_pos>4000) || (axis4_pos<-4000)) {
          v_tan = -((float)axis4_pos / 10000.0);
        } else {
          v_tan = 0;
        }
        send_commands(counter, robot_count, v_ang, v_norm, v_tan, kick_x, kick_z, spin);
    }
}

void init_sim(void) {
    cerr << "started" << endl;

    // Version check I/O

    const int compat_version = 1;
    string magic_string;
    int version;
    cin >> magic_string >> version;
    if (magic_string == "ROBOIME_AI_PROTOCOL" && version == compat_version) {
        cout << "COMPATIBLE " << compat_version << endl;
    } else {
        cout << "NOT_COMPATIBLE " << compat_version << endl;
        return;
    }
    cerr << "compatible" << endl;

    // Geometry input

    float field_length;
    float field_width;
    float goal_width;
    float center_circle_radius;
    float defense_radius;
    float defense_stretch;

    cin >> field_length
        >> field_width
        >> goal_width
        >> center_circle_radius
        >> defense_radius
        >> defense_stretch;

    cerr << "initialized" << endl;
    return;
}

void receive_state(int &counter, int &robot_count_player) {
    // Input

    float timestamp;
    char referee_state;
    float referee_time_left;
    int score_player, score_opponent;
    int goalie_id_player, goalie_id_opponent;

    cin >> counter
        >> timestamp
        >> referee_state >> referee_time_left
        >> score_player >> score_opponent
        >> goalie_id_player >> goalie_id_opponent;

    float ball_x, ball_y, ball_vx, ball_vy;

    cin >> ball_x >> ball_y >> ball_vx >> ball_vy;

    cin >> robot_count_player;

    for (int i = 0; i < robot_count_player; ++i) {
        int robot_id;
        float robot_x, robot_y, robot_w, robot_vx, robot_vy, robot_vw;

        cin >> robot_id >> robot_x >> robot_y >> robot_w >> robot_vx >> robot_vy >> robot_vw;
    }

    int robot_count_opponent;
    cin >> robot_count_opponent;

    for (int i = 0; i < robot_count_opponent; ++i) {
        int robot_id;
        float robot_x, robot_y, robot_w, robot_vx, robot_vy, robot_vw;

        cin >> robot_id >> robot_x >> robot_y >> robot_w >> robot_vx >> robot_vy >> robot_vw;
    }
}

void send_commands(int counter, int robot_counter, float v_ang0, float v_norm0, float v_tan0, float kick_x0, float kick_z0, bool spin0) {
    cout << counter << endl;

    for (int i = 0; i < robot_counter; ++i) {
        float v_tan = 0.0f;
        float v_norm = 0.0f;
        float v_ang = 0.0f;
        float kick_x = 0.0f;
        float kick_z = 0.0f;
        bool spin = false;

        if (i == 0) {
            v_ang=v_ang0;
            v_tan=v_tan0;
            v_norm=v_norm0;
            kick_x=kick_x0;
            kick_z=kick_z0;
            spin=spin0;
        }
        cout << v_tan << " " << v_norm << " " << v_ang << " " << kick_x << " " << kick_z << " " << spin << endl;
    }
}
