import chisel3._
import chisel3.util._

class ALU extends Module {
  val io = IO(new Bundle {
    // Define the module interface here (inputs/outputs)
    val operand1 = Input(SInt(32.W))
    val operand2 = Input(SInt(32.W))
    val opcode = Input(UInt(4.W))
    val result = Output(SInt(32.W))
    val zero_falg = Output(Bool())
  })

  // Implement this module here

}

