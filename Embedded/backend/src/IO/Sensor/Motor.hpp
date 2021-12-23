#pragma once
#include "IMotor.hpp"

namespace IO {
    
    class Motor : public IMotor {
    public:
        Motor(int id, const MotorData& data);
        ~Motor() override;
        void Setup() override;
        void Set(bool turnOn) override;
        void SetOnTimer(int nanoSeconds) override;
        bool Get() override;
        SensorMessageResponse HandleMessage(const SensorMessage& message) override;
    protected:
        int m_pin;
        bool m_isTurnedOn;
    };
}