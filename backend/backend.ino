#include "src/IO/Comms/i2c.hpp"
#include "src/BoardDefinitions.hpp"
#include "src/IO/IOOverseer.hpp"
#include "src/IO/Sensor/ServoMotor.hpp"
//#define BOARD_TYPE DRIVER_BOARD
#define BOARD_TYPE DRIVER_BOARD
#define ACTIVE_BOARD BOARD_1

//Comm::i2c comm(0);
IO::IOOverseer* overseer;
#if BOARD_TYPE == DRIVER_BOARD
void setup()
{
    overseer = new IO::IOOverseer();
    //comm.Connect();
    overseer->Setup();
    Serial.begin(9600);
}

void loop()
{
    delay(1000);
    overseer->Update();
}
#endif

#if BOARD_TYPE == PASSENGER_BOARD
void setup()
{
    comm = Comm::i2c(1);
    comm.Connect();
    Serial.begin(9600);
}

void loop()
{
    auto recv = comm.Recieve();
    if (recv.GetByteStream() != nullptr) {
        char* message = (char*)recv.GetByteStream();
        Serial.println(message);
    }
    delay(100);
}
#endif
