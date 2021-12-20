#pragma once
#include "../../Color.hpp"
#include "Sensor.hpp"

namespace IO {
    
    class IPixelLight : public Sensor {
    public:
        IPixelLight(int id) : Sensor(id) {};
        virtual ~IPixelLight() {}
        virtual void SetColor(const Shared::Color& color) = 0;
        virtual void Show() = 0;
        Definition::SensorType GetSensorType() override { return Definition::SensorType::PixelLight; }
    };
}