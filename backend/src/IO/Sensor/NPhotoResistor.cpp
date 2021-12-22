#include "NPhotoResistor.hpp"

namespace IO {
    NPhotoResistor::NPhotoResistor(int id, NetworkSensorInterface network) : IPhotoResistor(id), m_network(network) {

    }

    int NPhotoResistor::Get() {
        auto messageOut = SensorMessage(
            m_id,
            GetSensorType(),
            Method(PhotoResistorMethods::Get),
            SensorDataTypes()
        );
        // Send Message
        auto response = m_network.SendAndRecieveMessage(messageOut);
        if (response.wasSuccessful) {
            return response.data.integar;
        }
        return -1;
    }

    SensorMessageResponse NPhotoResistor::HandleMessage(const SensorMessage& message) {
        return SensorMessageResponse();
    }
}