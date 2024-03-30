#include <list>
#include <cmath>
#include "Term.hpp"

class Formula : public std::list<Term>
{
private:

public:
    void addTerm(double coeff, int exp);
    double evaluate(double x) const;

    Formula operator+(const Formula& other) const;
};
