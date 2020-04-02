cmake_minimum_required(VERSION 3.15)
project(lab04)

set(CMAKE_CXX_STANDARD 14)

include_directories(.)

add_executable(lab04
        Complex.cpp
        Complex.h
        main.cpp FFT.cpp FFT.h)
