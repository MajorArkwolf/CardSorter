#pragma once
#include "Sensor.hpp"

namespace IO {

    class IServoMotor : public Sensor {
    public:
        IServoMotor(int id) : Sensor(id) {};
        virtual ~IServoMotor() {}
        virtual void Set(int degrees) = 0;
        virtual int Get() = 0;
        Definition::SensorType GetSensorType() override { return Definition::SensorType::ServoMotor; }
    };
}