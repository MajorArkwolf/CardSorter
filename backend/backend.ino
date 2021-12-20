#include "src/IO/Comms/i2c.hpp"
#include "src/BoardDefinitions.hpp"
//#define BOARD_TYPE DRIVER_BOARD
#define BOARD_TYPE PASSENGER_BOARD
#define ACTIVE_BOARD BOARD_1


#if BOARD_TYPE == DRIVER_BOARD
void setup()
{

}

void loop()
{
	delay(5000)
}
#endif

#if BOARD_TYPE == PASSENGER_BOARD
void setup()
{
    auto Message = Comm::i2c(0);
    Serial.begin(9600);
}

void loop()
{
    delay(100);
}
#endif
