cmake_minimum_required(VERSION 3.0.0)
project(POC VERSION 0.1.0)

include(CTest)
enable_testing()

add_executable(${PROJECT_NAME} main.cpp wordCounter.cpp)
add_subdirectory(listener)
target_link_directories(${PROJECT_NAME} PUBLIC listener)
target_include_directories(${PROJECT_NAME} PUBLIC listener)
target_link_libraries(${PROJECT_NAME} PUBLIC NetworkListener)

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)
