cmake_minimum_required (VERSION 3.20) 

project(BackendTestsLANGUAGES CXX)

add_compile_definitions(_COMPUTER_ 1)

Include(FetchContent)

FetchContent_Declare(
  Catch2
  GIT_REPOSITORY https://github.com/catchorg/Catch2.git
  GIT_TAG        v3.0.0-preview3
)

FetchContent_MakeAvailable(Catch2)
# Configure our test executables

add_executable(MessageProtocolList backend_tests/MessageProtocol_tests.cpp 
backend/src/Message.cpp)
list(APPEND PROJECT_LIST MessageProtocolList)

add_executable(FactoryMessageTests backend_tests/FactoryMessage_tests.cpp
  backend/src/Message.cpp
  backend/src/IO/IOFactory.cpp
  backend/src/IO/Sensor/SensorDefinitions.cpp
  backend/src/IO/Sensor/SensorMessage.cpp
  backend/src/IO/Sensor/NPhotoResistor.cpp
  backend/src/IO/Sensor/NServoMotor.cpp
  backend/src/IO/Sensor/NMotor.cpp
  backend/src/IO/Sensor/NPixelLight.cpp
  backend/src/IO/Sensor/NetworkSensorInterface.cpp
  backend_tests/MockObjects/MockComm.cpp
  backend/src/IO/Sensor/NetworkSensorInterface.cpp)
list(APPEND PROJECT_LIST FactoryMessageTests)

add_executable(SensorMessageTests backend_tests/SensorMessaging_tests.cpp 
  backend/src/Message.cpp
  backend/src/IO/Sensor/SensorMessage.cpp
  backend/src/IO/Sensor/SensorDefinitions.cpp
)
list(APPEND PROJECT_LIST SensorMessageTests)
add_executable(SensorMessageExtendedTests backend_tests/SensorMessageExtended_tests.cpp 
  backend/src/Message.cpp
  backend/src/IO/Sensor/SensorMessage.cpp
  backend/src/IO/Sensor/SensorDefinitions.cpp
  backend/src/IO/Sensor/NetworkSensorInterface.cpp
  backend/src/IO/Sensor/NPhotoResistor.cpp
  backend_tests/MockObjects/MockComm.cpp
  backend_tests/MockObjects/MockPhotoResistor.cpp
)
list(APPEND PROJECT_LIST SensorMessageExtendedTests)

foreach( name ${PROJECT_LIST} )
  message(${name})
  # Set compile flags.
  target_compile_options(${name} PRIVATE
    # Clang
    $<$<OR:$<CXX_COMPILER_ID:Clang>,$<CXX_COMPILER_ID:AppleClang>>:
        -Weverything -fcolor-diagnostics
        # Disable specific warnings.
        -Wno-c++98-compat -Wno-c++98-compat-pedantic -Wno-padded
        -Wno-deprecated-declarations -Wno-exit-time-destructors
        -Wno-switch-enum -Wno-weak-vtables -Wno-global-constructors>
    # GCC
    $<$<CXX_COMPILER_ID:GNU>:-Wall -Wextra -Wpedantic -fdiagnostics-color=always>
    # Visual Studio
    $<$<CXX_COMPILER_ID:MSVC>:/W4>
  )
  target_link_libraries(${name} PRIVATE Catch2::Catch2 Catch2::Catch2WithMain)
  set_target_properties(${name} PROPERTIES
      CXX_STANDARD 11
      CXX_STANDARD_REQUIRED ON
      CXX_EXTENSIONS OFF
  )

endforeach()

list(APPEND CMAKE_MODULE_PATH ${catch2_SOURCE_DIR}/extras)
include(CTest)
include(Catch)

foreach( name ${PROJECT_LIST} )
  catch_discover_tests(${name})
endforeach()


