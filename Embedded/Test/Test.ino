#include "src/FeederSystem.h"
#include <Wire.h>

Controller::FeederSystem feederSystem = Controller::FeederSystem(
    Sensor::TripWire(2), 
    Sensor::MotorController(9, 10, 11)
);

void StateChange() {
    feederSystem.Trigger();
}

void setup() {
    Wire.begin(2);
    Wire.onReceive(i2cReceiveEvent);

    Serial.begin(9600);
    attachInterrupt(digitalPinToInterrupt(2), StateChange, CHANGE);
}

void loop() {
    feederSystem.FeedCard();
    delay(100);
}

void i2cReceiveEvent(int howMany) {
  while(1 < Wire.available()) // loop through all but the last
  {
    char c = Wire.read(); // receive byte as a character
    Serial.print(c);      // print the character
  }
  int x = Wire.read();    // receive byte as an integer
  Serial.println(x);      // print the integer
}