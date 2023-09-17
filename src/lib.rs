use std::{fs::File, io::Read};

use rand::Rng;

pub struct Board
{
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<u32>>,
}

impl Board
{
    pub fn new(width: usize, height: usize) -> Board
    {
        let cells = vec![vec![0; width]; height];

        Board
        {
            width,
            height,
            cells,
        }
    }

    pub fn dead_state(&mut self)
    {
        for x in 0..self.width
        {
            for y in 0..self.height
            {
                self.set_cell(x, y, 0);
            }
        }
    }

    pub fn get_cell(&self, x: isize, y: isize) -> Option<u32>
    {
        if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize
        {
            Some(self.cells[x as usize][y as usize])
        }
        else
        {
            None
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: u32)
    {
        if x < self.width && y < self.height
        {
            self.cells[x][y] = cell;
        }
    }

    pub fn random_state(&mut self, density: f32)
    {
        for x in 0..self.width
        {
            for y in 0..self.height
            {
                self.set_cell(x, y, 0);

                let random_number: f32 = rand::thread_rng().gen();

                if random_number < density
                {
                    self.set_cell(x, y, 1);
                }
            }
        }
    }

    pub fn update_state(&mut self)
    {
        let mut new_state = self.cells.clone();

        for x in 0..self.width
        {
            for y in 0..self.height
            {
                let neighbors = self.count_neighbors(x, y);

                match self.get_cell(x as isize, y as isize)
                {
                    Some(1) =>
                    {
                        match neighbors
                        {
                            0..=1 => new_state[x][y] = 0,
                            2..=3 => (),
                            _ => new_state[x][y] = 0,

                        }
                    }
                    Some(0) if neighbors == 3 =>
                    {
                        new_state[x][y] = 1;
                    }
                    _ => (),
                }

                //Temporary code to delete cells moving to the edge
                if x == self.width - 1 || y == self.height - 1
                {
                    new_state[x][y] = 0;
                }
                
            }
        }
        self.cells = new_state;
    }

    pub fn gosper_glider(&mut self)
    {
        self.dead_state();

        let mut new_state = self.cells.clone();

        let origin_x = (self.width / 2) - 36;
        let origin_y = (self.height / 2) - 36;
        
        let filename = "gosper_glider.txt";

        let mut file = File::open(filename).expect("File not found");

        let mut contents = String::new();

        
        file.read_to_string(&mut contents).expect("Error in file read");

        for (y, line) in contents.lines().enumerate()
        {
            for (x, token) in line.chars().enumerate()
            {
                if token == '1'
                {
                    new_state[origin_x + x][origin_y + y] = 1;
                }
            }
        }
        self.cells = new_state;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u32
    {
        let mut count: u32 = 0;
        let x = x as isize;
        let y = y as isize;

        for relative_x in -1..2
        {
            for relative_y in -1..2
            {          
                let result_x = x + relative_x;
                let result_y = y + relative_y;

                if !(result_x == x as isize && result_y == y as isize)
                {             
                    match self.get_cell(result_x, result_y)
                    {
                        Some(1) => count += 1,
                        _ => (),
                    }
                }
                
            }
        }
        count
    }
}

