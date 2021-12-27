#include "SerialJsonComm.hpp"
#include "Arduino.h"
namespace {
    constexpr char G_SOH = char(1);
    constexpr char G_SOX = char(2);
    constexpr char G_ETX = char(3);
    constexpr char G_EOT = char(4); 
    constexpr char G_ENQ = char(5);
    constexpr char G_ACK = char(6);
}


namespace Comm {

    SerialJsonComm::SerialJsonComm() {
        m_out = "init";
    }

    SerialJsonComm::~SerialJsonComm() {
        Serial.end();
    }

    void SerialJsonComm::Connect() {
        Serial.begin(9600);
    }

    bool SerialJsonComm::SerialDataPending() {
        return Serial.available() > 0;
    }

    void SerialJsonComm::Send(const String& key, const String& value) {
        String jsonString = "{\""+ key + "\": \"" + value + "\"}";
        Send(jsonString);
    }

    void SerialJsonComm::Send(const String& string) {
        while (Serial.availableForWrite() < 1) { delay(10); }
        Serial.write(G_ENQ);
        while ((char)Serial.read() != G_ACK) {delay(10);}
        size_t bytesTransmitted = string.length() * sizeof(char);
        String messageOut = G_SOH + bytesTransmitted + G_SOX + string + G_ETX + G_EOT;
        Serial.write(messageOut.c_str());
    }

    void SerialJsonComm::Send(DeserializationError error) {
        String jsonError = "{\"Error\": \"" + String(error.c_str()) + "\"}";
        Send(jsonError);
    }

    void SerialJsonComm::Send(const DynamicJsonDocument& payload) {
        String jsonString;
        serializeJson(payload, jsonString);
        Send(jsonString);
    }

    bool SerialJsonComm::Recieve(DynamicJsonDocument& jsonOut) {
        while ((char)Serial.read() != G_ENQ) { delay(10); }
        Serial.write(G_ACK);
        while ((char)Serial.read() != G_SOH) { delay(10); }
        String payloadSize = "";
        while (true) {
            if (Serial.available() > 0) {
                char readIn = 0;
                readIn = Serial.read();
                if (readIn == G_SOX) {
                    break;
                } else {
                    payloadSize += readIn;
                }
            }
        }
        size_t len = payloadSize.toInt();
        if (len == 0) {
            m_out = "len equals 0";
            return false;
        }
        int errorChar = 0;
        int dataRecv = 0;
        String data = "";
        do {
            if (Serial.available() > 0) {
                int input = Serial.read();
                if (input == G_ETX) {
                    break;
                } else { 
                    if(input > 31 && input < 127) {
                        data += char(input);
                    } else {
                        ++errorChar;
                    }
                    ++dataRecv;
                }
            }
        } while (dataRecv < len);
        jsonOut = DynamicJsonDocument(len);
        DeserializationError error = deserializeJson(jsonOut, data);
        if (error != DeserializationError::Ok) {
            m_out = "Deserialization Error: " + String(error.c_str());
            return false;
        } else if(jsonOut.overflowed()) {
            m_out = "JSON overflow Error: " +String(jsonOut.overflowed());
            return false;
        } else if (jsonOut.isNull()) {
            m_out = "JSON null Error: " + String(jsonOut.isNull());
            return false;
        }
        while (true) {
            if (Serial.available() > 0) {
                char input = Serial.read();
                if (input == G_ETX) {
                    continue;
                } else if (input == G_EOT) {
                    break;
                    
                } else {
                    m_out = "data still in serial buffer";
                    return false;
                }
            }
        }
        return true;
    }
}