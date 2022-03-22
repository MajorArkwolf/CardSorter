#include "PCA9685.h"
#include <Adafruit_PWMServoDriver.h>

namespace {
    static constexpr uint32_t OscillatorFrequency = 27000000;
    static constexpr float ServoFrequency = 50;
}

namespace Ark {
    PCA9685::PCA9685(byte argc, byte *argv) {
        m_controller = new Adafruit_PWMServoDriver();
        if (m_controller != nullptr) {
            m_controller->setOscillatorFrequency(OscillatorFrequency);
            m_controller->setPWMFreq(ServoFrequency);
        }
    }

    PCA9685::~PCA9685() {
        delete (m_controller);
    }

    bool PCA9685::Write(byte argc, byte *argv) {

    }

    byte* PCA9685::Read(byte argc, byte *argv) {

    }
}