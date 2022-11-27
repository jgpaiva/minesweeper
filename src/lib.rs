#![recursion_limit = "512"]

use lib_minesweeper::create_board;
use lib_minesweeper::numbers_on_board;
use lib_minesweeper::Board;
use lib_minesweeper::BoardState;
use lib_minesweeper::BoardState::Failed;
use lib_minesweeper::BoardState::NotReady;
use lib_minesweeper::BoardState::Playing;
use lib_minesweeper::BoardState::Ready;
use lib_minesweeper::BoardState::Won;
use lib_minesweeper::MapElement;
use lib_minesweeper::MapElement::Mine;
use lib_minesweeper::MapElement::Number;
use lib_minesweeper::MapElementCellState::Closed;
use lib_minesweeper::MapElementCellState::Flagged;
use lib_minesweeper::MapElementCellState::Open;
use lib_minesweeper::Point;

use wasm_bindgen::prelude::*;

use serde_derive::{Deserialize, Serialize};
//use yew::format::Json;
use yew::prelude::*;

use js_sys::Date;

//use yew::services::storage::{Area, StorageService};

fn small_board() -> Board {
    use rand::Rng;
    let width = 10;
    let height = 10;
    let mines = 10;

    let board = create_board(width, height, mines, |x, y| {
        rand::thread_rng().gen_range(x..y)
    });

    numbers_on_board(board)
}

fn medium_board() -> Board {
    use rand::Rng;
    let width = 16;
    let height = 16;
    let mines = 40;

    let board = create_board(width, height, mines, |x, y| {
        rand::thread_rng().gen_range(x..y)
    });

    numbers_on_board(board)
}

