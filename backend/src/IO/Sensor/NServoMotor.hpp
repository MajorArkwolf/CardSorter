#pragma once
#include "IServoMotor.hpp"

namespace IO {
    class NServoMotor : public IServoMotor {
    public:
        NServoMotor(int id, int boardAddress);
        ~NServoMotor() override {} 
        void Set(int degrees) override;
        int Get() override;
        SensorMessageResponse HandleMessage(const SensorMessage& message) override;
    private:
        int m_boardAddress;
    };
}