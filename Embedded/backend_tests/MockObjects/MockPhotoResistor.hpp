#pragma once

#include "../../backend/src/IO/Sensor/IPhotoResistor.hpp"

namespace MockObject { 
    class MockPhotoResistor : public IO::IPhotoResistor {
    public:
        MockPhotoResistor(int id);
        ~MockPhotoResistor() override {};
        int Get() override;
        void Setup() override;
        IO::SensorMessageResponse HandleMessage(const IO::SensorMessage& message) override;
    private:
        int m_SetValue;
    };
}