fn large_board() -> Board {
    use rand::Rng;
    let width = 16;
    let height = 30;
    let mines = 99;

    let board = create_board(width, height, mines, |x, y| {
        rand::thread_rng().gen_range(x..y)
    });

    numbers_on_board(board)
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum Mode {
    Flagging,
    Digging,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum Difficulty {
    Easy,
    Medium,
    Hard,
}
struct Model {
    //storage: StorageService,
    state: State,
}

enum Msg {
    ToggleDifficulty,
    ToggleMode,
    UpdateBoard { point: Point },
    RunRobot,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct State {
    difficulty: Difficulty,
    mode: Mode,
    board: Board,
}

//const KEY: &'static str = "jgpaiva.minesweeper.self";

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: &Context<Self>) -> Self {
        //let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        //        let difficulty = {
        //            if let Json(Ok(restored_model)) = storage.restore(KEY) {
        //                restored_model
        //            } else {
        //
        //            }
        //        };
        let state = State {
            difficulty: Difficulty::Easy,
            mode: Mode::Digging,
            board: small_board(),
        };
        Self {
            //storage,
            state,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleDifficulty => self.toggle_difficulty(),
            Msg::ToggleMode => self.toggle_mode(),
            Msg::UpdateBoard { point } => self.update_board(point),
            Msg::RunRobot => self.run_robot(),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={self.render_main_container_class()}>
                <div id="difficulty_button_placeholder" class="flex-container">
                    <div
                     id="difficulty-button"
                     class="clickable item"
                     onclick={ctx.link().callback(|_| Msg::ToggleDifficulty)} >
                        { self.render_difficulty() }
                    </div>
                    <div
                     id="mode-button"
                     class={self.render_mode_class()}
                     onclick={ctx.link().callback(|_| Msg::ToggleMode)} >
                        { self.render_mode() }
                    </div>
                    <div
                     id="robot-button"
                     class={self.render_mode_class()}
                     onclick={ctx.link().callback(|_| Msg::RunRobot)} >
                        { self.render_robot()}
                    </div>
                    <TimeKeeper op={
                        match self.state.board.state {
                            Won => TimeKeeperOp::Stopped,
                            Failed => TimeKeeperOp::Stopped,
                            Playing => TimeKeeperOp::Counting,
                            Ready => TimeKeeperOp::Reset,
                            NotReady => unreachable!(),
                        }}/>
                </div>
                <div id="board_game_placeholder">
                    <div id="board_game" class="flex-container">
                        {
                            (0..self.state.board.height)
                                .flat_map(|y| {
                                                (0..self.state.board.width+1).map(move |x| {
                                                    if x == self.state.board.width{
                                                        self.render_break()
                                                    } else {
                                                        let board = &self.state.board;
                                                        html!{
                                                            <BoardItem
                                                                x={x}
                                                                y={y}
                                                                board_state={board.state.clone()}
                                                                board_width={board.width}
                                                                element={board.at(&Point::new(x,y)).unwrap().clone()}
                                                                update_signal={ctx.link().callback(|msg:Msg| msg)}/>
                                                        }
                                                    }
                                                })
                                }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        }
    }
}

impl Model {
    fn toggle_difficulty(&mut self) {
        let (new_board, new_difficulty) = match (
            self.state.board.state.clone(),
            self.state.difficulty.clone(),
        ) {
            (Ready, Difficulty::Easy) => (medium_board(), Difficulty::Medium),
            (Ready, Difficulty::Medium) => (large_board(), Difficulty::Hard),
            (Ready, Difficulty::Hard) => (small_board(), Difficulty::Easy),
            (_, Difficulty::Easy) => (small_board(), Difficulty::Easy),
            (_, Difficulty::Medium) => (medium_board(), Difficulty::Medium),
            (_, Difficulty::Hard) => (large_board(), Difficulty::Hard),
        };
        self.state = State {
            difficulty: new_difficulty,
            board: new_board,
            ..self.state.clone()
        }
    }
    fn toggle_mode(&mut self) {
        if matches!(self.state.board.state, Won | Failed) {
            return;
        }
        self.state = match self.state.mode {
            Mode::Digging => State {
                mode: Mode::Flagging,
                ..self.state.clone()
            },
            Mode::Flagging => State {
                mode: Mode::Digging,
                ..self.state.clone()
            },
        }
    }

    fn render_main_container_class(&self) -> String {
        match self.state.board.state {
            Ready | Playing => "ongoing",
            Won => "won",
            Failed => "failed",
            NotReady => unreachable!(),
        }
        .into()
    }

    fn render_difficulty(&self) -> Html {
        html! {
            match self.state.difficulty {
                Difficulty::Easy => "😀",
                Difficulty::Medium => "🤨",
                Difficulty::Hard => "🧐",
            }
        }
    }

    fn render_mode_class(&self) -> String {
        match &self.state.board.state {
            Won | Failed => "item".into(),
            _ => "clickable item".into(),
        }
    }

    fn render_mode(&self) -> String {
        match (&self.state.board.state, self.state.mode.clone()) {
            (Ready, Mode::Flagging) | (Playing, Mode::Flagging) => "🚩",
            (Ready, Mode::Digging) | (Playing, Mode::Digging) => "⛏️",
            (Won, _) => "🏆",
            (Failed, _) => "☠️",
            _ => unreachable!(),
        }
        .into()
    }

    fn render_robot(&self) -> &str {
        if matches!(&self.state.board.state, Ready | Playing) {
            "🤖"
        } else {
            ""
        }
    }

    fn render_break(&self) -> Html {
        html! {
            <div class="break">
            </div>
        }
    }

    fn update_board(&mut self, p: Point) {
        match self.state.mode {
            Mode::Digging => {
                let new_board = self.state.board.cascade_open_item(&p);
                if let Some(b) = new_board {
                    self.state.board = b
                }
            }
            Mode::Flagging => {
                self.state.board = self.state.board.flag_item(&p);
            }
        }
    }

    fn run_robot(&mut self) {
        if matches!(self.state.board.state, Won | Failed) {
            return;
        }
        let board = &self.state.board;
        for x in 0..board.width {
            for y in 0..board.height {
                let p = Point::new(x, y);
                if let Some(board) = Self::run_robot_on_point(board, p) {
                    self.state.board = board;
                    return;
                }
            }
        }
    }

    fn run_robot_on_point(board: &Board, p: Point) -> Option<Board> {
        let el = board.at(&p).unwrap();
        let Number {
            state: Open,
            count: mine_count,
        } = el else {
            return None
        };
        if *mine_count == 0 {
            return None;
        }
        let surrounding_points = board.surrounding_points(&p);
        let surrounding_els: Vec<_> = surrounding_points
            .iter()
            .map(|p| (p, board.at(p).unwrap().clone()))
            .filter(|(_p, el)| {
                !matches!(
                    el,
                    Number {
                        state: Open,
                        count: 0
                    }
                )
            })
            .collect();
        let mut unopened = surrounding_els
            .iter()
            .filter(|(_p, el)| !matches!(el, Number { state: Open, .. }));
        let flagged_count = surrounding_els
            .iter()
            .filter(|(_p, el)| {
                matches!(el, Mine { state: Flagged } | Number { state: Flagged, .. })
            })
            .count();
        let unopened_count = unopened.clone().count();
        if *mine_count == unopened_count as i32 && flagged_count < unopened_count {
            let (p, _el) = unopened
                .find(|(_p, el)| {
                    !matches!(el, Mine { state: Flagged } | Number { state: Flagged, .. })
                })
                .unwrap();
            return Some(board.flag_item(p));
        }
        if *mine_count == flagged_count as i32 && unopened_count - flagged_count > 0 {
            let (p, _el) = unopened
                .find(|(_p, el)| {
                    !matches!(el, Mine { state: Flagged } | Number { state: Flagged, .. })
                })
                .unwrap();
            return board.cascade_open_item(p);
        }
        None
    }
}

#[derive(Copy, Clone, Properties, PartialEq)]
struct TimeKeeperProps {
    op: TimeKeeperOp,
}

#[derive(Copy, Clone, PartialEq)]
enum TimeKeeperOp {
    Reset,
    Counting,
    Stopped,
}

struct TimeKeeperState {
    started_at: Option<Date>,
    stopped_at: Option<Date>,
    _handle: gloo_timers::callback::Interval,
}

struct TimeKeeper {
    state: TimeKeeperState,
}

enum TimeKeeperMsg {
    Tick,
}

impl Component for TimeKeeper {
    type Message = TimeKeeperMsg;
    type Properties = TimeKeeperProps;
    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let _handle = gloo_timers::callback::Interval::new(100, move || {
            link.send_message(TimeKeeperMsg::Tick)
        });

        let state = TimeKeeperState {
            started_at: None,
            stopped_at: None,
            _handle,
        };
        Self { state }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let props = ctx.props();
        match (old_props.op, props.op) {
            (TimeKeeperOp::Counting, TimeKeeperOp::Reset)
            | (TimeKeeperOp::Stopped, TimeKeeperOp::Reset) => {
                self.state.started_at = None;
                self.state.stopped_at = None;
                true
            }
            (TimeKeeperOp::Reset, TimeKeeperOp::Reset) => false,
            (TimeKeeperOp::Stopped, TimeKeeperOp::Counting)
            | (TimeKeeperOp::Reset, TimeKeeperOp::Counting) => {
                self.state.started_at = Some(Date::new_0());
                true
            }
            (TimeKeeperOp::Counting, TimeKeeperOp::Counting) => true,
            (TimeKeeperOp::Counting, TimeKeeperOp::Stopped) => {
                self.state.stopped_at = Some(Date::new_0());
                true
            }
            (TimeKeeperOp::Reset, TimeKeeperOp::Stopped) => {
                self.state.started_at = Some(Date::new_0());
                self.state.stopped_at = Some(Date::new_0());
                true
            }
            (TimeKeeperOp::Stopped, TimeKeeperOp::Stopped) => false,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id = "time_container" class= "item not-clickable">
                <p> { self.render_timer() } </p>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TimeKeeperMsg::Tick => {}
        }
        true
    }
}

impl TimeKeeper {
    fn render_timer(&self) -> String {
        match (&self.state.started_at, &self.state.stopped_at) {
            (Some(started_at), None) => {
                let now = Date::new_0();
                format!(
                    "{}",
                    ((now.get_time() - started_at.get_time()) / 1000_f64)
                        .round()
                        .min(999_f64) // make sure we don't run out of space
                )
            }
            (Some(started_at), Some(stopped_at)) => format!(
                "{}",
                ((stopped_at.get_time() - started_at.get_time()) / 1000_f64)
                    .round()
                    .min(999_f64) // make sure we don't run out of space
            ),
            (None, None) => String::from("0"),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Properties, PartialEq)]
struct BoardItemProps {
    x: usize,
    y: usize,
    board_state: BoardState,
    board_width: usize,
    element: MapElement,
    update_signal: Callback<Msg>,
}

struct BoardItem {}

impl Component for BoardItem {
    type Message = Msg;
    type Properties = BoardItemProps;
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let props = ctx.props();
        !(old_props.x == props.x
            && old_props.y == props.y
            && old_props.board_state == props.board_state
            && old_props.board_width == props.board_width
            && old_props.element == props.element)
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateBoard { point } => {
                ctx.props().update_signal.emit(Msg::UpdateBoard { point })
            }
            _ => unreachable!(),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let x = props.x;
        let y = props.y;
        html! {
            <div
             class={
                 match(&props.board_state, &props.element) {
                     (Ready, Number { state: Closed, .. })
                         | (Ready, Mine { state: Closed, .. })
                         | (Playing, Number { state: Closed, .. })
                         | (Playing, Mine { state: Closed, .. }) => {
                             String::from("item clickable2")
                         },
                     (Playing, Number {state: Open, count})
                         | (Won,Number {count, ..})
                         | (Failed,Number {count, ..}) => {
                         format!("item not-clickable2 mines-{}", count)
                     },
                     _ => String::from("item not-clickable2")
             }}
                style={self.get_item_style(ctx.props().board_width)}
                onclick={ctx.link().callback(move |_| {Msg::UpdateBoard {point:Point::new(x,y)}})} >
                <div style="width:100%; text-align:center"> {
                    match (&props.board_state, &props.element) {
                        (Ready, Number { state: Flagged, .. })
                            | (Ready, Mine { state: Flagged, .. })
                            | (Playing, Number { state: Flagged, .. })
                            | (Playing, Mine { state: Flagged, .. }) => {
                                String::from("🚩")
                            }
                        (Ready, Number { state: Closed, .. })
                            | (Ready, Mine { state: Closed, .. })
                            | (Playing, Number { state: Closed, .. })
                            | (Playing, Mine { state: Closed, .. }) => {
                                String::from("❓")
                            }
                        (_, Number { count:0, .. }) => String::from(""),
                        (_, Number { count, .. }) => format!("{}",count),
                        (Failed, Mine { .. }) => String::from("💣"),
                        (Won, Mine { .. }) => String::from("🚩"),
                        _ => unreachable!(),
                    }
                }
            </div>
        </div>
        }
    }
}

impl BoardItem {
    fn get_item_style(&self, board_width: usize) -> String {
        let square_size: f64 = 100.0 / (board_width as f64);
        let margin: f64 = 0.05 * square_size;
        let width = format!("{:.2}", square_size - 2.0 * margin);

        format!("width: {}%; margin: {}%", width, margin)
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    yew::Renderer::<Model>::default().render();

    gloo::console::log!("App initialized");
    Ok(())
}
