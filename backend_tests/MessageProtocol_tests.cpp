#include <catch2/catch_test_macros.hpp>
#include "../backend/Message.hpp"

TEST_CASE("MessageByteStream Object Tests", "[single-file]") {
    GIVEN( "When a MessageByteStream object is created." ) {
        WHEN("A default version is constructed.") {
            auto defaultMsg = MessageProtocol::MessageByteStream();
            THEN("It should contain default values") {
                REQUIRE(defaultMsg.GetByteStream() == nullptr);
                REQUIRE(defaultMsg.GetNumberOfBytes() == 0);
            }
        }
        WHEN("A overloaded constructor is called with size_t and byte*.") {
            size_t amountToMalloc = 10;
            byte* ptr = (byte*)malloc(amountToMalloc);
            auto alternateMsg = MessageProtocol::MessageByteStream(amountToMalloc, ptr);
            THEN("It should shallow copy these values.") {
                REQUIRE(alternateMsg.GetByteStream() != nullptr);
                REQUIRE(alternateMsg.GetByteStream() == ptr);
                REQUIRE(alternateMsg.GetNumberOfBytes() == amountToMalloc);
            }
            AND_THEN("If we attempt to copy construct said object, it should be deep copied.") {
                
            }
        }
    }
}

TEST_CASE( "MessageProtocol Conversion", "[single-file]" ){
    GIVEN( "A Message with no payload ready to go." ) {
        auto message = MessageProtocol::Message(MessageProtocol::MessageType::CreateSensor, 0, nullptr);
        THEN("Data should be null.") {
            REQUIRE(message.GetData() == nullptr);
            REQUIRE(message.GetNumberOfBytes() == 0);
            REQUIRE(message.GetMessageType() == MessageProtocol::MessageType::CreateSensor);
        }

        WHEN ("MessageToBytes() is called") {
            auto byteArray = message.MessageToBytes();
            THEN ("Should not be null") {
                REQUIRE(byteArray.GetByteStream() != nullptr);
                REQUIRE(byteArray.GetNumberOfBytes() == sizeof(MessageProtocol::Message) - sizeof(byte*));
            }
            AND_WHEN("BytesToMessage() is called to convert back.") {
                auto returnMessage = MessageProtocol::Message::BytesToMessage(byteArray);
                THEN("Returned message should be same as the sent message.") {
                    REQUIRE(returnMessage.GetMessageType() == message.GetMessageType());
                    REQUIRE(returnMessage.GetNumberOfBytes() == message.GetNumberOfBytes());
                    REQUIRE(returnMessage.GetData() == nullptr);
                }
            }
        }
    }
    GIVEN( "A Message with a payload ready to go." ) {
        size_t numberOfBytes = 10;
        auto newData = (byte*)malloc(numberOfBytes);
        auto message = MessageProtocol::Message(MessageProtocol::MessageType::CreateSensor, numberOfBytes, newData);
        
        THEN("Data should not be null.") {
             REQUIRE(message.GetData() != nullptr);
        }
        WHEN ("MessageToBytes() is called") {
            auto byteArray = message.MessageToBytes();
            THEN ("Should not be null") {
                REQUIRE(byteArray.GetByteStream() != nullptr);
            }
            AND_WHEN("BytesToMessage() is called to convert back.") {
                auto returnMessage = MessageProtocol::Message::BytesToMessage(byteArray);
                THEN("Returned message should be same as the sent message.") {
                    REQUIRE(returnMessage.GetMessageType() == message.GetMessageType());
                    REQUIRE(returnMessage.GetNumberOfBytes() == message.GetNumberOfBytes());
                    REQUIRE(*returnMessage.GetData() == *message.GetData());
                }
            }
        }
    }
}