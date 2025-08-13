//#[derive(Debug)]
//
//enum Direc {
//    Left,
//    Right,
//}
//
//#[derive(Debug)]
//enum PlayM {
//    Pass,
//    Run(Direc),
//    Teleport{x:u32,y:u32},
//}
//
//fn main() {
//
//    let dir = Direc::Left;
//    let ply:PlayM = PlayM::Run(dir);
//    println!("on this return: {ply:?}");
//    
//}
#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

fn main() {
    let dir = Direction::Left;
    let player_move: PlayerMove = PlayerMove::Run(dir);
    println!("On this turn: {player_move:?}");
}
