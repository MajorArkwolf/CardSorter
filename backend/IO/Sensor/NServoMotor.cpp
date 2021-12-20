#include "NServoMotor.hpp"

namespace IO {
    NServoMotor::NServoMotor(int id, int boardAddress) : IServoMotor(id), m_boardAddress(boardAddress) {}
    
    void NServoMotor::Set(int degrees) {
        auto messageOut = SensorMessage(
            m_id,
            GetSensorType(),
            Method(ServoMotorMethods::Set),
            SensorDataTypes((int8_t)degrees)
        );
        // Send Message
        auto response = SensorMessageResponse();
        if (!response.wasSuccessful) {
            // log
        }
    }

    int NServoMotor::Get() {
        auto messageOut = SensorMessage(
            m_id,
            GetSensorType(),
            Method(ServoMotorMethods::Get),
            SensorDataTypes()
        );

        // Send Message
        auto response = SensorMessageResponse();
        if (response.wasSuccessful) {
            return (int)response.data.integar;
        }
        return -1;
    }

    SensorMessageResponse NServoMotor::HandleMessage(const SensorMessage& message) {
        return SensorMessageResponse();
    }
}