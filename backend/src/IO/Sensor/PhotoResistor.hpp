#pragma once
#include "IPhotoResistor.hpp"

namespace IO {
    class PhotoResistor : public IPhotoResistor {
    public:
        PhotoResistor(int id, const PhotoResitorData& data);
        ~PhotoResistor() override {};
        int Get() override;
        void Setup() override;
        SensorMessageResponse HandleMessage(const SensorMessage& message) override;
    private:
        int m_pin;
    };
}