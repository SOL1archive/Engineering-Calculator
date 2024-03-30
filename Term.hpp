
class Term {
protected:
    double coefficient;
    double exponent;

public:
    Term(double coefficient=0, double exponent=0);

    double getCoefficient() const;
    double getExponent() const;
    double evaluate(double x) const;
    bool isSameType(const Term& other) const;
};
