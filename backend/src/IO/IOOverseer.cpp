#include "IOOverseer.hpp"


namespace IO {
        IOOverseer::IOOverseer() : m_idUniqueDistributor(0) {
            m_sensorMapping = Container::Array<Sensor*>();
        }
        
        IOOverseer::~IOOverseer() {}
}