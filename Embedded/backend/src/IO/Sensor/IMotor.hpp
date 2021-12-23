#pragma once
#include "Sensor.hpp"

namespace IO {

    class IMotor : public Sensor {
    public:
        IMotor(int id) : Sensor(id) {};
        virtual ~IMotor() {}
        virtual void Set(bool turnOn) = 0;
        virtual void SetOnTimer(int nanoSeconds) = 0;
        virtual bool Get() = 0;
        Definition::SensorType GetSensorType() const override { return Definition::SensorType::Motor; }
    };
}