#include "PixelLight.hpp"
#include <Adafruit_NeoPixel.h>
#include <math.h>

namespace IO {
    PixelLight::PixelLight(int id, const PixelLightData& data) : 
    IPixelLight(id),
    m_pin(data.Pin),
    m_numberOfPixels(data.NumberOfPixels) {
        m_pixels = nullptr;
    }

    void PixelLight::Setup() {
        m_pixels = new Adafruit_NeoPixel(m_numberOfPixels, m_pin, NEO_GRB + NEO_KHZ800);
        m_pixels->begin();
    }

    void PixelLight::SetColor(const Shared::Color& color) {
        m_color = color;
    }

    void PixelLight::Show() {
        if (m_pixels != nullptr) {
            auto color = m_pixels->Color(m_color.red, m_color.green, m_color.blue);
            auto outputColor = m_pixels->gamma32(m_pixels->ColorHSV(color, m_sat, m_brightness));
            m_pixels->fill(color, 0, 0);
            m_pixels->show();
        }
    }

    SensorMessageResponse HandleMessage(const SensorMessage& message) {
        auto response = SensorMessageResponse();
        switch (message.method.pixelLightMethod)
        {
        case PixelLightMethods::SetColor:
            SetColor(message.data.color);
            break;
        case PixelLightMethods::Show:
            Show();
            response.wasSuccessful = true;
            break;
        default:
            break;
        }
        return response;
    }
}