#pragma once
#include "Sensor.hpp"

namespace IO {
    class IPhotoResistor : public Sensor {
    public:
        IPhotoResistor(int id) : Sensor(id) {}
        virtual ~IPhotoResistor() {};
        virtual int Get() = 0;
        Definition::SensorType GetSensorType() const override { return Definition::SensorType::PhotoResistor; }
    };
}