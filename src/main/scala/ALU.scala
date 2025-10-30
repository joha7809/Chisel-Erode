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
  switch(opcode){
    is("b0001".U) {result := operand1 + operand2} //Add
    is("b0010".U) {result := operand1 - operand2} //Subtract
    is("b0011".U) {result := operand1 + operand2} //Multiply
    is("b0100".U) {result := operand1 + operand2} //Add immediate
    is("b0101".U){result := operand1 - operand2} //Subtract immediate

    is("b0110".U){result := operand1 | operand2} //Bitwise OR
    is("b0111".U){result := ~operand1} //Bitwise NOT
    is("b1000".U){result := operand1 & operand2} //Bitwise AND

  }.otherwise{result:= 0}
}

