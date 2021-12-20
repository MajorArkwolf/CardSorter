#include "PhotoResistor.hpp"
#include <pins_arduino.h>
#include <Arduino.h>

namespace IO {
    PhotoResistor::PhotoResistor(int id, const PhotoResitorData& data) :
    IPhotoResistor(id) {
        m_pin = analogInputToDigitalPin(data.Pin);
        
    }

    int PhotoResistor::Get() {
        return analogRead(m_pin);
    }

    void PhotoResistor::Setup() {
        pinMode(m_pin, INPUT);
    }
}