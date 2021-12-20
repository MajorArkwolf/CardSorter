#pragma once
#include "../Array.hpp"
#include "Sensor/Sensor.hpp"
#include "../Message.hpp"
#include "IOFactory.hpp"

namespace IO {
    class IOManager {
    public:
        IOManager();
        void Setup();
        ~IOManager();
        void Reset();
        MessageProtocol::Message HandleMessage(const MessageProtocol::Message& messageIn);
    private:
        MessageProtocol::Message CreateSensorRequest(const MessageProtocol::Message& messageIn);
        MessageProtocol::Message SensorInstructionRequest(const MessageProtocol::Message& messageIn);
        Container::Array<Sensor*> m_sensorArray;
    };
}