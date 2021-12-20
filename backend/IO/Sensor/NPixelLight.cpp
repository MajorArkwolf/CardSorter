#include "NPixelLight.hpp"

namespace IO {
    NPixelLight::NPixelLight(int id, int boardAddress) : IPixelLight(id), m_boardAddress(boardAddress) {}

    void NPixelLight::SetColor(const Shared::Color& color) {
        auto messageOut = SensorMessage(
            m_id,
            GetSensorType(),
            Method(PixelLightMethods::SetColor),
            SensorDataTypes(color)
        );

        // Send Message
        auto response = SensorMessageResponse();
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
        // Send Message
        auto response = SensorMessageResponse();
        if (!response.wasSuccessful) {
            //report error
        }
    }

    SensorMessageResponse NPixelLight::HandleMessage(const SensorMessage& message) {
        return SensorMessageResponse();
    }
}