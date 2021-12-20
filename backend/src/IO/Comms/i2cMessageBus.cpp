#include "i2cMessageBus.hpp"
namespace Comm {
    i2cMessageBus::i2cMessageBus() :
        m_recv(),
        m_send(),
        m_holdingRecieveMessage(false),
        m_holdingSendMessage(false)
        {}

    bool i2cMessageBus::HoldingRecieveMessage() {
        return m_holdingRecieveMessage;
    }

    bool i2cMessageBus::HoldingSendMessage() {
        return m_holdingSendMessage;
    }

    bool i2cMessageBus::GetRecvMessage(MessageProtocol::MessageByteStream& out) {
        if (m_holdingRecieveMessage) {
            MessageProtocol::MessageByteStream::Swap(m_recv, out);
            m_holdingRecieveMessage = false;
            return true;
        }
        return false;
    }

    bool i2cMessageBus::GetSendMessage(MessageProtocol::MessageByteStream& out) {
        if (m_holdingSendMessage) {
            MessageProtocol::MessageByteStream::Swap(m_send, out);
            m_holdingSendMessage = false;
            return true;
        }
        return false;
    }

    bool i2cMessageBus::SetRecvMessage(MessageProtocol::MessageByteStream& in) {
        if (!m_holdingRecieveMessage) {
            MessageProtocol::MessageByteStream::Swap(m_recv, in);
            m_holdingRecieveMessage = true;
            return true;
        }
        return false;
    }

    bool i2cMessageBus::SetSendMessage(MessageProtocol::MessageByteStream& in) {
        if (!m_holdingSendMessage) {
            MessageProtocol::MessageByteStream::Swap(m_send, in);
            m_holdingSendMessage = true;
            return true;
        }
        return false;
    }
}