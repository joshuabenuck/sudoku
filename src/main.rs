use glutin_window::GlutinWindow;
use graphics::character::CharacterCache;
use graphics::types::Color;
use graphics::{Context, Graphics};
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::GenericEvent;
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::collections::HashSet;

const SIZE: usize = 9;

pub struct Gameboard {
    pub cells: [[u8; SIZE]; SIZE],
    rng: ThreadRng,
}

impl Gameboard {
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
            rng: thread_rng(),
        }
    }

    /// Gets the character at cell location.
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
        Some(match self.cells[ind[1]][ind[0]] {
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => return None,
        })
    }

    /// Set cell value.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        self.cells[ind[1]][ind[0]] = val;
    }

    pub fn leftright(&self, x: usize, y: usize) -> HashSet<u8> {
        let mut leftright = HashSet::new();
        for i in 0..x {
            leftright.insert(self.cells[y][i]);
        }
        for i in (x + 1)..9 {
            leftright.insert(self.cells[y][i]);
        }
        leftright
    }

    pub fn updown(&self, x: usize, y: usize) -> HashSet<u8> {
        let mut updown = HashSet::new();
        for j in 0..y {
            updown.insert(self.cells[j][x]);
        }
        for j in (y + 1)..9 {
            updown.insert(self.cells[j][x]);
        }
        updown
    }

    pub fn inbox(&self, x: usize, y: usize) -> HashSet<u8> {
        let mut inbox = HashSet::new();
        let grid_x = x / 3;
        let grid_y = y / 3;
        for j in grid_y * 3..(grid_y + 1) * 3 {
            for i in grid_x * 3..(grid_x + 1) * 3 {
                inbox.insert(self.cells[j][i]);
            }
        }
        inbox
    }

    fn fullset(&self) -> HashSet<u8> {
        let mut fullset = HashSet::new();
        fullset.insert(1);
        fullset.insert(2);
        fullset.insert(3);
        fullset.insert(4);
        fullset.insert(5);
        fullset.insert(6);
        fullset.insert(7);
        fullset.insert(8);
        fullset.insert(9);
        fullset
    }

    pub fn populate(&mut self) {
        let mut seed = Vec::new();
        let set = self.fullset();
        let mut possibilities: Vec<u8> = set.into_iter().collect();
        for _ in 0..9 {
            let index = self.rng.gen_range(0, possibilities.len());
            seed.push(possibilities.remove(index));
        }
        // https://gamedev.stackexchange.com/questions/56149/how-can-i-generate-sudoku-puzzles
        let indexes = vec![
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8],
            vec![3, 4, 5, 6, 7, 8, 0, 1, 2], // shift 3
            vec![6, 7, 8, 0, 1, 2, 3, 4, 5], // shift 3
            vec![7, 8, 0, 1, 2, 3, 4, 5, 6], // shift 1
            vec![1, 2, 3, 4, 5, 6, 7, 8, 0], // shift 3
            vec![4, 5, 6, 7, 8, 0, 1, 2, 3], // shift 3
            vec![5, 6, 7, 8, 0, 1, 2, 3, 4], // shift 1
            vec![8, 0, 1, 2, 3, 4, 5, 6, 7], // shift 3
            vec![2, 3, 4, 5, 6, 7, 8, 0, 1], // shift 3
        ];
        for j in 0..9 {
            for i in 0..9 {
                self.cells[j][i] = seed[indexes[j][i]];
            }
        }
    }

    pub fn solved(&self) -> bool {
        false
    }
}

pub struct GameboardController {
    pub gameboard: Gameboard,
    pub selected_cell: Option<[usize; 2]>,
    cursor_pos: [f64; 2],
}

impl GameboardController {
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        use piston::input::{Button, Key, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x <= size && y >= 0.0 && y <= size {
                // Compute the cell position.
                let cell_x = (x / size * 9.0) as usize;
                let cell_y = (y / size * 9.0) as usize;
                self.selected_cell = Some([cell_x, cell_y]);
            }
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if let Some(ind) = self.selected_cell {
                // Set cell value.
                match key {
                    Key::D1 => self.gameboard.set(ind, 1),
                    Key::D2 => self.gameboard.set(ind, 2),
                    Key::D3 => self.gameboard.set(ind, 3),
                    Key::D4 => self.gameboard.set(ind, 4),
                    Key::D5 => self.gameboard.set(ind, 5),
                    Key::D6 => self.gameboard.set(ind, 6),
                    Key::D7 => self.gameboard.set(ind, 7),
                    Key::D8 => self.gameboard.set(ind, 8),
                    Key::D9 => self.gameboard.set(ind, 9),
                    _ => {}
                }
            }
        }
    }
}

