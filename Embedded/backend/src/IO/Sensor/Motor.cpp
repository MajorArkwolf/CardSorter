#include "Motor.hpp"

namespace IO {
    Motor::Motor(int id, const MotorData& data) : IMotor(id)  {
        m_pin = data.Pin;
    }

    Motor::~Motor() {
        pinMode(m_pin, INPUT);
    }

    void Motor::Setup() {
        pinMode(m_pin, OUTPUT);
    }

    void Motor::Set(bool turnOn) {
        if (turnOn) {
            digitalWrite(m_pin, HIGH);
        } else {
            digitalWrite(m_pin, LOW);
        }
        m_isTurnedOn = turnOn;
    }

    void Motor::SetOnTimer(int nanoSeconds) {
        digitalWrite(m_pin, HIGH);
        // TODO: set a timer
        m_isTurnedOn = true;
    }

    bool Motor::Get() {
        return m_isTurnedOn;
    }

    SensorMessageResponse Motor::HandleMessage(const SensorMessage& message) {
        auto response = SensorMessageResponse();
        switch (message.method.motorMethods)
        {
        case MotorMethods::Get:
            response.data.integar = Get();
            response.wasSuccessful = true;
            break;
        case MotorMethods::Set:
            Set(message.data.integar);
            response.wasSuccessful = true;
            break;
        case MotorMethods::SetOnTimer:
            SetOnTimer(response.data.integar);
            response.wasSuccessful = true;
            break;
        default:
            break;
        }
        return response;
    }
}