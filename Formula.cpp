#include "Formula.hpp"

void Formula::addTerm(double coeff, int exp) {
    push_back(Term(coeff, exp));
}

double Formula::evaluate(double x) const {
    double result = 0.0;
    for (const auto& term : *this) {
        result += term.evaluate(x);
    }
    return result;
}

Formula Formula::operator+(const Formula& other) const {
    Formula result;
    auto it1 = begin();
    auto it2 = other.begin();

    while (it1 != end() && it2 != other.end()) {
        if (it1->isSameType(*it2)) {
            result.addTerm(it1->getCoefficient() + it2->getCoefficient(), it1->getExponent());
            ++it1;
            ++it2;
        } else if (it1->getExponent() > it2->getExponent()) {
            result.addTerm(it1->getCoefficient(), it1->getExponent());
            ++it1;
        } else {
            result.addTerm(it2->getCoefficient(), it2->getExponent());
            ++it2;
        }
    }
    
    while (it1 != end()) {
        result.addTerm(it1->getCoefficient(), it1->getExponent());
        ++it1;
    }

    while (it2 != other.end()) {
        result.addTerm(it2->getCoefficient(), it2->getExponent());
        ++it2;
    }
    return result;
}
