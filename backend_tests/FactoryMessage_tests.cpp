#include <catch2/catch_test_macros.hpp>
#include "../backend/Message.hpp"
#include "../backend/IO/IOFactory.hpp"


TEST_CASE( "FactoryMessage Conversion", "[single-file]" ) {
    GIVEN( "A factory message ready to generate a sensor." ) {
        auto message = IO::Factory::FactoryMessage();
        message.Location = IO::Factory::SensorLocation::Network;
        message.Type = IO::Definition::SensorType::PhotoResistor;
        auto nSensorData = IO::NSensor(0);
        message.Data.networkSensor = nSensorData;
        WHEN("FactoryMessageToBytes() is called") {
            auto messageStream = MessageProtocol::GenericMessageToBytes<IO::Factory::FactoryMessage>(message);
            THEN("The returned object should not be nullptr") {
                REQUIRE(messageStream.byteStream != nullptr);
            }
            AND_THEN("The amount of bytes should be equal to the orginal object") {
                REQUIRE(messageStream.numberOfBytes == sizeof(IO::Factory::FactoryMessage));
            }
            WHEN("We call BytesToFactoryMessage() and convert it back...") {
                auto convertedMessage = MessageProtocol::BytesToGenericMessage<IO::Factory::FactoryMessage>(messageStream.byteStream);
                THEN("The new message should contain the same contents as the old message") {
                    REQUIRE(message.Location == convertedMessage.Location);
                    REQUIRE(message.Type == convertedMessage.Type);
                    REQUIRE(message.ID == convertedMessage.ID);
                    REQUIRE(message.Data.networkSensor.BoardID == convertedMessage.Data.networkSensor.BoardID);
                }
            }
        }
    }
}