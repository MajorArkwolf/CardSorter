#pragma once
#include "IMotor.hpp"
#include "NetworkSensorInterface.hpp"

namespace IO {
    
    class NMotor : public IMotor {
    public:
        NMotor(int id, NetworkSensorInterface network);
        ~NMotor() override;
        void Setup() override;
        void Set(bool turnOn) override;
        void SetOnTimer(int nanoSeconds) override;
        bool Get() override;
        SensorMessageResponse HandleMessage(const SensorMessage& message) override;
    protected:
        NetworkSensorInterface m_network;
        int m_pin; 
    };
}