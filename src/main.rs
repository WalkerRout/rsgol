#![allow(dead_code)]

type Position = (usize, usize);

#[derive(Debug, Clone)]
struct Node {
  future_status: Option<bool>,
  is_alive: bool,
  position: Position
}

impl Node {
  fn new(is_alive: bool, position: Position) -> Self {
    Node { future_status: None, is_alive, position }
  }
  
  fn char(&self) -> String {
    format!("{}", if self.is_alive {ALIVE_SYMBOL} else {DEFAULT_SYMBOL})
  }
}

#[derive(Debug)]
struct Grid {
  width: usize,
  height: usize,
  map: Vec<Vec<Node>>
}

impl Grid {
  fn new(width: usize, height: usize) -> Self {
    let mut map: Vec<Vec<Node>> = Vec::new();
    
    for i in 0..height {
      map.push(Vec::new());
      for j in 0..width {
        map[i].push(Node::new(false, (i, j)));
      }
    }
    
    Grid { width, height, map }
  }

  fn modify<F>(&mut self, mod_func: F)
  where F: Fn((usize, &mut Node)) -> () {
    self.map
      .iter_mut()
      .flatten()
      .enumerate()
      .for_each(mod_func);
  }

  fn load_map(&mut self, map: &Vec<Vec<Node>>) -> bool {
    let width = map[0].len();
    
    for i in 1..map.len() {
      if width != map[i].len() {
        return false;
      }
    }

    self.width = width;
    self.height = map.len();
    self.map = map.clone();

    true
  }

  fn node_ref(&mut self, position: Position) -> Option<&mut Node> {
    if position.0 >= self.height || position.1 >= self.width {
      None
    } else {
      Some(&mut self.map[position.0][position.1])
    }
  }

  fn set_node(&mut self, position: Position, node: Node) -> bool {
    if let Some(r) = self.node_ref(position) {
      *r = node;
      true
    } else {
      false
    }
  }

  fn step_iterations(&mut self, num_iter: usize) -> &Vec<Vec<Node>> {
    for _ in 0..num_iter {
      self.update();
    }

    &self.map
  }

  fn update(&mut self) {
    for i in 0..self.height {
      // could parallelize this section as an exercise
      for j in 0..self.width {
        let mut neighbours: Vec<Node> = Vec::new();
        
        for ni in -1..=1 {
          for nj in -1..=1 {
            if ((i as i32 + ni) < 0)
            || ((i as i32 + ni) >= self.height as i32)
            || ((j as i32 + nj) < 0)
            || ((j as i32 + nj) >= self.width as i32)
            || (ni == 0 && nj == 0) {
              continue;
            }
            neighbours.push(self.map[(i as i32 + ni) as usize][(j as i32 + nj) as usize].clone());
          }
        }
        
        let curr_node = &mut self.map[i][j];
        let alive_neighbours = neighbours
          .iter()
          .filter(|x| x.is_alive)
          .collect::<Vec<&Node>>()
          .len();

        if alive_neighbours == 3 {
          curr_node.future_status = Some(true);
        }

        if alive_neighbours <= 1 {
          curr_node.future_status = Some(false);
        }

        if alive_neighbours >= 4 {
          curr_node.future_status = Some(false);
        }
      }
    }
    
    self.map
      .iter_mut()
      .flatten()
      .for_each(|ele| {
        if let Some(status) = ele.future_status {
          ele.is_alive = status;
        }
      });
  }
  
  fn draw(&self) {
    let mut print_string = String::new();

    // this implementation could be done with a for loop, but i wanted to try to do it this way as a learning activity
    for i in 0..self.height * self.width {
      if i % self.width == 0 {
        print_string.push('\n');
      } else {
        let j: usize = i % self.width;
        let i: usize = i / self.width;
        print_string.push_str(&self.map[i][j].char());
      }
    }

    println!("{}", print_string);
  }

  fn run(&mut self, step_duration: std::time::Duration) -> ! {
    let mut iter = 0;
    loop {
      print!("\x1B[2J\x1B[1;1H\n");
      //std::io::stdout().flush().unwrap();
      
      self.draw();
      self.update();
      
      println!("iter: {}", iter);
      iter += 1;
      
      std::thread::sleep(step_duration);
    }
  }
}

const DEFAULT_SYMBOL: char = ' ';
const ALIVE_SYMBOL:   char = '@';

fn main() {
  let mut grid = Grid::new(50, 100);

  grid.modify(|(i, ele)| {
    if i % 7 == 0
    || i % 5 == 0 
    || i % 4 == 0
    && i % 3 != 0 {
      ele.is_alive = true;
    }
  });

  let map = grid.step_iterations(1).clone();
  grid.load_map(&map);

  grid.run(std::time::Duration::from_millis(100));
}


/*
 * NOTES:
 * 
 * Some other cool generations:
   if i % 7 == 0
   || i % 5 == 0 
   || i % 4 == 0
   && i % 3 != 0 {
     ele.is_alive = true;
   }
 * 
 * 
 * 
 * 
*/
