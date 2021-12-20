#pragma once
#include "IPhotoResistor.hpp"

namespace IO {
    class NPhotoResistor : public IPhotoResistor {
    public:
        NPhotoResistor(int id, int boardAddress);
        ~NPhotoResistor() override {};
        int Get() override;
        SensorMessageResponse HandleMessage(const SensorMessage& message) override;
    private:
        int m_id;
        int m_boardAddress;
    };
}