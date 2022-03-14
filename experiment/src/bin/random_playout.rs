use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{thread, time};
use yasai::Position;

fn main() {
    let mut rng = thread_rng();
    let mut pos = Position::default();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!();
    println!("{pos}");
    for i in 0.. {
        thread::sleep(time::Duration::from_millis(50));
        let moves = pos.legal_moves().into_iter().collect::<Vec<_>>();
        if let Some(&m) = moves.choose(&mut rng) {
            pos.do_move(m);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("{:>20}: {m}", i + 1);
            println!("{pos}");
        } else {
            break;
        }
    }
}
