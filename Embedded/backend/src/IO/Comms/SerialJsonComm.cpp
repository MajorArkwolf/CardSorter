#include "SerialJsonComm.hpp"
#include "Arduino.h"

namespace Comm {

    SerialJsonComm::SerialJsonComm() {
        Serial.begin(9600);
    }

    SerialJsonComm::~SerialJsonComm() {
        Serial.end();
    }

    bool SerialJsonComm::SerialDataPending() {
        return Serial.available() > 0;
    }

    void SerialJsonComm::Send(const DynamicJsonDocument& payload) {
        serializeJson(payload, Serial);
    }

    DynamicJsonDocument SerialJsonComm::Recieve() {
        auto jb = DynamicJsonDocument(400);
        DeserializationError error = deserializeJson(jb, Serial);
        return jb;
    }
}