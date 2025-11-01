import chisel3._
import chisel3.util._

class CPUTop extends Module {
  val io = IO(new Bundle {
    val done = Output(Bool())
    val run = Input(Bool ())
    //This signals are used by the tester for loading and dumping the memory content, do not touch
    val testerDataMemEnable = Input(Bool ())
    val testerDataMemAddress = Input(UInt (16.W))
    val testerDataMemDataRead = Output(UInt (32.W))
    val testerDataMemWriteEnable = Input(Bool ())
    val testerDataMemDataWrite = Input(UInt (32.W))
    //This signals are used by the tester for loading and dumping the memory content, do not touch
    val testerProgMemEnable = Input(Bool ())
    val testerProgMemAddress = Input(UInt (16.W))
    val testerProgMemDataRead = Output(UInt (32.W))
    val testerProgMemWriteEnable = Input(Bool ())
    val testerProgMemDataWrite = Input(UInt (32.W))
  })

  //Creating components
  val programCounter = Module(new ProgramCounter())
  val dataMemory = Module(new DataMemory())
  val programMemory = Module(new ProgramMemory())
  val registerFile = Module(new RegisterFile())
  val controlUnit = Module(new ControlUnit())
  val alu = Module(new ALU())

  //Connecting the modules
  programCounter.io.run := io.run
  programCounter.io.stop := io.done
  programMemory.io.address := programCounter.io.programCounter

  val opcode       = Wire(UInt(4.W))
  val register1    = Wire(UInt(5.W))
  val register2    = Wire(UInt(5.W))
  val register3    = Wire(UInt(5.W))
  val immediate18  = Wire(UInt(18.W))
  val immediate28  = Wire(UInt(28.W))
  val outImmediate  = Wire(UInt(32.W))

  val ReadR1OrR3  = Wire(UInt(32.W))
  val ReadImedOrR = Wire(UInt(32.W))

  val ALUMemResult = Wire(UInt(32.W))
  val WriteData = Wire(UInt(32.W))

  val instruction = programMemory.io.instructionRead

  opcode := instruction(31,28)
  register1 := instruction(27,23)
  register2 := instruction(22,18)
  register3 := instruction(17,13)

  immediate18 := instruction(17,0)
  immediate28 := instruction(27,0)

  registerFile.io.register1 := register1
  registerFile.io.register2 := register2
  registerFile.io.register3 := register3

  controlUnit.io.opcode := opcode

  //Control signals:
  registerFile.io.regWrite := controlUnit.io.regWrite
  dataMemory.io.readMem := controlUnit.io.readMem
  dataMemory.io.writeEnable := controlUnit.io.writeMem

  //Logic wiring for immediates vs registers
  outImmediate := Mux(controlUnit.io.jumpImmediate, immediate28.pad(32), immediate18.pad(32))
  ReadR1OrR3 := Mux(controlUnit.io.readR1, registerFile.io.read1, registerFile.io.read3)
  ReadImedOrR := Mux(controlUnit.io.readImmediate, outImmediate, ReadR1OrR3)

  //ALU wiring:
  alu.io.opcode := opcode
  alu.io.operand1 := registerFile.io.read2
  alu.io.operand2 := ReadImedOrR

  //Memory wiring:
  dataMemory.io.address := registerFile.io.read2
  dataMemory.io.dataWrite := ReadImedOrR
  ALUMemResult := Mux(controlUnit.io.memToReg, dataMemory.io.dataRead, alu.io.result)
  WriteData := Mux(controlUnit.io.loadImmediate, outImmediate, ALUMemResult)

  //Jump logic:
  programCounter.io.programCounterJump := outImmediate.tail(16)
  programCounter.io.jump := (alu.io.result.head(1) & controlUnit.io.jumpLess) | (alu.io.zero_flag & controlUnit.io.jumpEqual) | controlUnit.io.jumpImmediate


  ////////////////////////////////////////////
  //Continue here with your connections
  ////////////////////////////////////////////

  //This signals are used by the tester for loading the program to the program memory, do not touch
  programMemory.io.testerAddress := io.testerProgMemAddress
  io.testerProgMemDataRead := programMemory.io.testerDataRead
  programMemory.io.testerDataWrite := io.testerProgMemDataWrite
  programMemory.io.testerEnable := io.testerProgMemEnable
  programMemory.io.testerWriteEnable := io.testerProgMemWriteEnable
  //This signals are used by the tester for loading and dumping the data memory content, do not touch
  dataMemory.io.testerAddress := io.testerDataMemAddress
  io.testerDataMemDataRead := dataMemory.io.testerDataRead
  dataMemory.io.testerDataWrite := io.testerDataMemDataWrite
  dataMemory.io.testerEnable := io.testerDataMemEnable
  dataMemory.io.testerWriteEnable := io.testerDataMemWriteEnable
}