#include <iostream>
#include "mighty.h"

using namespace Nonsense;
using namespace Math;

int main() {
    std::cout << "before" << std::endl;
    Input question = {128};
    auto val = Ask(question);
    std::cout << val << std::endl;
    std::cout << "and after" << std::endl;

    auto a = 7.4;
    auto b = 99;

    std::cout << "a + b = " << Arithmetic::Add(a, b) << std::endl;
    std::cout << "a - b = " << Arithmetic::Subtract(a, b) << std::endl;
    std::cout << "a * b = " << Arithmetic::Multiply(a, b) << std::endl;
    std::cout << "a / b = " << Arithmetic::Divide(a, b) << std::endl;
}
