//
// Created by Grzes on 2020-04-02.
//
#include "FFT.h"

int main() {
    FFT<double> fftDouble;
    FFT<float> fftFloat;
    try {
        fftDouble.dataLoader("E:\\Projekty\\Kodowe\\MATLAB\\CPS\\lab04\\xcpp.dat");
        fftFloat.dataLoader("E:\\Projekty\\Kodowe\\MATLAB\\CPS\\lab04\\xcpp.dat");
    }
    catch (const char* msg) {
        std::cerr << msg << std::endl;
    }
    //fftDouble.tmp();

    fftDouble.ditfft2(fftDouble.xcppVec);
    fftFloat.ditfft2(fftFloat.xcppVec);
    return 0;
}