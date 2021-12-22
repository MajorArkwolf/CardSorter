#pragma once
#include "IServoMotor.hpp"
#include "NetworkSensorInterface.hpp"

namespace IO {
    class NServoMotor : public IServoMotor {
    public:
        NServoMotor(int id, NetworkSensorInterface network);
        ~NServoMotor() override {} 
        void Set(int degrees) override;
        int Get() override;
        SensorMessageResponse HandleMessage(const SensorMessage& message) override;
    private:
        NetworkSensorInterface m_network;
        int m_boardAddress;
    };
}