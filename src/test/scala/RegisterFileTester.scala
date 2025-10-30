import chisel3._
import chiseltest._
import org.scalatest.flatspec.AnyFlatSpec

class RegisterFileTester extends AnyFlatSpec with ChiselScalatestTester {

  "RegisterFileTester" should "pass" in {
    test(new RegisterFile())
      .withAnnotations(Seq(WriteVcdAnnotation)) { dut =>

        // Write 37 to register 4
        dut.io.register1.poke(4.U)
        dut.io.writeData.poke(37.S)
        dut.io.regWrite.poke(true.B)
        dut.clock.step() // perform the write on rising edge

        // Now disable writing
        dut.io.regWrite.poke(false.B)

        // Read from register 4 and 5 and 6
        dut.io.register1.poke(4.U)
        dut.io.register2.poke(5.U)
        dut.io.register3.poke(6.U)

        dut.clock.step()

        // Expect register4 == 37
        dut.io.read1.expect(37.S)
        // Expect registers 5 and 6 are still 0
        dut.io.read2.expect(0.S)
        dut.io.read3.expect(0.S)

      }
  }
}

