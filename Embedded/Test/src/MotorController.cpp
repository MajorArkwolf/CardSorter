#include "MotorController.h"
#include <Arduino.h>

namespace Sensor {
    MotorController::MotorController(int enablePin, int forwardPin, int reversePin) : m_enablePin(enablePin), 
    m_forwardPin(forwardPin), 
    m_reversePin(reversePin) {
        ConfigurePins();
    }

    void MotorController::ConfigurePins() {
        pinMode(m_enablePin, OUTPUT);
        pinMode(m_forwardPin, OUTPUT);
        pinMode(m_reversePin, OUTPUT);
        digitalWrite(m_enablePin, HIGH);
    }

    void MotorController::EnableMotor(bool turnOn) {
        digitalWrite(m_enablePin, turnOn);
    }

    void MotorController::SetMotorState(Direction direction) {
        switch (direction)
        {
        case Direction::Disabled:
            digitalWrite(m_forwardPin, LOW);
            digitalWrite(m_reversePin, LOW);
            break;
        case Direction::Forward:
            digitalWrite(m_forwardPin, HIGH);
            digitalWrite(m_reversePin, LOW);
            break;
        case Direction::Reverse:
            digitalWrite(m_forwardPin, LOW);
            digitalWrite(m_reversePin, HIGH);
            break;
        default:
            break;
        }
        m_lastState = direction;
    }

    MotorController::Direction MotorController::GetMotorState() {
        return m_lastState;
    }
}