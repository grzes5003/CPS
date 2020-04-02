//
// Created by Grzes on 2020-04-02.
//

#include "FFT.h"

template<class T>
void FFT<T>::ditfft2(std::vector<std::complex<T>> &x) {

    // check if power of 2

//    std::vector<std::complex<T>> x1, x2;
//    bool toggle = false;
//    std::partition_copy(x.begin(),
//                        x.end(),
//                        std::back_inserter(x1),
//                        std::back_inserter(x2),
//                        [&toggle](int) { return toggle = !toggle; });
//
//    std::vector<std::complex<T>> X = FFT<T>::ditfft2(x1, N/2, 2*s);
//    std::vector<std::complex<T>> X2 = FFT<T>::ditfft2(x1, N/2, 2*s);

//    X.insert(X.end(), X2.begin(), X2.end());

    // Length variables
    size_t n = x.size();
    int levels = 0;  // Compute levels = floor(log2(n))
    for (size_t temp = n; temp > 1U; temp >>= 1)
        levels++;
    if (static_cast<size_t>(1U) << levels != n)
        throw std::domain_error("Length is not a power of 2");

    // Trignometric table
    std::vector<std::complex<T> > expTable(n / 2);
    for (size_t i = 0; i < n / 2; i++)
        expTable[i] = std::polar(1.0, -2 * M_PI * i / n);

    // Bit-reversed addressing permutation
    for (size_t i = 0; i < n; i++) {
        size_t j = reverseBits(i, levels);
        if (j > i)
            std::swap(x[i], x[j]);
    }

    // Cooley-Tukey decimation-in-time radix-2 FFT
    for (size_t size = 2; size <= n; size *= 2) {
        size_t halfsize = size / 2;
        size_t tablestep = n / size;
        for (size_t i = 0; i < n; i += size) {
            for (size_t j = i, k = 0; j < i + halfsize; j++, k += tablestep) {
                std::complex<T> temp = x[j + halfsize] * expTable[k];
                x[j + halfsize] = x[j] - temp;
                x[j] += temp;
            }
        }
        if (size == n)  // Prevent overflow in 'size *= 2'
            break;
    }
}

template<class T>
void FFT<T>::dataLoader(const std::string &filename) {
    std::ifstream infile(filename);
    if (!infile.is_open()) {
        throw "cannot open file";
    }

    std::string line;
    while (std::getline(infile, line)) {
        std::istringstream iss(line);
        T a, b;
        if (!(iss >> a >> b)) { break; } // error

        xcppVec.push_back(std::complex<T>(a, b));
    }
}

template<class T>
int FFT<T>::tmp() {
    return (1);
}

template<class T>
const std::vector<std::complex<T>> &FFT<T>::getXcppVec() const {
    return xcppVec;
}

template<class T>
FFT<T>::FFT() = default;

template
class FFT<float>;

template
class FFT<double>;

