#include "src/IO/Comms/i2c.hpp"
#include "src/BoardDefinitions.hpp"
//#define BOARD_TYPE DRIVER_BOARD
#define BOARD_TYPE DRIVER_BOARD
#define ACTIVE_BOARD BOARD_1

Comm::i2c comm(0);
#if BOARD_TYPE == DRIVER_BOARD
void setup()
{
    comm.Connect();
}

void loop()
{
    delay(5000);
    size_t dataSize = sizeof(char) * 5;
    byte* charData = (byte*)malloc(dataSize);
    memcpy(charData, "test\0", dataSize);
    auto message = MessageProtocol::MessageByteStream(dataSize, charData);
    comm.Send(1, message);
	delay(5000);
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
