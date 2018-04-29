// Copyright © 2018 Cormac O'Brien
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software
// and associated documentation files (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or
// substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING
// BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

pub mod console;
pub mod game;

use std::cell::RefCell;
use std::rc::Rc;

use common::console::Console;

use failure::Error;
use winit::WindowEvent;

use self::console::ConsoleInput;
use self::game::GameInput;

#[derive(Clone, Copy)]
pub enum InputFocus {
    Game,
    Console,
    Menu,
}

pub struct Input {
    current_focus: InputFocus,

    game_input: GameInput,
    console_input: ConsoleInput,
    // menu_input: MenuInput,
}

impl Input {
    pub fn new(init_focus: InputFocus, console: Rc<RefCell<Console>>) -> Input {
        Input {
            current_focus: init_focus,

            game_input: GameInput::new(console.clone()),
            console_input: ConsoleInput::new(console.clone()),
        }
    }

    pub fn handle_event(&self, event: WindowEvent) -> Result<(), Error> {
        match self.current_focus {
            InputFocus::Game => (),
            InputFocus::Console => self.console_input.handle_event(event)?,
            InputFocus::Menu => unimplemented!(),
        }

        Ok(())
    }

    pub fn set_focus(&mut self, new_focus: InputFocus) -> Result<(), Error> {
        self.current_focus = new_focus;

        Ok(())
    }
}


