#pragma once

namespace Nonsense {
    typedef struct Input {
        unsigned int data;
    } Input;

    // Returns 42 if question.data is between 16 and 65536
    unsigned int Ask(const Input &question);

    // Returns always 0
    unsigned int Unused();
}

namespace Math
{
    class Arithmetic
    {
    public:
        // Returns a + b
        static double Add(double a, double b);

        // Returns a - b
        static double Subtract(double a, double b);

        // Returns a * b
        static double Multiply(double a, double b);

        // Returns a / b
        static double Divide(double a, double b);
    };
}