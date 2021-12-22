#include "MockPhotoResistor.hpp"

namespace MockObject {
    
    MockPhotoResistor::MockPhotoResistor(int id) : IPhotoResistor(id) {
        m_SetValue = 10;
    }
    int MockPhotoResistor::Get() {
        return m_SetValue;
    }
    void MockPhotoResistor::Setup() {
        m_SetValue = 105;
    }
    IO::SensorMessageResponse MockPhotoResistor::HandleMessage(const IO::SensorMessage& message) {
        auto response = IO::SensorMessageResponse();
        switch (message.method.photoResistorMethod)
        {
        case IO::PhotoResistorMethods::Get:
            response.data.integar = Get();
            response.wasSuccessful = true;
            break;
        default:
            break;
        }
        return response;
    }
}