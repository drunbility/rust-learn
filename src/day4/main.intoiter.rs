struct Grid {
    x_c: Vec<u32>,
    y_c: Vec<u32>,
}

struct GridIter {
    grid:Grid,
    i:usize,
    j:usize,
}


impl IntoIterator for Grid {
    type Item = (u32,u32);
    type IntoIter = GridIter;

    fn into_iter(self) -> GridIter{
        GridIter {grid:self,i:0,j:0}
    }
    
}

impl Iterator for GridIter {
    type Item = (u32,u32);
    fn next(&mut self) -> Option<(u32,u32)> {
        if self.i >= self.grid.x_c.len() {
            self.i =0;
            self.j +=1; 
            if self.j >= self.grid.y_c.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_c[self.i],self.grid.y_c[self.j]));
        self.i+=1;
        return res;
    }
}

fn main() {
    let grid = Grid { x_c: vec![3, 5, 7, 9], y_c: vec![10, 20, 30, 40] };
    for (x, y) in grid {
        println!("point = {x}, {y}");
    }
}
