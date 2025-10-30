import chisel3._
import chiseltest._
import org.scalatest.flatspec.AnyFlatSpec

class ALUTester extends AnyFlatSpec with ChiselScalatestTester {

  "ALUTester" should "pass" in {
    test(new ALU())
      .withAnnotations(Seq(WriteVcdAnnotation)) { dut =>

        //Minus gives negative
        dut.io.opcode.poke(2.U)
        dut.io.operand1.poke(37.S)
        dut.io.operand2.poke(45.S)
        dut.io.result.expect(-8.S)
        dut.io.zero_flag.expect(0.B)
        //Minus gives zero
        dut.io.opcode.poke(2.U)
        dut.io.operand1.poke(37.S)
        dut.io.operand2.poke(37.S)
        dut.io.result.expect(0.S)
        dut.io.zero_flag.expect(1.B)

        //Multiplier does multiplication
        dut.io.opcode.poke(3.U)
        dut.io.operand1.poke(37.S)
        dut.io.operand2.poke(45.S)
        dut.io.result.expect(1665.S)
        dut.io.zero_flag.expect(0.B)
        //Load -- Irrelevant opcode, should return 0.S
        dut.io.opcode.poke(9.U)
        dut.io.operand1.poke(37.S)
        dut.io.operand2.poke(45.S)
        dut.io.result.expect(0.S)
        dut.io.zero_flag.expect(1.B)
        //

    }
  }
}

