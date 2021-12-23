#pragma once

#include "IServoMotor.hpp"

class Servo;

namespace IO {
    
    class ServoMotor : public IServoMotor {
    public:
        ServoMotor(int id, const MotorData& data);
        ~ServoMotor() override;
        void Setup() override;
        void Set(int degrees) override;
        int Get() override;
        SensorMessageResponse HandleMessage(const SensorMessage& message) override;
    protected:
        int m_pin;
        Servo* m_servo;    
    };
}