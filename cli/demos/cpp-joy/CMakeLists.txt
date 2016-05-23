cmake_minimum_required(VERSION 2.8)
project(demo-joystick)
include_directories("${PROJECT_SOURCE_DIR}")
add_executable(demo-joystick
    main.cpp
    joystick.cc
)
