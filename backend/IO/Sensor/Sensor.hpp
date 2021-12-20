#pragma once
#include "../../Message.hpp"
#include "SensorDefinitions.hpp"
#include "SensorMessage.hpp"

namespace IO {

    struct NSensor {
        int BoardID;
        NSensor() : BoardID(0) {}
        NSensor(int boardID) : BoardID(boardID) {}
    };

    class Sensor {
    public:
        Sensor(int id) : m_id(id) {}
        virtual ~Sensor() {};
        virtual Definition::SensorType GetSensorType() = 0;
        virtual void Setup() {}
        virtual int GetID() { return m_id; }
        virtual SensorMessageResponse HandleMessage(const SensorMessage& message) = 0; 
    protected:
        int m_id;
    };
}