#pragma once
#include "IPhotoResistor.hpp"
#include "NetworkSensorInterface.hpp"

namespace IO {
    class NPhotoResistor : public IPhotoResistor {
    public:
        NPhotoResistor(int id, NetworkSensorInterface network);
        ~NPhotoResistor() override {};
        int Get() override;
        SensorMessageResponse HandleMessage(const SensorMessage& message) override;
    private:
        NetworkSensorInterface m_network;
        int m_id;
    };
}