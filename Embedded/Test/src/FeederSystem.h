#pragma once
#include "TripWire.h"
#include "MotorController.h"
#include "Response.h"

namespace Controller {
    class FeederSystem {
    public:
        FeederSystem(Sensor::TripWire tripwire, Sensor::MotorController motorController);
        void Trigger();
        Operation::Result FeedCard();
        void StopFeeding();
        const Sensor::TripWire& GetTripWire() const;
        const Sensor::MotorController& GetMotorController() const;

    private:
        Sensor::TripWire m_tripwire;
        Sensor::MotorController m_motorController;
    };
}