/*
   Çalışmakta olduğum Mazes for Programmers kitabını uygulamaya çalışıyorum.
   İlk örnekte binary tree algoritmasına göre labirent oluşturulması ele alınıyor.
   Hatta başlangıç noktasından bitişe ulaşmak için Dijkstra algoritması kullanılıyor.
   Sorun şu ki kitap örnek kodları Ruby ile yazmış. Rust'a evirmek pek kolay değil.
*/
use crate::grid::Grid;

mod cell;
mod grid;
mod grid_row;
mod test;

fn main() {
    let mut grid = Grid::new(4, 5);
    grid.prepare();
    println!("Grid size {}", grid.get_size());

    for row in grid.iter_rows() {
        for cell in row.iter() {
            print!("{}\t", cell.mark);
        }
        println!("");
    }
}
