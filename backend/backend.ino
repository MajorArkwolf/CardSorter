
#include "BoardDefinitions.hpp"
#include "i2cComms.hpp"

#define BOARD_TYPE DRIVER_BOARD
#define ACTIVE_BOARD BOARD_1

Comms::i2c comm;
#if BOARD_TYPE == DRIVER_BOARD
#include "IO/IOOverseer.hpp"
void setup()
{
    comm = Comms::i2c();
}

void loop()
{
	
}
#endif

#if BOARD_TYPE == PASSENGER_BOARD
#include "IO/IOManager.hpp"

void setup()
{
    comm = Comms::i2c(ACTIVE_BOARD);
    manager = IO::IOManager();
    manager.Setup();
}

void loop()
{
    manager.Update();
    delay(2000);
}
#endif