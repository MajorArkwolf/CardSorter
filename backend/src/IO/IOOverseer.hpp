#pragma once
#include "Sensor/Sensor.hpp"
#include "../Array.hpp"
#include "IOManager.hpp"


namespace IO {
    class IOOverseer {
        IOOverseer();
        ~IOOverseer();

    private:
        uint8_t m_idUniqueDistributor;
        Container::Array<Sensor*> m_sensorMapping;
    };
}