fn main() {
    let num_rings = 50;

    let mut game = Game::new(num_rings);

    let mut solver = Solver {
        num_rings,
    };

    let moves = solver.solve();

    println!("{:#?}", &moves);

    for m in &moves {
        game.move_ring(&m);
    }

    println!("{:#?}", game);
}

#[derive(Debug)]
pub struct Game {
    pub num_rings: usize,
    towers: Vec<Vec<usize>>,
}

impl Game {
    pub fn new(num_rings: usize) -> Game {
        let mut towers = Vec::new();
        for _ in 0..3 {
            towers.push(Vec::new());
        }

        for r in (0..num_rings).rev() {
            towers[0].push(r + 1);
        }

        Game {
            num_rings,
            towers,
        }
    }

    pub fn move_ring(&mut self, mv: &HanoiMove) {
        if !self.can_move(mv.index_of_from(), mv.index_of_to()) {
            panic!(format!("Cannot move from {:?} to {:?} with state {:?}", mv.from, mv.to, &self.towers));
        }

        let ring = self.towers[mv.index_of_from()].pop()
            .expect(&format!("Failed to pop from {:?}", mv.from));

        self.towers[mv.index_of_to()].push(ring);
    }

    fn can_move(&self, from: usize, to: usize) -> bool {
        let from_tower = &self.towers[from];
        if from_tower.is_empty() {
            return false;
        }

        let ring_to_move = from_tower.last();

        let to_tower = &self.towers[to];
        if !to_tower.is_empty() && to_tower.last() <= ring_to_move {
            return false;
        }

        return true;
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Tower {
    First,
    Second,
    Third,
}

#[derive(Debug)]
pub struct HanoiMove {
    ring: usize,
    from: Tower,
    to: Tower,
}

impl HanoiMove {
    pub fn index_of_from(&self) -> usize {
        HanoiMove::map_tower_to_index(&self.from)
    }

    pub fn index_of_to(&self) -> usize {
        HanoiMove::map_tower_to_index(&self.to)
    }

    fn map_tower_to_index(tower: &Tower) -> usize {
        match tower {
            Tower::First => 0,
            Tower::Second => 1,
            Tower::Third => 2,
        }
    }
}

pub struct Solver {
    num_rings: usize,
}

impl Solver {
    pub fn solve(&mut self) -> Vec<HanoiMove> {
        self.move_ring(
            self.num_rings,
            &Tower::First,
            &Tower::Third,
            &Tower::Second)
    }

    pub fn move_ring(&mut self, ring_number: usize, from: &Tower, to: &Tower, aux: &Tower) -> Vec<HanoiMove> {
        let current_move = HanoiMove {
            ring: ring_number,
            from: from.clone(),
            to: to.clone(),
        };

        if ring_number <= 1 {
            return vec![current_move];
        }

        let mut moves = Vec::new();

        let mut pre_moves = self.move_ring(ring_number - 1, from, aux, to);
        let mut post_moves = self.move_ring(ring_number - 1, aux, to, from);

        moves.append(&mut pre_moves);
        moves.push(current_move);
        moves.append(&mut post_moves);

        moves
    }
}