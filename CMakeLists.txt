cmake_minimum_required(VERSION 3.12)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# import Pico SDK
include($ENV{PICO_SDK_PATH}/external/pico_sdk_import.cmake)

# setup project
project(chiba C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

# initalize pico sdk
pico_sdk_init()

# add executable, sources, and include dir
add_executable(chiba)
target_sources(chiba PUBLIC
    ${CMAKE_CURRENT_LIST_DIR}/src/main.c
)

# headers are also in src/
target_include_directories(chiba PUBLIC ./src/)

# link libraries
target_link_libraries(chiba PUBLIC
    pico_stdlib
    hardware_i2c
)

# additional configuration
pico_add_extra_outputs(chiba)
# example_auto_set_url(chiba)
pico_enable_stdio_usb(chiba 1)
pico_enable_stdio_uart(chiba 0)

