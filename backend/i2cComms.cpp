#include "i2cComms.hpp"
#include <Wire.h>

namespace Comms {
        i2c::i2c() {
            m_address = 0;
            m_commType = Comms::CommType::Driver;
        }

        i2c::i2c(int address) {
            m_address = address;
            if ( address != 0 ) {
                m_commType = Comms::CommType::Passenger;
            } else {
                m_commType = Comms::CommType::Driver;
            }
        }

        void i2c::Connect() {
            if (m_commType == Comms::CommType::Driver) {
                Wire.begin();
            } else {
                Wire.begin(m_address);
            }
        }
}