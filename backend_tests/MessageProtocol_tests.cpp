#include <catch2/catch_test_macros.hpp>
#include "../backend/Message.hpp"

TEST_CASE( "MessageProtocol Conversion", "[single-file]" ){
    GIVEN( "A Message with no payload ready to go." ) {
        auto message = MessageProtocol::Message();
        message.Type = MessageProtocol::MessageType::CreateSensor;
        message.NumberOfBytes = 0;
        THEN("Data should be null.") {
             REQUIRE(message.Data == nullptr);
        }

        WHEN ("MessageToBytes() is called") {
            auto byteArray = MessageProtocol::MessageToBytes(message);
            THEN ("Should not be null") {
                REQUIRE(byteArray.byteStream != nullptr);
            }
            AND_WHEN("BytesToMessage() is called to convert back.") {
                auto returnMessage = MessageProtocol::BytesToMessage(byteArray.byteStream);
                THEN("Returned message should be same as the sent message.") {
                    REQUIRE(returnMessage.Type == message.Type);
                    REQUIRE(returnMessage.NumberOfBytes == message.NumberOfBytes);
                    REQUIRE(returnMessage.Data == nullptr);
                }
                delete(returnMessage.Data);
            }
            delete(byteArray.byteStream);
        }
    }
    GIVEN( "A Message with a payload ready to go." ) {
        auto message = MessageProtocol::Message();
        message.Type = MessageProtocol::MessageType::CreateSensor;
        message.NumberOfBytes = 10;
        size_t numberOfBytes = sizeof(byte) * 10u;
        message.Data = (byte*)malloc(numberOfBytes);
        THEN("Data should not be null.") {
             REQUIRE(message.Data != nullptr);
        }
        WHEN ("MessageToBytes() is called") {
            auto byteArray = MessageProtocol::MessageToBytes(message);
            THEN ("Should not be null") {
                REQUIRE(byteArray.byteStream != nullptr);
            }
            AND_WHEN("BytesToMessage() is called to convert back.") {
                auto returnMessage = MessageProtocol::BytesToMessage(byteArray.byteStream);
                THEN("Returned message should be same as the sent message.") {
                    REQUIRE(returnMessage.Type == message.Type);
                    REQUIRE(returnMessage.NumberOfBytes == message.NumberOfBytes);
                    REQUIRE(*returnMessage.Data == *message.Data);
                }
                delete(returnMessage.Data);
            }
            delete(byteArray.byteStream);
        }
    }
}