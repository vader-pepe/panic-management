use raylib::prelude::*;
use std::collections::HashMap;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SceneID {
    Logo,
    MainMenu,
    MainLoop,
    Ending,
}

pub struct SceneManager {
    pub current_scene: Option<SceneID>,
    pub next_scene: Option<SceneID>,
    scenes: HashMap<SceneID, Box<dyn Scene>>, // Store scenes dynamically
    transition: Transition,
}

impl SceneManager {
    pub fn new() -> Self {
        let mut scenes = HashMap::new();
        scenes.insert(
            SceneID::Logo,
            Box::new(SceneMainMenu::new()) as Box<dyn Scene>,
        );
        scenes.insert(
            SceneID::MainMenu,
            Box::new(SceneLobby::new()) as Box<dyn Scene>,
        );

        SceneManager {
            current_scene: None,
            next_scene: None,
            scenes,
            transition: Transition::new(),
        }
    }

    pub fn init(&mut self) {
        self.push(SceneID::MainMenu);
    }

    pub fn update(&mut self, delta: f32) {
        if self.transition.direction != TransitionDirection::None {
            if self.transition.direction == TransitionDirection::Out {
                self.transition.opacity += 5;
                if self.transition.opacity >= 255 {
                    self.transition(TransitionDirection::In);
                }
            } else {
                self.transition.opacity -= 5;
                if self.transition.opacity <= 0 {
                    self.transition(TransitionDirection::None);
                }
            }
        }

        if let Some(scene_id) = self.current_scene {
            if let Some(scene) = self.scenes.get_mut(&scene_id) {
                scene.update(delta);
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if let Some(scene_id) = self.current_scene {
            if let Some(scene) = self.scenes.get(&scene_id) {
                scene.draw(d);
            }
        }

        if self.transition.direction != TransitionDirection::None {
            d.draw_rectangle(
                0,
                0,
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                Color::new(0, 0, 0, self.transition.opacity as u8),
            );
        }
    }

    pub fn push(&mut self, scene_id: SceneID) {
        self.next_scene = Some(scene_id);
        self.transition(if self.current_scene.is_some() {
            TransitionDirection::Out
        } else {
            TransitionDirection::In
        });
    }

    fn transition(&mut self, direction: TransitionDirection) {
        if direction == TransitionDirection::In {
            self.current_scene = self.next_scene;
            self.next_scene = None;
        }

        self.transition.direction = direction;

        match direction {
            TransitionDirection::Out => self.transition.opacity = 0,
            TransitionDirection::In => self.transition.opacity = 255,
            TransitionDirection::None => self.transition.opacity = -1,
        }
    }
}

pub trait Scene {
    fn new() -> Self
    where
        Self: Sized;
    fn update(&mut self, delta: f32);
    fn draw(&self, d: &mut RaylibDrawHandle);
}

struct SceneMainMenu;
impl Scene for SceneMainMenu {
    fn new() -> Self {
        SceneMainMenu
    }
    fn update(&mut self, _delta: f32) {}
    fn draw(&self, _d: &mut RaylibDrawHandle) {}
}

struct SceneLobby;
impl Scene for SceneLobby {
    fn new() -> Self {
        SceneLobby
    }
    fn update(&mut self, _delta: f32) {}
    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_text("This is Lobby", 0, 0, 24, Color::BLACK);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TransitionDirection {
    None,
    In,
    Out,
}

struct Transition {
    direction: TransitionDirection,
    opacity: i32,
}

impl Transition {
    fn new() -> Self {
        Transition {
            direction: TransitionDirection::None,
            opacity: 0,
        }
    }
}
