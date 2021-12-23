#pragma once
#include <ArduinoJson.h>

namespace Comm {
    class SerialJsonComm {
    public:
        SerialJsonComm();
        ~SerialJsonComm();
        bool SerialDataPending();
        void Send(const DynamicJsonDocument& payload);
        DynamicJsonDocument Recieve();
    };
}