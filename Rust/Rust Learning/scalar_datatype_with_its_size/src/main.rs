fn main() {
    { /*Two type of Data Type:-(a)Scalar (b)Compound */ }
    //(1)Scalar:-For Single val. it can be int,float,char,boolean.
    //(2)Compound:-

    //(A)Integer

    { /* i8 ni size -128 to 127 chhe jo aena thi nani ke moti lakhi to err aape.  */ }
    //Niche no code error throw kare.
    // let x:i8=128;
    // let x:i8=-129;
    // println!("x's value {x}");

    { /*"u8" ni size 0 to 255 chhe,jo aeni to nani ke moti hase to err. nakhse. */ }
    // let y:u8=256; error nakhe
    // let y:u8=-2; error nakhe
    // println!("y's value {y}");

    { /*  */ }
    let a = 155u8; //Aam val. ni pasad pn lakhi sakai
    println!("a's value {a}");

    { /*1 lakh ne aam "_" thi pn lakhai. */ }
    let b = 1_00_000;
    println!("b's value {b}");

    //Aa nicheno code err. aape,but aene cargo run --release thi karo to '44' aape.
    // let c: u8 = myrandom_num() + 100;
    // println!("c's value {c}");

    // fn myrandom_num() -> u8 {
    //     200
    // }

    //(B)Float:-By default f64 j hoi.

    let f1 = -12.7;
    let f2: f32 = -15.7;

    //** Operations */
    let quetient = 56.7 / 32.2;
    println!("quetient's value {quetient}");

    {/*Aa nichena num. ne "int" gane chhe,bec. point passi kai nathi atle,
     pn aene jo 5.0 & 3.0 karie to o/p ma "-1.66666667" ave.
     */}
    let truncated = -5 / 3; //Aa trunc karse atle -1.6666 na '-1' kare.
    println!("truncated's value {truncated}");

    let remainder = 43 % 5;
    println!("remainder's value {remainder}");
}
