#include "IPlugin.h"

class Adafruit_PWMServoDriver;

namespace Ark {

    class PCA9685 : public IPlugin {
        public:
            PCA9685(byte argc, byte *argv);
            ~PCA9685() override;
            bool Write(byte argc, byte *argv) override;
            byte* Read(byte argc, byte *argv) override;

        private:
            Adafruit_PWMServoDriver *m_controller = nullptr;
    };

}