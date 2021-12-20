#pragma once

#include "IServoMotor.hpp"

class Servo;

namespace IO {
    
    class ServoMotor : public IServoMotor {
    public:
        ServoMotor(int id, const ServoMotorData& data);
        ~ServoMotor();
        void Setup() override;
        void Set(int degrees) override;
        int Get() override;
        SensorMessageResponse HandleMessage(const SensorMessage& message) override;
    private:
        Servo* m_servo;
        int m_pin;
    };
}