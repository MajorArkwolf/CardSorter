#include "ServoMotor.hpp"
#include <Servo.h>

namespace IO {
    ServoMotor::ServoMotor(int id, const ServoMotorData& data) :
    IServoMotor(id) {
        m_pin = data.Pin;
    }

    ServoMotor::~ServoMotor() {
        delete(m_servo);
    }

    void ServoMotor::Setup() {
        m_servo = new Servo();
        m_servo->attach(pin);
    }

    void ServoMotor::SetPosition(int degrees) {
        m_servo->write(degrees);
    }

    int ServoMotor::GetLastPosition() {
        return m_servo->read();
    }
}