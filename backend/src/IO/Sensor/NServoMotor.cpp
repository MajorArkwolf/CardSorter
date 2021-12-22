#include "NServoMotor.hpp"

namespace IO {
    NServoMotor::NServoMotor(int id, NetworkSensorInterface network) : 
    IServoMotor(id), 
    m_network(network)
    {}
    
    void NServoMotor::Set(int degrees) {
        auto messageOut = SensorMessage(
            m_id,
            GetSensorType(),
            Method(ServoMotorMethods::Set),
            SensorDataTypes((int8_t)degrees)
        );
        // Send Message
        auto response = m_network.SendAndRecieveMessage(messageOut);
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

        auto response = m_network.SendAndRecieveMessage(messageOut);

        if (response.wasSuccessful) {
            return (int)response.data.integar;
        }
        return -1;
    }

    SensorMessageResponse NServoMotor::HandleMessage(const SensorMessage& message) {
        return SensorMessageResponse();
    }
}