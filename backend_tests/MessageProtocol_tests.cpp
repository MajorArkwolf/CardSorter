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
        WHEN("A overloaded constructor/assignment is called.") {
            size_t amountToMalloc = 10;
            byte* ptr = (byte*)malloc(amountToMalloc);
            auto alternateMsg = MessageProtocol::MessageByteStream(amountToMalloc, ptr);
            THEN("Alternative constructor with size_t and byte* is invoked, It should shallow copy these values.") {
                REQUIRE(alternateMsg.GetByteStream() != nullptr);
                REQUIRE(alternateMsg.GetByteStream() == ptr);
                REQUIRE(alternateMsg.GetNumberOfBytes() == amountToMalloc);
            }
            AND_THEN("Copy constructed object from said object should be deep copied.") {
                auto cpyObj = MessageProtocol::MessageByteStream(alternateMsg);
                REQUIRE(cpyObj.GetByteStream() != nullptr);
                REQUIRE(cpyObj.GetByteStream() != ptr);
                REQUIRE(memcmp(cpyObj.GetByteStream(), alternateMsg.GetByteStream(), alternateMsg.GetNumberOfBytes()) == 0);
                REQUIRE(cpyObj.GetNumberOfBytes() == alternateMsg.GetNumberOfBytes());
            }
            AND_THEN("Copy assignment object from said object should be deep copied.") {
                auto cpyAssignedObj = MessageProtocol::MessageByteStream();
                cpyAssignedObj = alternateMsg;
                REQUIRE(cpyAssignedObj.GetByteStream() != nullptr);
                REQUIRE(cpyAssignedObj.GetByteStream() != ptr);
                REQUIRE((memcmp(cpyAssignedObj.GetByteStream(), alternateMsg.GetByteStream(), alternateMsg.GetNumberOfBytes())) == 0);
                REQUIRE(cpyAssignedObj.GetNumberOfBytes() == alternateMsg.GetNumberOfBytes());
            }
        }
    }
}

TEST_CASE("Message Object Tests", "[single-file]") {
    GIVEN( "When a Message object is created." ) {
        
        WHEN("A default version is constructed.") {
            auto defaultMsg = MessageProtocol::Message();
            THEN("It should contain default values") {
                REQUIRE(defaultMsg.GetMessageType() == MessageProtocol::MessageType::Failure);
                REQUIRE(defaultMsg.GetNumberOfBytes() == 0);
                REQUIRE(defaultMsg.GetData() == nullptr);
            }
        }
        WHEN("A overloaded constructor/assignment is called.") {
            size_t amountToMalloc = 10;
            byte* ptr = (byte*)malloc(amountToMalloc);
            auto msgObj = MessageProtocol::Message(MessageProtocol::MessageType::CreateSensor, amountToMalloc, ptr);
            THEN("A shallow copy should be performed") {
                REQUIRE(msgObj.GetMessageType() == MessageProtocol::MessageType::CreateSensor);
                REQUIRE(msgObj.GetNumberOfBytes() == amountToMalloc);
                REQUIRE(msgObj.GetData() == ptr);
            }
            AND_THEN("Copy constructed object from said object should be deep copied.") {
                auto cpyObj = MessageProtocol::Message(msgObj);
                REQUIRE(cpyObj.GetData() != nullptr);
                REQUIRE(cpyObj.GetMessageType() == msgObj.GetMessageType());
                REQUIRE(cpyObj.GetNumberOfBytes() == msgObj.GetNumberOfBytes());
                REQUIRE(cpyObj.GetData() != msgObj.GetData());
                REQUIRE((memcmp(cpyObj.GetData(), msgObj.GetData(), msgObj.GetNumberOfBytes())) == 0);
            }
            AND_THEN("Copy assignment object from said object should be deep copied.") {
                auto cpyObj = MessageProtocol::Message();
                cpyObj = msgObj;
                REQUIRE(cpyObj.GetData() != nullptr);
                REQUIRE(cpyObj.GetMessageType() == msgObj.GetMessageType());
                REQUIRE(cpyObj.GetNumberOfBytes() == msgObj.GetNumberOfBytes());
                REQUIRE(cpyObj.GetData() != msgObj.GetData());
                REQUIRE((memcmp(cpyObj.GetData(), msgObj.GetData(), msgObj.GetNumberOfBytes())) == 0);
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