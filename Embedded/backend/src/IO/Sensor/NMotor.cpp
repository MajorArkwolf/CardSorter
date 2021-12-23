#include "NMotor.hpp"

namespace IO {

    NMotor::NMotor(int id, NetworkSensorInterface network) : IMotor(id), m_network(network) {}

    NMotor::~NMotor() {

    }

    void NMotor::Setup() {

    }

    void NMotor::Set(bool turnOn) {
        auto messageOut = SensorMessage(
            m_id,
            GetSensorType(),
            Method(MotorMethods::Set),
            SensorDataTypes(turnOn)
        );
        // Send Message
        auto response = m_network.SendAndRecieveMessage(messageOut);
        if (!response.wasSuccessful) {
            // log
        }
    }

    void NMotor::SetOnTimer(int nanoSeconds) {
        auto messageOut = SensorMessage(
            m_id,
            GetSensorType(),
            Method(MotorMethods::SetOnTimer),
            SensorDataTypes(nanoSeconds)
        );
        // Send Message
        auto response = m_network.SendAndRecieveMessage(messageOut);
        if (!response.wasSuccessful) {
            // log
        }
    }

    bool NMotor::Get() {
        auto messageOut = SensorMessage(
            m_id,
            GetSensorType(),
            Method(MotorMethods::Get),
            SensorDataTypes()
        );
        // Send Message
        auto response = m_network.SendAndRecieveMessage(messageOut);
        if (!response.wasSuccessful) {
            // log
        }
        return true;
    }
    
    SensorMessageResponse NMotor::HandleMessage(const SensorMessage& message) {
        return SensorMessageResponse();
    }
}