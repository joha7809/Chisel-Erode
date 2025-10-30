import chisel3._
import chisel3.util._
import firrtl.Utils.True

class ALU extends Module {
  val io = IO(new Bundle {
    // Define the module interface here (inputs/outputs)
    val operand1 = Input(SInt(32.W))
    val operand2 = Input(SInt(32.W))
    val opcode = Input(UInt(4.W))
    val result = Output(SInt(32.W))
    val zero_flag = Output(Bool())
  })

  io.result := 0.S
  // Implement this module here
  switch(io.opcode){
    is("b0001".U) {io.result := io.operand1 + io.operand2} //Add
    is("b0010".U) {io.result := io.operand1 - io.operand2} //Subtract
    is("b0011".U) {io.result := io.operand1 * io.operand2} //Multiply
    is("b0100".U) {io.result := io.operand1 + io.operand2} //Add immediate
    is("b0101".U){io.result := io.operand1 - io.operand2} //Subtract immediate

    is("b0110".U){io.result := io.operand1 | io.operand2} //Bitwise OR
    is("b0111".U){io.result := ~io.operand1} //Bitwise NOT
    is("b1000".U){io.result := io.operand1 & io.operand2} //Bitwise AND

  }

  when(io.result === 0.S){
    io.zero_flag := 1.B
  }.otherwise(
    io.zero_flag := 0.B
  )
}

