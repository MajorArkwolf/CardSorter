#pragma once
#include <ArduinoJson.h>
#include <StreamUtils.h>

namespace Comm {
    class SerialJsonComm {
    public:
        SerialJsonComm();
        ~SerialJsonComm();
        void Connect();
        bool SerialDataPending();
        void Send(const DynamicJsonDocument& payload);
        void Send(const String& key, const String& value);
        bool Recieve(DynamicJsonDocument& jsonOut);
        String m_out;
    private:
        void Send(const String& string);
        void Send(DeserializationError error);
    };
}