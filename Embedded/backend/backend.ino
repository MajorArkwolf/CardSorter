#include "src/BoardDefinitions.hpp"


//#define BOARD_TYPE PASSENGER_BOARD
#define BOARD_TYPE DRIVER_BOARD
#define ACTIVE_BOARD BOARD_1

#if BOARD_TYPE == DRIVER_BOARD
#include "src/IO/IOOverseer.hpp"
IO::IOOverseer overseer = IO::IOOverseer();
Comm::SerialJsonComm comm;
void setup()
{
    delay(5000);
    overseer.Setup();
    
}

void loop()
{
    overseer.Update();
}
#endif

#if BOARD_TYPE == PASSENGER_BOARD
#include "src/IO/IOManager.hpp"
IO::IOManager manager = IO::IOManager(1);
void setup()
{
    Serial.begin(9600);
    Serial.println("start");
    manager.Setup();
}

void loop()
{
    manager.Update();
}
#endif
