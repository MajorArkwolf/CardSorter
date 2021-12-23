#pragma once
#include "Sensor/Sensor.hpp"
#include "../Array.hpp"
#include "IOManager.hpp"
#include "Comms/i2c.hpp"
#include "../System.hpp"
#include "Comms/SerialJsonComm.hpp"

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
        void RegisterSensor(const SensorTemplate& data);

        SensorIDDistributor m_sensorIDDistributor;
        Comm::i2cDriver m_i2c;
        Comm::SerialJsonComm m_serialComm;
        Container::Array<Sensor*> m_sensorMapping;
        System::BoardStatus m_systemStatus;
    };
}