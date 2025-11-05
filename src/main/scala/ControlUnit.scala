import chisel3._
import chisel3.util._
import ujson.IndexedValue.True

class ControlUnit extends Module {
  val io = IO(new Bundle {
    val opcode = Input(UInt(4.W))
    val regWrite = Output(Bool())
    val loadImmediate = Output(Bool())

    val readR1 = Output(Bool())
    val readImmediate = Output(Bool())
    val readMem = Output(Bool())
    val writeMem = Output(Bool())
    val memToReg = Output(Bool())

    val jumpLess = Output(Bool())
    val jumpEqual = Output(Bool())
    val jumpImmediate = Output(Bool())

    val halt = Output(Bool())
    //Define the module interface here (inputs/outputs)
  })
  // default values, when control flags are set we override
  io.regWrite := false.B
  io.loadImmediate := false.B
  io.readR1 := false.B
  io.readImmediate := false.B
  io.readMem := false.B
  io.writeMem := false.B
  io.memToReg := false.B
  io.jumpLess := false.B
  io.jumpEqual := false.B
  io.jumpImmediate := false.B
  io.halt := false.B

  //Implement this module here
  switch(io.opcode){
    //ALU opcodes:
    is("b0001".U) {io.regWrite := 1.B} //Add needs to write to reg
    is("b0010".U) {io.regWrite := 1.B} //Same for subtraction
    is("b0011".U) {io.regWrite := 1.B} //Same for multiplication
    is("b0100".U) {io.regWrite := 1.B; io.readImmediate := 1.B} //Add immediate needs immediate (and reg)
    is("b0101".U) {io.regWrite := 1.B; io.readImmediate := 1.B} //Same for subtract immediate
    is("b0110".U){io.regWrite := 1.B} //Bitwise OR only needs to write to reg
    is("b0111".U){io.regWrite := 1.B} //Bitwise NOT needs to write to reg
    is("b1000".U){io.regWrite := 1.B} //Bitwise AND only needs to write to reg as well

    //Data opcodes:
    is("b1001".U){io.regWrite := 1.B; io.loadImmediate := 1.B; io.readImmediate := 1.B}
    is("b1010".U){io.regWrite := 1.B; io.readMem := 1.B; io.memToReg := 1.B} 
    is("b1011".U){io.readR1 := 1.B; io.writeMem := 1.B}

    //Jump opcodes:
    // jumpImmediate ensures we get correct immediate, readImmediate just passes to ALU so this is not needed. With other flags false, we ensure the immediate wires into the PC as the next instr
    is("b1100".U){io.jumpImmediate := 1.B}     
    // For the next two instructions we need to read R1, otherwise R3 is passed along which is corrupt data.
    is("b1101".U){io.jumpEqual := 1.B; io.readR1 := 1.B}  // JEQ needs to read R1 for comparison
    is("b1110".U){io.jumpLess := 1.B; io.readR1 := 1.B}   // JLT needs to read R1 for comparison

    //Strict control opcodes:
    is("b0000".U){} //Nothing is done when NOP
    is("b1111".U){io.halt := 1.B} //End operation is just halt
  }
}
