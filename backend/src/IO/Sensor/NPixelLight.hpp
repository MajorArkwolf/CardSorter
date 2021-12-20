#pragma once
#include "../../Color.hpp"
#include "Sensor.hpp"
#include "IPixelLight.hpp"

namespace IO {
    class NPixelLight : public IPixelLight {
    public:
        NPixelLight(int id, int boardAddress);
        void SetColor(const Shared::Color& color) override;
        void Show();
        SensorMessageResponse HandleMessage(const SensorMessage& message) override;
    private:
        int m_boardAddress;
    };
}