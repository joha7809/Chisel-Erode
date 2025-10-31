import chisel3._
import chisel3.util._
import ujson.IndexedValue.True

class ControlUnit extends Module {
  val io = IO(new Bundle {
    val opcode = Input(UInt(4.W))
    val regWrite = Output(0.B)
    val loadImmediate = Output(0.B)

    val readR1 = Output(0.B)
    val readImmediate = Output(0.B)
    val readMem = Output(0.B)
    val writeMem = Output(0.B)
    val memToReg = Output(0.B)

    val jumpLess = Output(0.B)
    val jumpEqual = Output(0.B)
    val jumpImmediate = Output(0.B)

    val halt = Output(0.B)
    //Define the module interface here (inputs/outputs)
  })

  //Implement this module here
  switch(io.opcode){
    //ALU opcodes:
    is("b0001".U) {io.regWrite := 1.B} //Add needs to write to reg
    is("b0010".U) {io.regWrite := 1.B} //Same for subtraction
    is("b0011".U) {io.regWrite := 1.B} //Same for multiplication
    is("b0100".U) {io.regWrite := 1.B; io.readImmediate := 1.B} //Add immediate needs immediate (and reg)
    is("b0101".U) {io.regWrite := 1.B; io.readImmediate := 1.B} //Same for subtract immediate
    is("b0110".U){io.regWrite := 1.B} //Bitwise OR only needs to write to reg
    is("b0111".U){io.regWrite := 1.B; io.readR1} //Bitwise NOT needs to read R1 and write to reg
    is("b1000".U){io.regWrite := 1.B} //Bitwise AND only needs to write to reg as well

    //Data opcodes:
    is("b1001".U){io.regWrite := 1.B; io.loadImmediate := 1.B; io.readImmediate := 1.B}
    is("b1010".U){io.regWrite := 1.B; io.readMem := 1.B; io.memToReg := 1.B} //Technically we don't need both readMem and MemToReg
    is("b1011".U){io.readR1 := 1.B; io.writeMem := 1.B}

    //Jump opcodes:
    is("b1100".U){io.jumpImmediate := 1.B}
    is("b1101".U){io.jumpEqual := 1.B}
    is("b1110".U){io.jumpLess := 1.B}

    //Strict control opcodes:
    is("b0000".U){} //Nothing is done when NOP
    is("b1111".U){io.halt := 1.B} //End operation is just halt
  }
}