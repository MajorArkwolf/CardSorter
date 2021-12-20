#include "NPhotoResistor.hpp"

namespace IO {
    NPhotoResistor::NPhotoResistor(int id, int boardAddress) : IPhotoResistor(id), m_boardAddress(boardAddress) {

    }

    int NPhotoResistor::Get() {
        return 0;
    }

    SensorMessageResponse NPhotoResistor::HandleMessage(const SensorMessage& message) {
        return SensorMessageResponse();
    }
}