pub struct GameboardViewSettings {
    pub position: [f64; 2],
    pub size: f64,
    pub background_color: Color,
    pub border_color: Color,
    pub board_edge_color: Color,
    pub section_edge_color: Color,
    pub cell_edge_color: Color,
    pub board_edge_radius: f64,
    pub section_edge_radius: f64,
    pub cell_edge_radius: f64,
    pub selected_cell_background_color: Color,
    pub text_color: Color,
}

impl GameboardViewSettings {
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0; 2],
            size: 400.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
            text_color: [0.0, 0.0, 0.1, 1.0],
        }
    }
}

pub struct GameboardView {
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView { settings }
    }

    pub fn draw<G: Graphics, C>(
        &self,
        controller: &GameboardController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
    ) where
        C: CharacterCache<Texture = G::Texture>,
    {
        use graphics::{Image, Line, Rectangle, Transformed};

        let ref settings = self.settings;
        let board_rect = [
            settings.position[0],
            settings.position[1],
            settings.size,
            settings.size,
        ];

        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        if let Some(ind) = controller.selected_cell {
            let cell_size = settings.size / 9.0;
            let pos = [ind[0] as f64 * cell_size, ind[1] as f64 * cell_size];
            let cell_rect = [
                settings.position[0] + pos[0],
                settings.position[1] + pos[1],
                cell_size,
                cell_size,
            ];
            Rectangle::new(settings.selected_cell_background_color).draw(
                cell_rect,
                &c.draw_state,
                c.transform,
                g,
            );
        }

        // Draw characters.
        let text_image = Image::new_color(settings.text_color);
        let cell_size = settings.size / 9.0;
        for j in 0..9 {
            for i in 0..9 {
                if let Some(ch) = controller.gameboard.char([i, j]) {
                    let pos = [
                        settings.position[0] + i as f64 * cell_size + 15.0,
                        settings.position[1] + j as f64 * cell_size + 34.0,
                    ];
                    if let Ok(character) = glyphs.character(34, ch) {
                        let ch_x = pos[0] + character.left();
                        let ch_y = pos[1] - character.top();
                        let text_image = text_image.src_rect([
                            character.atlas_offset[0],
                            character.atlas_offset[1],
                            character.atlas_size[0],
                            character.atlas_size[1],
                        ]);
                        text_image.draw(
                            character.texture,
                            &c.draw_state,
                            c.transform.trans(ch_x, ch_y),
                            g,
                        );
                    }
                }
            }
        }

        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        for i in 0..9 {
            if (i % 3) == 0 {
                continue;
            }

            let x = settings.position[0] + i as f64 / 9.0 * settings.size;
            let y = settings.position[1] + i as f64 / 9.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            cell_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            cell_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);
        for i in 0..3 {
            let x = settings.position[0] + i as f64 / 3.0 * settings.size;
            let y = settings.position[1] + i as f64 / 3.0 * settings.size;
            let x2 = settings.position[0] + settings.size;
            let y2 = settings.position[1] + settings.size;

            let vline = [x, settings.position[1], x, y2];
            section_edge.draw(vline, &c.draw_state, c.transform, g);

            let hline = [settings.position[0], y, x2, y];
            section_edge.draw(hline, &c.draw_state, c.transform, g);
        }

        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius).draw(
            board_rect,
            &c.draw_state,
            c.transform,
            g,
        );
    }
}

fn main() {
    let mut settings = EventSettings::new();
    settings.set_lazy(true);
    settings.swap_buffers(true);
    settings.max_fps(1);
    settings.ups(1);
    let mut events = Events::new(settings);
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [512; 2])
        .exit_on_esc(true)
        .graphics_api(opengl);
    let mut window: GlutinWindow = settings.build().expect("Could not create window");
    let mut gl = GlGraphics::new(opengl);

    let mut gameboard = Gameboard::new();
    gameboard.populate();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");
    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(
            gameboard_view.settings.position,
            gameboard_view.settings.size,
            &e,
        );
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::clear;
                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, glyphs, &c, g);
            });
        }
    }
}
