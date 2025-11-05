import chisel3._
import chisel3.util._
import firrtl.Utils.True

class ALU extends Module {
  val io = IO(new Bundle {
    // Define the module interface here (inputs/outputs)
    val operand1 = Input(SInt(32.W)) // Always R2
    val operand2 = Input(SInt(32.W)) // Will be one of R1, R3 or Imm
    val opcode = Input(UInt(4.W))
    val result = Output(SInt(32.W))
    val zero_flag = Output(Bool())
  })
  // Default value. When an irrelevant opcode is received, simply pass along 0.S
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

    // For JEQ and JLT, operand1=R2 and operand2=R1, but we want to compare R1-R2
    // So we swap: compute operand2 - operand1
    is("b1101".U){io.result := io.operand2 - io.operand1} // Jump if equal (R1 - R2) Order does not matteer here, we only care if the result i 0.
    is("b1110".U){io.result := io.operand2 - io.operand1} // Jump if less than If R1 is less than R2 then R1 - R2 will be negative <=> operand2(R2) - operand1(R1)



  }

  when(io.result === 0.S){
    io.zero_flag := 1.B
  }.otherwise(
    io.zero_flag := 0.B
  )
}

