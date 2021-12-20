#include "NServoMotor.hpp"

namespace IO {
    NServoMotor::NServoMotor(int id, int boardAddress) : IServoMotor(id), m_boardAddress(boardAddress) {}
    
    void NServoMotor::Set(int degrees) {

    }

    int NServoMotor::Get() {
        return 0;
    }

    SensorMessageResponse NServoMotor::HandleMessage(const SensorMessage& message) {
        return SensorMessageResponse();
    }
}