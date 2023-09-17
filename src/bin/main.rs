extern crate board_crate;

use board_crate::Board;
use rltk::{Rltk, GameState, RGB, VirtualKeyCode};

struct State 
{
    board: Board
}

impl State
{

}

impl GameState for State 
{
    fn tick(&mut self, ctx : &mut Rltk) 
    {
        ctx.cls();

        for x in 0..self.board.width
        {
            for y in 0..self.board.height
            {
                let glyph: u32;

                match self.board.get_cell(x as isize, y as isize)
                {
                    Some(state) => glyph = state,
                    None => glyph = ' ' as u32,
                }

                ctx.set(x, y, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), glyph);
            }
        }

        input(self, ctx);

        //let mouse_position = ctx.mouse_point();

    }
}

fn input(game_state: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key 
    {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::NumpadEnter => game_state.board.update_state(),
            VirtualKeyCode::R => game_state.board.random_state(0.1),
            VirtualKeyCode::C => game_state.board.set_cell(ctx.mouse_point().x as usize, ctx.mouse_point().y as usize, 1),
            VirtualKeyCode::E => game_state.board.dead_state(),
            VirtualKeyCode::G => game_state.board.gosper_glider(),
            _ => {}
        },
    }
}


fn main() -> rltk::BError 
{
    use rltk::RltkBuilder;
    /*let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;*/

    let mut context = RltkBuilder::simple(100, 100)
    .unwrap()
    .with_title("Roguelike Tutorial")
    .with_font("vga8x16.png", 8, 16)
    .with_sparse_console(80, 30, "vga8x16.png")
    .with_vsync(false)
    .build()?;

    let game_state = State
        { 
            board: Board::new(100, 100),
        };

    rltk::main_loop(context, game_state)
}
