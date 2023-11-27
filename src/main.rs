mod manipulation;
mod util;

use manipulation::process;

fn main() {
    process("wałówka");
    process("okno");
    process("koń");
    process("kwadrat");
    process("nożyk");
    process("bezwzględny");
    process("pierdolić");
//    process("większość_bezwzględna"); -> debug
//    process("większość_absolutna"); -> debug
}