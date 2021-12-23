#pragma once
#include "../../Color.hpp"
#include "IPixelLight.hpp"

class Adafruit_NeoPixel;

namespace IO {
    class PixelLight : public IPixelLight {
    public:
        PixelLight(int id, const PixelLightData& data);
        PixelLight(int pin, int numberOfPixels);
        ~PixelLight() override {};
        void SetColor(const Shared::Color& color) override;
        void Show() override;
        void Setup() override;
        SensorMessageResponse HandleMessage(const SensorMessage& message) override;
    private:
        int m_pin = 0;
        int m_numberOfPixels = 0;
        Shared::Color m_color = {};
        Adafruit_NeoPixel* m_pixels;
    };
}