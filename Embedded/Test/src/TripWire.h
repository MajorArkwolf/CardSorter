#pragma once
namespace Sensor {
    class TripWire {
        public:
            TripWire(int pin);
            bool HasTripped();
            void Trigger();

        private:
            bool m_lastState = true;
            bool m_hasTripped = false;
            int m_pin = 0;
    };
}