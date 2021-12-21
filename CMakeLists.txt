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
  backend/src/IO/Sensor/NPixelLight.cpp)
list(APPEND PROJECT_LIST FactoryMessageTests)

foreach( name ${PROJECT_LIST} )
  message(${name})
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


