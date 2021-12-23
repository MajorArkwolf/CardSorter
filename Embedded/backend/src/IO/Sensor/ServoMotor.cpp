#include "ServoMotor.hpp"
#include <Servo.h>

namespace IO {
    ServoMotor::ServoMotor(int id, const MotorData& data) :
    IServoMotor(id) {
        m_pin = data.Pin;
    }

    ServoMotor::~ServoMotor() {
        delete(m_servo);
    }

    void ServoMotor::Setup() {
        m_servo = new Servo();
        m_servo->attach(m_pin);
    }

    void ServoMotor::Set(int degrees) {
        m_servo->write(degrees);
    }

    int ServoMotor::Get() {
        return m_servo->read();
    }

    SensorMessageResponse ServoMotor::HandleMessage(const SensorMessage& message) {
        auto response = SensorMessageResponse();
        switch (message.method.servoMotorMethod)
        {
        case ServoMotorMethods::Get:
            response.data.integar = Get();
            response.wasSuccessful = true;
            break;
        case ServoMotorMethods::Set:
            Set(message.data.integar);
            response.wasSuccessful = true;
            break;
        default:
            break;
        }
        return response;
    }
}