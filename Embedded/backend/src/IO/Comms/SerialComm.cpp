#include "SerialComm.hpp"
#include "Arduino.h"

namespace Comm {

    SerialComm::SerialComm() {
        
    }

    bool SerialComm::Connect() {
        Serial.begin(9600);
    }

    void SerialComm::Disconnect() {
        Serial.end();
    }

    bool SerialComm::Send(const int address, const MessageProtocol::MessageByteStream& payload) {
        Serial.write(payload.GetByteStream(), payload.GetNumberOfBytes());
    }

    MessageProtocol::MessageByteStream SerialComm::Recieve() {
        auto message = MessageProtocol::MessageByteStream();
        if (Serial.available() > 0) {

        }
        return message;
    }
}