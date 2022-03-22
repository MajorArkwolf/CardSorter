#include "TripWire.h"
#include <Arduino.h>

namespace Sensor {
    TripWire::TripWire(int pin) {
        pinMode(pin, INPUT_PULLUP);
        m_lastState = digitalRead(m_pin);
        m_hasTripped = !m_lastState;
        m_pin = pin;
    }

    bool TripWire::HasTripped() {
        return m_hasTripped;
    }

    void TripWire::Trigger() 
    {
        bool sensorState = digitalRead(m_pin);
        
        if (sensorState && !m_lastState) {
            m_hasTripped = false;
        } 
        if (!sensorState && m_lastState) {
            m_hasTripped = true;
        }
        m_lastState = sensorState;
    }
}