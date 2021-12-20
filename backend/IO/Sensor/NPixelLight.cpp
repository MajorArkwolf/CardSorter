#include "NPixelLight.hpp"

namespace IO {
    NPixelLight::NPixelLight(int id, int boardAddress) : IPixelLight(id), m_boardAddress(boardAddress) {}
    void NPixelLight::SetColor(const Shared::Color& color) {}
    void NPixelLight::Show() {}

    SensorMessageResponse NPixelLight::HandleMessage(const SensorMessage& message) {
        return SensorMessageResponse();
    }
}