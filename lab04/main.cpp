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
        return 1;
    }
    //fftDouble.tmp();

    fftDouble.ditfft2(fftDouble.xcppVec);
    fftFloat.ditfft2(fftFloat.xcppVec);

    try {
        fftDouble.dataSaver("E:\\Projekty\\Kodowe\\MATLAB\\CPS\\lab04\\XcppDouble.txt");
        fftFloat.dataSaver("E:\\Projekty\\Kodowe\\MATLAB\\CPS\\lab04\\XcppFloat.txt");
    }
    catch (const char* msg) {
        std::cerr << msg << std::endl;
        return 1;
    }
    return 0;
}