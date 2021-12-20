#include "NPhotoResistor.hpp"

namespace IO {
    NPhotoResistor::NPhotoResistor(int id, int boardAddress) : IPhotoResistor(id), m_boardAddress(boardAddress) {

    }

    int NPhotoResistor::Get() {
        auto messageOut = SensorMessage(
            m_id,
            GetSensorType(),
            Method(PhotoResistorMethods::Get),
            SensorDataTypes()
        );
        // Send Message
        auto response = SensorMessageResponse();
        if (response.wasSuccessful) {
            return response.data.integar;
        }
        return -1;
    }

    SensorMessageResponse NPhotoResistor::HandleMessage(const SensorMessage& message) {
        return SensorMessageResponse();
    }
}