#include <catch2/catch_test_macros.hpp>
#include "../backend/src/Message.hpp"
#include "../backend/src/IO/Sensor/SensorMessage.hpp"
#include "../backend/src/IO/Sensor/NetworkSensorInterface.hpp"
#include "../backend/src/IO/Sensor/NPhotoResistor.hpp"
#include "MockObjects/MockComm.hpp"
#include "MockObjects/MockPhotoResistor.hpp"
#include <cstring>

TEST_CASE("Sending a message from NPhotoResistor to a Mock Object and back") {
    auto mockComm = MockObject::MockComm();
    auto networkInterface = IO::NetworkSensorInterface(1, &mockComm);
    auto networkSensor = IO::NPhotoResistor(2, networkInterface);
    auto mockPhotoResistor = MockObject::MockPhotoResistor(2);
    auto message = IO::SensorMessage();
    auto messageResponse = IO::SensorMessageResponse();

    WHEN("NPhotoResistor requests and updated") {
        int8_t sensorValue = mockPhotoResistor.Get();
        auto expectedReturn = IO::SensorMessageResponse(true, IO::SensorDataTypes(sensorValue));
        auto expectedMessageReturned = MessageProtocol::Message(MessageProtocol::MessageType::Acknowledge, MessageProtocol::GenericMessageToBytes<IO::SensorMessageResponse>(expectedReturn));
        mockComm.m_recvPayload = expectedMessageReturned.MessageToBytes();
        auto returnValue = networkSensor.Get();
        THEN("Given an expected return message, the following request should yield the same result.") {
            REQUIRE(returnValue == sensorValue);
        }
        WHEN("Using the previous request validating that the MockObject responds accordingly") {
            auto messageIn = MessageProtocol::Message::BytesToMessage(mockComm.m_sentPayload);
            THEN("The request should contain a payload") {
                REQUIRE(messageIn.GetNumberOfBytes() > 0);
                REQUIRE(messageIn.GetData() != nullptr);
            }
            auto messagePayload = MessageProtocol::BytesToGenericMessage<IO::SensorMessage>(messageIn.GetData());
            auto response = mockPhotoResistor.HandleMessage(messagePayload);
            THEN("The expected and actual response values should match") {
                REQUIRE(response.wasSuccessful == expectedReturn.wasSuccessful);
                REQUIRE(response.data.integar == expectedReturn.data.integar);
            }
        }
    }
}