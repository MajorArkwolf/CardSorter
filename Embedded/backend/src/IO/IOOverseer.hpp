#pragma once
#include "Sensor/Sensor.hpp"
#include "../Array.hpp"
#include "IOManager.hpp"
#include "Comms/i2c.hpp"
#include "../System.hpp"

namespace IO {
    class IOOverseer {
        class SensorIDDistributor {
        private:
            IO::SensorID m_idUniqueDistributor;
        public:
            SensorIDDistributor() : m_idUniqueDistributor(0) {}
            IO::SensorID GetNewUniqueID() { return m_idUniqueDistributor++; }
        };

    public:
        IOOverseer();
        ~IOOverseer();
        void Setup();
        void Update();

    private:
        void RegisterSensor(int boardAddress, Definition::SensorType type, SensorInitData data);

        SensorIDDistributor m_sensorIDDistributor;
        Comm::i2cDriver i2c;
        Container::Array<Sensor*> m_sensorMapping;
        System::BoardStatus m_systemStatus;
    };
}