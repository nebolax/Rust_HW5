use regex::Regex;

#[derive(PartialEq, Eq, Copy, Clone)]
enum CellStatus {
    Cross,
    Zero,
    Empty,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum MoveStatus {
    Success,
    LimitError,
    CellValError,
}

struct Field {
    data: Vec<Vec<Option<CellStatus>>>,
}

impl Clone for Field {
    fn clone(&self) -> Field {
        let mut res = Vec::new();
        for i in 0..3 {
            let to_add: Vec<Option<CellStatus>> = Vec::new();
            res.push(to_add);
            for a in 0..3 {
                res[i].push(self.data[i][a]);
            }
        }
        Self { data: res }
    }
}

impl Field {
    pub fn new() -> Self {
        let mut res = Vec::new();
        for i in 0..3 {
            let to_add: Vec<Option<CellStatus>> = Vec::new();
            res.push(to_add);
            for _ in 0..3 {
                res[i].push(Option::from(CellStatus::Empty));
            }
        }

        Self { data: res }
    }
    fn win_rows(self) -> CellStatus {
        let data = self.data;
        for i in data {
            let cval = i[0].unwrap();
            let mut isgood = true;
            for a in i.iter() {
                if a.unwrap() != cval {
                    isgood = false;
                }
            }
            if isgood && cval != CellStatus::Empty {
                return cval;
            }
        }
        CellStatus::Empty
    }
    fn win_cols(self) -> CellStatus {
        let data = self.data;
        for i in 0..3 {
            let cur_val = data[0][i].unwrap();
            let mut is_good = true;
            for a in data.iter() {
                if a[i].unwrap() != cur_val {
                    is_good = false;
                }
            }
            if is_good && cur_val != CellStatus::Empty {
                return cur_val;
            }
        }
        CellStatus::Empty
    }
    fn win_diagonal(self) -> CellStatus {
        let data = self.data;
        if data[0][0] == data[1][1]
            && data[1][1] == data[2][2]
            && data[0][0].unwrap() != CellStatus::Empty
        {
            return data[0][0].unwrap();
        }
        if data[0][2] == data[1][1]
            && data[1][1] == data[2][0]
            && data[0][2].unwrap() != CellStatus::Empty
        {
            return data[0][2].unwrap();
        }
        CellStatus::Empty
    }
    pub fn check_win(&mut self) -> CellStatus {
        let r1 = self.clone().win_cols();
        let r2 = self.clone().win_rows();
        let r3 = self.clone().win_diagonal();
        if r1 != CellStatus::Empty {
            return r1;
        } else if r2 != CellStatus::Empty {
            return r2;
        } else if r3 != CellStatus::Empty {
            return r3;
        }
        CellStatus::Empty
    }
    pub fn make_move(&mut self, x: i32, y: i32, val: CellStatus) -> MoveStatus {
        if x > 2 || x < 0 || y > 2 || y < 0 {
            MoveStatus::LimitError
        } else if self.data[y as usize][x as usize].unwrap() != CellStatus::Empty {
            MoveStatus::CellValError
        } else {
            self.data[y as usize][x as usize] = Option::from(val);
            MoveStatus::Success
        }
    }
    pub fn get_item(self, x: i32, y: i32) -> CellStatus {
        self.data[y as usize][x as usize].unwrap()
    }
}

fn print_field(field: &Field) {
    println!("Текущее поле:");
    println!(" |0|1|2|");
    for i in 0..3 {
        print!("{}|", i);
        for a in 0..3 {
            match field.clone().get_item(a, i) {
                CellStatus::Cross => {
                    print!("X");
                }
                CellStatus::Zero => {
                    print!("0");
                }
                _ => {
                    print!(" ");
                }
            }
            print!("|");
        }
        println!();
    }
}

fn main() {
    let mut field = Field::new();

    let mut run = true;
    let mut cur_turn = CellStatus::Cross;
    let mut counter = 0;

    print_field(&field);
    while run {
        println!("Введите ход, формат: x y.");
        let mut inp_move: String = "".to_string();
        let _res = std::io::stdin().read_line(&mut inp_move);
        let re = Regex::new(r"^\d \d\n$").unwrap();
        if re.is_match(inp_move.as_str()) {
            let x: i32 = inp_move
                .chars()
                .next()
                .unwrap()
                .to_string()
                .parse()
                .unwrap();
            let y: i32 = inp_move
                .chars()
                .nth(2)
                .unwrap()
                .to_string()
                .parse()
                .unwrap();
            let field_ans = field.make_move(x, y, cur_turn);

            match field_ans {
                MoveStatus::Success => {
                    if cur_turn == CellStatus::Cross {
                        cur_turn = CellStatus::Zero;
                    } else {
                        cur_turn = CellStatus::Cross;
                    }
                    counter += 1;
                }
                MoveStatus::CellValError => {
                    println!("Эта клетка уже занята.");
                }
                MoveStatus::LimitError => {
                    println!("Индекс вне границ поля.");
                }
            }
        } else {
            println!("Неправильный формат ввода.");
        }

        print_field(&field);

        match field.check_win() {
            CellStatus::Cross => {
                println!("Крестики победили.");
                run = false;
            }
            CellStatus::Zero => {
                println!("Нолики победили.");
                run = false;
            }
            _ => {}
        }
        if counter == 9 {
            println!("Ничья.");
            run = false;
        }
    }
}
