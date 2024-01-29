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

add_executable(chiba main.c)
target_link_libraries(chiba pico_stdlib)
pico_add_extra_outputs(chiba)
pico_enable_stdio_usb(chiba 1)
pico_enable_stdio_uart(chiba 0)
