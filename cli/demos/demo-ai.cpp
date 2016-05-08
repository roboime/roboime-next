#include <iostream>
#include <stdio.h>
#include <fstream>
#include <vector>
#include <math.h>

const double PI = 3.14159265; 

int main (){

	//Version check I/O

	char protocol_version_string[32];
	int version;
	std::cin >> protocol_version_string;
	std::cin >> version;
	if (version == 1){
		std::cout << "COMPATIBLE 1" << std::endl;
	}
	else {
		std::cout << "NOT_COMPATIBLE 1" << std::endl;
	}

	//Geometry input
	
	float field_length;
	float field_width;
	float goal_width;
	float center_circle_radius;
	float defense_radius;
	float defense_stretch;
	float free_kick_from_defense_distance;
	float penalty_spot_from_field_line_dist;
	float penalty_line_from_spot_dist;

	std::cin >> field_length 
	>> field_width 
	>> goal_width 
	>> center_circle_radius 
	>> defense_radius 
	>> defense_stretch 
	>> free_kick_from_defense_distance 
	>> penalty_spot_from_field_line_dist 
	>> penalty_line_from_spot_dist;

	//Game state I/O
	int counter;
	float timestamp;
	char referee_state;
	int referee_time_left;
	int score_player;
	int score_opponent;
	int goalkeeper_id_player;
	int goalkeeper_id_opponent;
	int robot_num_player;
	int robot_num_opponent;
	float ball_x, ball_y, ball_vx, ball_vy;

	while (true){
		// State
		std::vector<int> identifiers;
		float x = 0.0f, y = 0.0f, w = 0.0f;
		float tx = 0.0f, ty = 0.0f, tw = 0.0f;

		// Input

		std::cin >> counter
		>> timestamp 
		>> referee_state >> referee_time_left
		>> score_player >> score_opponent
		>> goalkeeper_id_player
		>> goalkeeper_id_opponent
		>> robot_num_player
		>> robot_num_opponent;

		std::cin >> ball_x >> ball_y >> ball_vx >> ball_vy;

		for (int i = 0; i < robot_num_player; ++i ){
			int robot_id;
			float robot_x, robot_y, robot_w, robot_vx, robot_vy, robot_vw;
			std::cin >> robot_id >> robot_x >> robot_y >> robot_w >> robot_vx >> robot_vy >> robot_vw;
			identifiers.push_back(robot_id);
			if (robot_id == 0){
				x = robot_x;
				y = robot_y;
				w = robot_w;
			}
		}

		for (int i = 0; i < robot_num_opponent; ++i){
			int robot_id;
			float robot_x, robot_y, robot_w, robot_vx, robot_vy, robot_vw;
			std::cin >> robot_id >> robot_x >> robot_y >> robot_w >> robot_vx >> robot_vy >> robot_vw;
		}
		tx = ball_x;
		ty = ball_y;
		tw = 0;
		std::cout << counter << std::endl;
		const float p_t = 0.4f;
		const float p_w = 0.8000f;
		for (int i = 0; i < identifiers.size() ; ++i){
			if (identifiers.at(i) == 0){
				const float v_tan = p_t*((tx-x)*cos(w*PI/180.0) + (ty-y)*sin(w*PI/180.0));
				const float v_norm = p_t*((ty-y)*cos(w*PI/180.0) + (tx-x)*sin(w*PI/180.0));
				const float v_ang = p_w*(tw-w);
				const float kick_x = 4.0f;
				const float kick_z = 0.0f;
				const bool spin = true;
				std::cout << v_tan << " " << v_norm << " " << v_ang << " " << kick_x << " " << kick_z << " " << spin << std::endl;
			}
			else{
				const float v_tan = 0.0f;
				const float v_norm = 0.0f;
				const float v_ang = 0.0f;
				const float kick_x = 0.0f;
				const float kick_z = 0.0f;
				const bool spin = false;
				std::cout << v_tan << " " << v_norm << " " << v_ang << " " << kick_x << " " << kick_z << " " << spin << std::endl;			}
		}
	}
}