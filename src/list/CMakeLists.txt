# Minimum CMake version required
cmake_minimum_required(VERSION 3.10)

# Project name and C++ standard
project(LinkedListProject VERSION 1.0)

# Set the C++ standard to C++11 (or later)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Add library for the linked list
add_library(linked_list_lib STATIC linked_list.cpp)

# Include the header files directory in the project
target_include_directories(linked_list_lib PRIVATE ${CMAKE_CURRENT_SOURCE_DIR})
