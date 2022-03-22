#include "FeederSystem.h"
#include <Arduino.h>

namespace Controller {
    FeederSystem::FeederSystem(Sensor::TripWire tripwire, Sensor::MotorController motorController) : 
    m_tripwire(tripwire),
    m_motorController(motorController) {
        
    }

    Operation::Result FeederSystem::FeedCard() {
        if (!m_tripwire.HasTripped()) {
            m_motorController.SetMotorState(Sensor::MotorController::Direction::Forward);
            Serial.write("FeedCard: Enabling feeder.\n");
            return Operation::Result::Success;
        } else {
            Serial.write("FeedCard: Beam trepped, not enabling feeder.\n");
            return Operation::Result::Failure;
        }
    }
    
    void FeederSystem::StopFeeding() {
        m_motorController.SetMotorState(Sensor::MotorController::Direction::Disabled);
    }

    void FeederSystem::Trigger() {
        m_tripwire.Trigger();
        if (m_tripwire.HasTripped() && m_motorController.GetMotorState() != Sensor::MotorController::Direction::Disabled) {
            Serial.write("Beam tripped disabling motor.\n");
            StopFeeding();
        }
    }

    const Sensor::TripWire& FeederSystem::GetTripWire() const {
        return m_tripwire;
    }

    const Sensor::MotorController& FeederSystem::GetMotorController() const {
        return m_motorController;
    }
}