use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;
use web_sys::{TouchEvent, TouchList};
use yew::events::KeyboardEvent;
use yew::prelude::*;

use crate::grid::{Grid, Move};

//Test
const TOUCH_MOVE_THRESHOLD: i32 = 30;

fn get_color_for_cell(value: u64) -> &'static str {
    match value {
        0 => "rgba(238, 228, 218, 0.35)",
        2 => "#eee4da",
        4 => "#ede0c8",
        8 => "#f2b179",
        16 => "#f59563",
        32 => "#f67c5f",
        64 => "#f65e3b",
        128 => "#edcf72",
        256 => "#edcc61",
        512 => "#edc850",
        1024 => "#edc53f",
        2048 => "#edc22e",
        _ => "#3c3a32",
    }
}

fn get_color_for_text(value: u64) -> &'static str {
    match value {
        2 => "#6c6462",
        4 => "#6c6462",
        _ => "#FFFFFF",
    }
}

pub enum Msg {
    KeyDown(KeyboardEvent),
    TouchStart(TouchEvent),
    TouchMove(TouchEvent),
    TouchEnd(TouchEvent),
    NewGame,
}

fn get_move(dx: i32, dy: i32) -> Move {
    if dx.abs() > dy.abs() {
        if dx.is_positive() {
            Move::Right
        } else {
            Move::Left
        }
    } else {
        if dy.is_positive() {
            Move::Down
        } else {
            Move::Up
        }
    }
}

pub struct Model {
    grid: Grid,
    grid_node: NodeRef,
    touch_start_x: Option<i32>,
    touch_start_y: Option<i32>,
}

impl Model {
    fn view_row(&self, (y, row): (usize, &[u64; 4])) -> Html {
        html! {
            <div class="square-row">
                { for row.iter().enumerate().map(|(x, cell)| self.view_cell(*cell, x, y)) }
            </div>
        }
    }

    fn view_cell(&self, cell: u64, x: usize, y: usize) -> Html {
        let background_color = format!("background-color:{};", get_color_for_cell(cell));
        let position_top = format!("top:{}px;", y * 100); // Adjust this value based on your grid cell size
        let position_left = format!("left:{}px;", x * (100 + 7)); // Adjust this value based on your grid cell size
        let style = format!("{}{}{}", background_color, position_top, position_left);
        let cell_text = match cell {
            0 => "".to_string(),
            _ => cell.to_string(),
        };
        let text_color = get_color_for_text(cell);
        let text_style = format!("color:{};", text_color);
        html! {
            <div class="square" style={style}>
                <span class="square-number" style={text_style}>{ cell_text }</span>
            </div>
        }
    }

    fn make_move(&mut self, mov: Move) {
        self.grid.attempt(mov);
    }
}

impl Component for Model {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let model = Model {
            grid: Grid::default(),
            grid_node: NodeRef::default(),
            touch_start_x: None,
            touch_start_y: None,
        };

        let grid_node = model.grid_node.clone();
        let closure = Closure::wrap(Box::new(move || {
            if let Some(grid) = grid_node.cast::<HtmlDivElement>() {
                grid.focus().unwrap();
            }
        }) as Box<dyn FnMut()>);
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                10,
            )
            .unwrap();
        closure.forget();

        model
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game_over_popup = if self.grid.has_player_lost() {
            html! {
                <div class="game-over-popup">
                    <div class="game-over-content">
                        <h2>{ "Game Over" }</h2>
                        <button onclick={ctx.link().callback(|_| Msg::NewGame)}>{ "New Game" }</button>
                    </div>
                </div>
            }
        } else {
            html! {}
        };

        html! {
            <>
            <div class="scoreboard">
                <div class="score-container"> // Add this wrapper div
                    <h2>{ "Score" }</h2>
                    <p>{ self.grid.get_score() }</p>
                </div>
                <button onclick={ctx.link().callback(|_| Msg::NewGame)}>{ "New Game" }</button>
            </div>
            <div class="grid disable-scroll" tabindex="0" ref={self.grid_node.clone()}
            onkeydown={ctx.link().callback(|event| Msg::KeyDown(event))}
            ontouchstart={ctx.link().callback(|event| Msg::TouchStart(event))}
            ontouchmove={ctx.link().callback(|event| Msg::TouchMove(event))}
            ontouchend={ctx.link().callback(|event| Msg::TouchEnd(event))}
            >
            <section class="section">
                <div class="container">
                    <div class="vcenter">
                        <div class="board">
                            <div class="square-grid">
                                { for self.grid.cells.iter().enumerate().map(|(y, row)| self.view_row((y, row))) }
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
        { game_over_popup }
        </>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyDown(event) => {
                let key_code = event.key_code();
                let arrow = match key_code {
                    37 => Some(Move::Left),
                    38 => Some(Move::Up),
                    39 => Some(Move::Right),
                    40 => Some(Move::Down),
                    _ => None,
                };
                if let Some(a) = arrow {
                    self.make_move(a);
                }

                true
            }
            Msg::TouchStart(event) => {
                let touches: TouchList = event.target_touches();
                if touches.length() > 0 {
                    let touch = touches.item(0).expect("No touch found");
                    self.touch_start_x = Some(touch.client_x());
                    self.touch_start_y = Some(touch.client_y());
                }
                true
            }
            Msg::TouchMove(_) => true,
            Msg::TouchEnd(event) => {
                if self.touch_start_x.is_none() || self.touch_start_y.is_none() {
                    return false;
                }
                let x_start = self.touch_start_x.unwrap();
                let y_start = self.touch_start_y.unwrap();

                let changed_touches: TouchList = event.changed_touches();
                if changed_touches.length() == 0 {
                    return false;
                }
                // Can't be empty because we checked;
                let touch = changed_touches.item(0).unwrap();
                let dx = touch.client_x() - x_start;
                let dy = touch.client_y() - y_start;
                if dx.abs() < TOUCH_MOVE_THRESHOLD && dy.abs() < TOUCH_MOVE_THRESHOLD {
                    return false;
                }
                let mov = get_move(dx, dy);
                self.grid.attempt(mov);

                self.touch_start_x = None;
                self.touch_start_y = None;

                true
            }
            Msg::NewGame => {
                self.grid = Grid::default();
                true
            }
        }
    }
}
