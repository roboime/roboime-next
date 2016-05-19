io.stderr:write("started\n")

-- Version check I/O

compat_version = 1
magic_string = io.read(19)
version = io.read("*n")
if magic_string == "ROBOIME_AI_PROTOCOL" and version == compat_version then
    print("COMPATIBLE " .. compat_version)
else
    print("NOT_COMPATIBLE " .. compat_version)
    os.exit(0)
end
io.stdout:flush()

io.stderr:write("compatible\n")

-- Geometry input

field_width = io.read("*n")
field_heigth = io.read("*n")
goal_width = io.read("*n")
center_circle_radius = io.read("*n")
defense_radius = io.read("*n")
defense_stretch = io.read("*n")
free_kick_from_defense_dist = io.read("*n")
penalty_spot_from_field_line_dist = io.read("*n")
penalty_line_from_spot_dist = io.read("*n")

io.stderr:write("initialized\n")

-- Game state I/O

while true do

    -- State

    local ids = {}
    local x, y, w = 0, 0, 0
    local tx, ty, tw = 0, 0, 0

    -- Input

    local counter = io.read("*n")
    local timestamp = io.read("*n", 1) -- read the whitespace
    local referee_state, referee_time_left = io.read(1, "*n")
    local score_player, score_opponent = io.read("*n", "*n")
    local goalie_id_player, goalie_id_opponent = io.read("*n", "*n")
    local robot_count_player, robot_count_opponent = io.read("*n", "*n")

    local ball_x, ball_y, ball_vx, ball_vy = io.read("*n", "*n", "*n", "*n")

    for _ = 1, robot_count_player do
        local robot_id, robot_x, robot_w, robot_vx, robot_vy, robot_vw = io.read("*n", "*n", "*n", "*n", "*n", "*n", "*n")
        table.insert(ids, robot_id)
        if robot_id == 0 then
            x, y, w = robot_x, robot_y, robot_w
        end
    end

    for _ = 1, robot_count_player do
        local robot_id, robot_x, robot_w, robot_vx, robot_vy, robot_vw = io.read("*n", "*n", "*n", "*n", "*n", "*n", "*n")
    end

    tx, ty, tw = ball_w, ball_y, 0

    print(counter)

    for _, robot_id in ipairs(ids) do
        local v_tangent = 0
        local v_normal = 0
        local v_angular = 0
        local kick_force = 0
        local chip_force = 0
        local dribble = 0

        if robot_id == 0 then
            PL = 0.40
            PW = 0.80
            v_tangent  = PL * ((tx - x) * math.cos(w) + (ty - y) * math.sin(w))
            v_normal = PL * ((ty - y) * math.cos(w) + (tx - x) * math.sin(w))
            v_angular  = PW * (tw - w)
            kick_force = 4.0
            chip_force = 0.0
            dribble = 1
        end

        print(table.concat({v_tangent, v_normal, v_angular, kick_force, chip_force, dribble}), " ")
    end

    io.stdout:flush()
end
