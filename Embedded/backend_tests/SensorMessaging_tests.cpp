#include <catch2/catch_test_macros.hpp>
#include "../backend/src/Message.hpp"
#include "../backend/src/IO/Sensor/SensorMessage.hpp"
#include <cstring>

TEST_CASE("Sensor Messages Conversion", "[single-file]") {
    GIVEN("A SensorMessage Payload") {
        int ID = 5;
        auto sensorType = IO::Definition::SensorType::PhotoResistor;
        auto method = IO::Method(IO::PhotoResistorMethods::Get);
        auto data = IO::SensorDataTypes();
        auto payLoad = IO::SensorMessage(ID, sensorType, method, data);
        auto payLoadSize = sizeof(payLoad);
        WHEN("Converted to a bytestream.") {
            auto payLoadByteStream = MessageProtocol::GenericMessageToBytes<IO::SensorMessage>(payLoad);
            THEN("The bytestream should not be null and the size should be correct.") {
                REQUIRE(payLoadByteStream.GetByteStream() != nullptr);
                REQUIRE(payLoadByteStream.GetNumberOfBytes() == payLoadSize);
            }

            WHEN("Adding the bytestream to a message out") {
                auto messageType = MessageProtocol::MessageType::Acknowledge;
                auto messageOut = MessageProtocol::Message(messageType, payLoadByteStream);
                THEN("The data should be packed in accordingly.") {
                    REQUIRE(messageOut.GetNumberOfBytes() == payLoadSize);
                    REQUIRE(messageOut.GetData());
                }
                WHEN ("Converting the message into its own MessageByteStream") {
                    auto byteStream = messageOut.MessageToBytes();
                    auto totalSize = MessageProtocol::MessageSize + payLoadSize;
                    THEN("The total size should equal Message + SensorMessage size") {
                        REQUIRE(byteStream.GetNumberOfBytes() == totalSize);
                    } AND_WHEN("We convert it back to a message.") {
                        auto convertedBackMsg = MessageProtocol::Message::BytesToMessage(byteStream);
                        THEN("Message should be identical") {
                            REQUIRE(convertedBackMsg.GetMessageType() == messageType);
                            REQUIRE(convertedBackMsg.GetNumberOfBytes() == messageOut.GetNumberOfBytes());
                        } AND_THEN("When converted back into SensorMessage, it too should be identical") {
                            auto convertedPayLoad = MessageProtocol::BytesToGenericMessage<IO::SensorMessage>(convertedBackMsg.GetData());
                            REQUIRE(convertedPayLoad.sensorID == payLoad.sensorID);
                            REQUIRE(convertedPayLoad.type == payLoad.type);
                            REQUIRE(std::memcmp(&convertedPayLoad.method, &payLoad.method, sizeof(IO::SensorMessage)) == 0);
                            REQUIRE(std::memcmp(&convertedPayLoad.data, &payLoad.data, sizeof(IO::SensorMessage)) == 0);
                        }
                    }
                }
            }
        }
    }
}