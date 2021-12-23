#pragma once
#include "ServoMotor.hpp"

namespace IO {
    /**
     * @brief This expands upon a normal ServoMotor with the exception that
     * upon changing its position it gets deattached. This stops jittering
     * due to timing issues. It should be noted that doing so will hamper
     * the servos ability to hold any weight.
     */
    class DeattachedServoMotor : public ServoMotor {
    public:
        DeattachedServoMotor(int id, const MotorData& data);
        void Setup() override;
        void Set(int degrees) override;
        int Get() override;
    private:
        int m_lastValue;
    };
}