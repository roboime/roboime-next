#include <iostream>
#include <vector>
#include <cmath>

int main() {
    using namespace std;

    cin.sync_with_stdio(false);
    cout.sync_with_stdio(false);
    cerr.sync_with_stdio(false);

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
        return 0;
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

    // Game state I/O

    while (true) {

        // State

        vector<int> ids;
        float x = 0.0f, y = 0.0f, w = 0.0f;
        float tx = 0.0f, ty = 0.0f, tw = 0.0f;

        // Input

        int counter;
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

        int robot_count_player;
        cin >> robot_count_player;

        for (int i = 0; i < robot_count_player; ++i) {
            int robot_id;
            float robot_x, robot_y, robot_w, robot_vx, robot_vy, robot_vw;

            cin >> robot_id >> robot_x >> robot_y >> robot_w >> robot_vx >> robot_vy >> robot_vw;
            ids.push_back(robot_id);
            if (robot_id == 0) {
                x = robot_x;
                y = robot_y;
                w = robot_w;
            }
        }

        int robot_count_opponent;
        cin >> robot_count_opponent;

        for (int i = 0; i < robot_count_opponent; ++i) {
            int robot_id;
            float robot_x, robot_y, robot_w, robot_vx, robot_vy, robot_vw;

            cin >> robot_id >> robot_x >> robot_y >> robot_w >> robot_vx >> robot_vy >> robot_vw;
        }

        tx = ball_x;
        ty = ball_y;
        tw = 0.0f;

        cout << counter << endl;

        for (int i = 0; i < ids.size() ; ++i) {
            const int robot_id = ids[i];
            float v_tangent = 0.0f;
            float v_normal = 0.0f;
            float v_angular = 0.0f;
            float kick_force = 0.0f;
            float chip_force = 0.0f;
            bool dribble = false;

            if (robot_id == 0) {
                const float PL = 0.40f;
                const float PW = 0.80f;
                v_tangent  = PL * ((tx - x) * cos(w) + (ty - y) * sin(w));
                v_normal = PL * ((ty - y) * cos(w) + (tx - x) * sin(w));
                v_angular  = PW * (tw - w);
                kick_force = 4.0f;
                chip_force = 0.0f;
                dribble = true;
            }

            cout << v_tangent << " " << v_normal << " " << v_angular << " " << kick_force << " " << chip_force << " " << dribble << endl;
        }
    }
}
