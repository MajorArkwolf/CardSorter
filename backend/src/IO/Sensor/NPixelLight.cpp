#include "NPixelLight.hpp"

namespace IO {
    NPixelLight::NPixelLight(int id, NetworkSensorInterface network) : IPixelLight(id), m_network(network) {}

    void NPixelLight::SetColor(const Shared::Color& color) {
        auto messageOut = SensorMessage(
            m_id,
            GetSensorType(),
            Method(PixelLightMethods::SetColor),
            SensorDataTypes(color)
        );
        auto response = m_network.SendAndRecieveMessage(messageOut);
        if (!response.wasSuccessful) {
            //report error
        }
    }
    
    void NPixelLight::Show() {
        auto messageOut = SensorMessage(
            m_id,
            GetSensorType(),
            Method(PixelLightMethods::Show),
            SensorDataTypes()
        );
        auto response = m_network.SendAndRecieveMessage(messageOut);
        if (!response.wasSuccessful) {
            //report error
        }
    }

    SensorMessageResponse NPixelLight::HandleMessage(const SensorMessage& message) {
        return SensorMessageResponse();
    }
}