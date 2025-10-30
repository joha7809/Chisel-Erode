import chisel3._
import chisel3.util._

class RegisterFile extends Module {
  val io = IO(new Bundle {
    val register1 = Input(UInt(5.W)) //If we write we always write to the first given register
    val register2 = Input(UInt(5.W))
    val register3 = Input(UInt(5.W))
    val read1 = Output(SInt(32.W))
    val read2 = Output(SInt(32.W))
    val read3 = Output(SInt(32.W))
    val writeData = Input(SInt(32.W))
    val regWrite = Input(Bool())
    //Define the module interface here (inputs/outputs)
  })
  val registers = RegInit(VecInit(Seq.fill(32)(0.S(32.W))))

  io.read1 := registers(io.register1)
  io.read2 := registers(io.register2)
  io.read3 := registers(io.register3)

  when(io.regWrite) {
    registers(io.register1) := io.writeData
  }



  //Implement this module here

}