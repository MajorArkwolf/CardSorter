#pragma once
#include "../Array.hpp"
#include "Sensor/Sensor.hpp"
#include "../Message.hpp"
#include "IOFactory.hpp"
#include "Comms/i2c.hpp"
#include "../System.hpp"

namespace IO {
    class IOManager {
    public:
        IOManager(int thisBoardID);
        void Setup();
        ~IOManager();
        void Reset();
        void Update();
        MessageProtocol::Message HandleMessage(const MessageProtocol::Message& messageIn);
    private:
        MessageProtocol::Message CreateSensorRequest(const MessageProtocol::Message& messageIn);
        MessageProtocol::Message SensorInstructionRequest(const MessageProtocol::Message& messageIn);
        MessageProtocol::Message GenerateHeartBeat();

        Comm::i2cPassenger i2c;
        Container::Array<Sensor*> m_sensorArray;
        System::BoardStatus m_systemStatus;
    };
}