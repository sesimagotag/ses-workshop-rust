#include "mighty.h"

namespace Nonsense {
    unsigned int Ask(const Input &question) {
        if (question.data < 65536 && question.data > 16) {
            return 42;
        }

        return Unused();
    }

    unsigned int Unused() {
        return 0;
    }
}

namespace Math
{
    double Arithmetic::Add(double a, double b)
    {
        return a + b;
    }

    double Arithmetic::Subtract(double a, double b)
    {
        return a - b;
    }

    double Arithmetic::Multiply(double a, double b)
    {
        return a * b;
    }

    double Arithmetic::Divide(double a, double b)
    {
        return a / b;
    }
}