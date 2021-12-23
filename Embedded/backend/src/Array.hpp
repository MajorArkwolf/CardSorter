#pragma once
#define MAX_ARRAY_INDEX 20

namespace Container {
    template <typename T>
    class Array {
    public:
        Array() {
            m_size = 0;
        }

        bool Append(T sensor) {
            if (m_size < MAX_ARRAY_INDEX) {
                m_array[m_size] = sensor;
                ++m_size;
                return true;
            }
            return false;
        }

        bool Insert(unsigned index, T sensor) {
            if (index < MAX_ARRAY_INDEX || index < m_size) {
                m_array[index] = sensor;
                return true; 
            }
            return false;
        }

        const T& Get(unsigned index) const {
            if (index < m_size) {
                return m_array[index];
            }
            return m_array[0];
        }

        unsigned GetSize() const { return m_size; }
    private:
        T m_array[MAX_ARRAY_INDEX];
        unsigned m_size;
    };
}