//
// Created by Grzes on 2020-04-02.
//

#ifndef LAB04_COMPLEX_H
#define LAB04_COMPLEX_H


template<class T>
class Complex {
    T real; T imag;
public:
    Complex(T _real, T _imag) : real(_real), imag(_imag) {};
    Complex(T _real) : real(_real), imag(0) {};
    Complex() : real(0), imag(0) {};

    bool operator==(const Complex &rhs) const {
        return real == rhs.real &&
               imag == rhs.imag;
    }

    bool operator!=(const Complex &rhs) const {
        return !(rhs == *this);
    }

    Complex operator+(Complex const &complex) const {
        return(Complex(real+complex.real, imag+complex.imag));
    }
    Complex operator-(Complex const &complex){
        return(Complex(real-complex.real, imag-complex.imag));
    }

};


#endif //LAB04_COMPLEX_H
