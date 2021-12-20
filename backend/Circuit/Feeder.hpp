#pragma once
#include "ICircuit.hpp"

namespace Circuit {
    class Feeder : public ICircuit {
    public:
        Feeder();
        ~Feeder() override {}
    private:
        //IO::Motor m_feedermotor;
        //IO::PhotoResistor m_detector

    };
}