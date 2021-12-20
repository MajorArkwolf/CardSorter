cmake_minimum_required (VERSION 3.15) 
set(PROJECT_NAME BackendTests)
project(${PROJECT_NAME} LANGUAGES CXX)

#find_package(Catch2 CONFIG REQUIRED)

Include(FetchContent)

FetchContent_Declare(
  Catch2
  GIT_REPOSITORY https://github.com/catchorg/Catch2.git
  GIT_TAG        v3.0.0-preview3
)

FetchContent_MakeAvailable(Catch2)

add_executable(${PROJECT_NAME} MessageProtocol_tests.cpp)
target_link_libraries(${PROJECT_NAME} PRIVATE Catch2::Catch2WithMain)

set_target_properties(${PROJECT_NAME} PROPERTIES
    CXX_STANDARD 11
    CXX_STANDARD_REQUIRED ON
    CXX_EXTENSIONS OFF
)

list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/extras)
include(CTest)
include(Catch)
catch_discover_tests(${PROJECT_NAME})