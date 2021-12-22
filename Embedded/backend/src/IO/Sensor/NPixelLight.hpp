#pragma once
#include "../../Color.hpp"
#include "Sensor.hpp"
#include "IPixelLight.hpp"
#include "NetworkSensorInterface.hpp"

namespace IO {
    class NPixelLight : public IPixelLight {
    public:
        NPixelLight(int id, NetworkSensorInterface network);
        void SetColor(const Shared::Color& color) override;
        void Show();
        SensorMessageResponse HandleMessage(const SensorMessage& message) override;
    private:
        NetworkSensorInterface m_network;
        int m_boardAddress;
    };
}