// compile & run with:
// clear && rustc pythagoras_constant.rs && ./pythagoras_constant
// https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Digit-by-digit_calculation
// https://pl.10steps.org/Calculate-a-Square-Root-by-Hand-9247

fn main() {
    println!("{:?}", (99./70.));

    let mut x = 577;
    let y = 408;
    let mut z = x / y; // 1.4142857142857144 as i8 = 1
    let mut r = x - y; // 29
    print!("{:?}", z);
    print!(".");

    for i in 0..1000 {
        x = r * 10; // 29*10 = 290
        z = x / y; // 4.142857142857143 as i8 = 4
        r = x - y * z; // 290 - (70*4) = 10
        print!("{:?}", z);
    }
}

// expected result:
// 1.4142135623730950488016887242096980785696718753769480731766797379907324784621070388503875343276415727350138462309122970249248360558507372126441214970999358314132226659275055927557999505011527820605714701095599716059702745345968620147285174186408891986095523292304843087143214508397626036279952514079896872533965463318088296406206152583523950547457502877599617298355752203375318570113543746034084988471603868999706990048150305440277903164542478230684929369186215805784631115966687130130156185689872372352885092648612494977154218334204285686060146824720771435854874155657069677653720226485447015858801620758474922657226002085584466521458398893944370926591800311388246468157082630100594858704003186480342194897278290641045072636881313739855256117322040245091227700226941127573627280495738108967504018369868368450725799364729060762996941380475654823728997180326802474420629269124859052181004459842150591120249441341728531478105803603371077309182869314710171111683916581726889419758716582152128229518488472
