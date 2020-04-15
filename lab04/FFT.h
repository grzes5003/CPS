//
// Created by Grzes on 2020-04-02.
//

#ifndef LAB04_FFT_H
#define LAB04_FFT_H

#include <complex>
#include <vector>
#include <string>
#include <fstream>
#include <iostream>
#include <algorithm>
#include <iterator>
#include <iomanip>

template<class T>
class FFT {
public:
    std::vector<std::complex<T>> xcppVec;

    FFT(const std::vector<std::complex<T>> &xcppVec) : xcppVec(xcppVec) {}

    FFT();

    void dataLoader(const std::string &filename);
    void dataSaver(const std::string &filename);

    void ditfft2(std::vector<std::complex<T>> &x);

    int tmp();

    const std::vector<std::complex<T>> &getXcppVec() const;
};

static size_t reverseBits(size_t x, int n) {
    size_t result = 0;
    for (int i = 0; i < n; i++, x >>= 1)
        result = (result << 1) | (x & 1U);
    return result;
}

#endif //LAB04_FFT_H
