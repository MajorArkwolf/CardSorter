#pragma once

namespace Sensor {
    class MotorController {
    public:
        enum class Direction
        {
            Disabled,
            Forward,
            Reverse
        };

        MotorController(int enablePin, int forwardPin, int reversePin);
        void ConfigurePins();
        void EnableMotor(bool turnOn);
        void SetMotorState(Direction direction);
        Direction GetMotorState();

    private:
        int m_enablePin;
        int m_forwardPin;
        int m_reversePin;
        Direction m_lastState = Direction::Disabled;
    };
}