package fft

import scala.annotation.tailrec
import scala.io.Source
import scala.Vector

object Raidix2 extends App {
  def power2toNTail(n: Int): Int = {
    @tailrec
    def helper(n: Int, currentVal: Int): Int = {
      if (n == 0) currentVal else helper(n - 1, currentVal * 2)
    }

    helper(n, 1)
  }


  def loadFileToDoubleVector(filename: String): Vector[ComplexDouble] = Source
    .fromFile(filename)
    .getLines()
    .map(x =>
      ComplexDouble(
        x.split(",")(0).toDouble,
        x.split(",")(1).toDouble))
    .toVector

  @tailrec
  def sumComplexVector(sum: Double, x: Vector[ComplexDouble]): Double = {
    if (x.isEmpty) return sum
    sumComplexVector(sum + x.head.modulus, x.drop(0))
  }

  def raidix2(x: Vector[ComplexDouble], N: Int, s: Int): Vector[ComplexDouble] = {
    // make sure N is power of 2

    val N = x.length
    val S = math.log(N)
    var Half = N / 2
    var X = Vector[ComplexDouble]()

    def Helper(n: Int, stage: Int, index: Int, Half: Int, x: Vector[ComplexDouble]):
    Vector[ComplexDouble] = {
      if (n == Half - 1) {
        return x
      }
      val pos = n + index + 1
      val pow = (2 ^ (stage - 1)) * n
      val w = math.exp((ComplexDouble(0, -1) * ComplexDouble(2 * math.Pi) * ComplexDouble(pow / N)).modulus)
      val a = x(pos) + x(pos + Half)
      val b = (x(pos) - x(pos + Half)) * w
      Helper(n + 1, stage, index + 1, Half, x)
    }

    for (stage <- 1 to S.toInt) {
      for (index <- 0 until N by (N / (2 ^ (stage - 1)))) {
        X = Helper(0, stage, index, Half, x)
      }
      Half = Half / 2
    }
    X
  }

  val result = loadFileToDoubleVector("toload.txt")
  println(result)
}