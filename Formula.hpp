#include <list>
#include <cmath>
#include "Term.hpp"

class Formula : public std::list<Term>
{
private:

public:
    void addTerm(double coefficient, int exponent);
    double evaluate(double x) const;
    double differential(double x) const;
    double integral(double start, double end) const;

    Formula operator+(const Formula& other) const;
};
