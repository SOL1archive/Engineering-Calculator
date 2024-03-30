#include <cmath>

#include "Term.hpp"

Term::Term(double coefficient=0, double exponent=0)
    : coefficient(coefficient)
    , exponent(exponent)
{

}

double Term::getCoefficient() const
{
    return this->coefficient;
}

double Term::getExponent() const 
{
    return this->exponent;
}

double Term::evaluate(double x) const
{
    return this->coefficient * std::pow(x, this->exponent);
}

bool Term::isSameType(const Term& other) const {
    return this->exponent == other.exponent;
}
