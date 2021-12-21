#include "DeattachedServoMotor.hpp"

#include "ServoMotor.hpp"
#include <Servo.h>

namespace {
    /**
     * @brief A helper function based on the time an average stepper motor takes to move X degrees.
     * https://www.arduino.cc/reference/en/libraries/servo/writemicroseconds/
     * @param degrees The degrees to be converted
     * @return int microseconds from 0 to X where x is the degrees required.
     */
    int ConvertDegreesToMS(int degrees) {
        return (degrees * 5.55555) + 1000;
    }
}

namespace IO {
    DeattachedServoMotor::DeattachedServoMotor(int id, const ServoMotorData& data) :
    ServoMotor(id, data), m_lastValue(0) {}

    void DeattachedServoMotor::Setup() {
        m_servo = new Servo();
    }

    void DeattachedServoMotor::Set(int degrees) {
        int mS = abs(ConvertDegreesToMS(m_lastValue) - ConvertDegreesToMS(degrees));
        if (m_servo != nullptr) {
            m_lastValue = degrees;
            m_servo->attach(m_pin);
            m_servo->write(degrees);
            delay(mS);
            m_servo->detach();
        }
    }

    int DeattachedServoMotor::Get() {
        return m_lastValue;
    }
}