fn main() {
    use std::io::prelude::*;
    use std::io::{stdout, stderr, stdin, BufReader, BufWriter};

    let debug = &mut stderr();
    let mut input = BufReader::new(stdin()).lines();
    let mut output = BufWriter::new(stdout());

    writeln!(debug, "started").unwrap();

    // Version check I/O

    let compat_version = 1;
    let line = input.next().unwrap().unwrap();
    let mut version_input = line.split(' ');
    let magic_string = version_input.next().unwrap();
    let version: u32 = version_input.next().unwrap().parse().unwrap();
    if magic_string == "ROBOIME_AI_PROTOCOL" && version == compat_version {
        writeln!(output, "COMPATIBLE {}", compat_version).unwrap();
    } else {
        writeln!(output, "NOT_COMPATIBLE {}", compat_version).unwrap();
        return;
    }
    output.flush().unwrap();

    writeln!(debug, "compatible").unwrap();

    // Geometry input

    let line = input.next().unwrap().unwrap();
    let mut geom_input = line.split(' ');
    let _field_length: f32 = geom_input.next().unwrap().parse().unwrap();
    let _field_width: f32 = geom_input.next().unwrap().parse().unwrap();
    let _goal_width: f32 = geom_input.next().unwrap().parse().unwrap();
    let _center_circle_radius: f32 = geom_input.next().unwrap().parse().unwrap();
    let _defense_radius: f32 = geom_input.next().unwrap().parse().unwrap();
    let _defense_stretch: f32 = geom_input.next().unwrap().parse().unwrap();
    let _free_kick_from_defense_dist: f32 = geom_input.next().unwrap().parse().unwrap();
    let _penalty_spot_from_field_line_dist: f32 = geom_input.next().unwrap().parse().unwrap();
    let _penalty_line_from_spot_dist: f32 = geom_input.next().unwrap().parse().unwrap();

    writeln!(debug, "initialized").unwrap();

    // Game state I/O

    loop {

        // State

        let mut ids = Vec::with_capacity(6);
        let mut x = 0.0;
        let mut y = 0.0;
        let mut w = 0.0;
        let tx;
        let ty;
        let tw;

        // Input

        let line = input.next().unwrap().unwrap();
        let mut state_input = line.split(' ');
        let counter: u32 = state_input.next().unwrap().parse().unwrap();
        let _timestamp: f32 = state_input.next().unwrap().parse().unwrap();
        let _referee_state: char = state_input.next().unwrap().chars().next().unwrap();
        let _referee_time_left: f32 = state_input.next().unwrap().parse().unwrap();
        let _score_player: u8 = state_input.next().unwrap().parse().unwrap();
        let _score_opponent: u8 = state_input.next().unwrap().parse().unwrap();
        let _goalie_id_player: u8 = state_input.next().unwrap().parse().unwrap();
        let _goalie_id_opponent: u8 = state_input.next().unwrap().parse().unwrap();
        let robot_count_player: u8 = state_input.next().unwrap().parse().unwrap();
        let robot_count_opponent: u8 = state_input.next().unwrap().parse().unwrap();

        let line = input.next().unwrap().unwrap();
        let mut ball_input = line.split(' ');
        let ball_x: f32 = ball_input.next().unwrap().parse().unwrap();
        let ball_y: f32 = ball_input.next().unwrap().parse().unwrap();
        let _ball_vx: f32 = ball_input.next().unwrap().parse().unwrap();
        let _ball_vy: f32 = ball_input.next().unwrap().parse().unwrap();

        for _ in 0..robot_count_player {
            let line = input.next().unwrap().unwrap();
            let mut robot_input = line.split(' ');
            let robot_id: u8 = robot_input.next().unwrap().parse().unwrap();
            let robot_x: f32 = robot_input.next().unwrap().parse().unwrap();
            let robot_y: f32 = robot_input.next().unwrap().parse().unwrap();
            let robot_w: f32 = robot_input.next().unwrap().parse().unwrap();
            let _robot_vx: f32 = robot_input.next().unwrap().parse().unwrap();
            let _robot_vy: f32 = robot_input.next().unwrap().parse().unwrap();
            let _robot_vw: f32 = robot_input.next().unwrap().parse().unwrap();

            ids.push(robot_id);
            if robot_id == 0 {
                x = robot_x;
                y = robot_y;
                w = robot_w;
            }
        }

        for _ in 0..robot_count_opponent {
            let line = input.next().unwrap().unwrap();
            let mut robot_input = line.split(' ');
            let _robot_id: u8 = robot_input.next().unwrap().parse().unwrap();
            let _robot_x: f32 = robot_input.next().unwrap().parse().unwrap();
            let _robot_y: f32 = robot_input.next().unwrap().parse().unwrap();
            let _robot_w: f32 = robot_input.next().unwrap().parse().unwrap();
            let _robot_vx: f32 = robot_input.next().unwrap().parse().unwrap();
            let _robot_vy: f32 = robot_input.next().unwrap().parse().unwrap();
            let _robot_vw: f32 = robot_input.next().unwrap().parse().unwrap();
        }

        tx = ball_x;
        ty = ball_y;
        tw = 0.0;

        writeln!(output, "{}", counter).unwrap();

        for robot_id in ids {
            let mut v_tangent = 0.0;
            let mut v_normal = 0.0;
            let mut v_angular = 0.0;
            let mut kick_force = 0.0;
            let mut chip_force = 0.0;
            let mut dribble = 0;

            if robot_id == 0 {
                const PL: f32 = 0.40;
                const PW: f32 = 0.80;
                v_tangent  = PL * ((tx - x) * w.cos() + (ty - y) * w.sin());
                v_normal   = PL * ((ty - y) * w.cos() + (tx - x) * w.sin());
                v_angular  = PW * (tw - w);
                kick_force = 4.0;
                chip_force = 0.0;
                dribble = 1;
            }

            writeln!(output, "{} {} {} {} {} {}", v_tangent, v_normal, v_angular, kick_force, chip_force, dribble).unwrap();
        }

        output.flush().unwrap();
    }
}
