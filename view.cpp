#include "view.hpp"

std::ostream& operator<<(const std::ostream os, const Formula formula)
{
    for (auto it = formula.begin(); it != formula.end(); it++) {
        std::cout << it->getCoefficient() << "x^" << it->getExponent();
        if (std::next(it) != formula.end()) {
            std::cout << " + ";
        }
    }
    std::cout << std::endl;
}
