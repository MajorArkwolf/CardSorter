#include <Firmata.h>
#include "src/NeoPixelPlugin.h"

#define NEO_PIN 9
#define STRIP_LEN 24

NeoPixelPlugin *g_neoPixel = nullptr;

void stringCallback(char *myString)
{
  g_neoPixel->ParseMessage(String(myString));
}

void sysexCallback(byte command, byte argc, byte *argv)
{
  Firmata.sendSysex(command, argc, argv);
}

void setup()
{
  g_neoPixel = new NeoPixelPlugin(NEO_PIN, STRIP_LEN);
  Firmata.setFirmwareVersion(FIRMATA_FIRMWARE_MAJOR_VERSION, FIRMATA_FIRMWARE_MINOR_VERSION);
  Firmata.attach(STRING_DATA, stringCallback);
  Firmata.attach(START_SYSEX, sysexCallback);
  Firmata.begin(57600);
}

void loop()
{
  while (Firmata.available()) {
    Firmata.processInput();
  }
}
