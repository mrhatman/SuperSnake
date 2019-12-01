use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        transform::{TransformBundle,Transform},
        math::{Vector3 ,Point3,Point2},Time
    },
    ecs::prelude::*,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle, ImageFormat,sprite::SpriteSheetHandle,
        Texture,SpriteSheet,SpriteSheetFormat,Camera,ActiveCamera
    },
    input::{InputBundle,InputHandler,StringBindings,VirtualKeyCode},
    tiles::{TileMap,Tile, RenderTiles2D,MortonEncoder},
    utils::application_root_dir,
};
use std::collections::VecDeque;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources_dir = app_root.join("resources\\");

    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<SnakeGameTile, MortonEncoder>::default()),
        )?
        .with_bundle(InputBundle::<StringBindings>::new())?
        .with_bundle(TransformBundle::new())?
        .with(DirectionChangeSystem{},"Direction Change",&[])
        .with(MoveSystem::default(),"Move system",&[]);

    let mut game = Application::new(resources_dir, SnakeState, game_data)?;
    game.run();

    Ok(())
}

struct SnakeState;

impl SimpleState for SnakeState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {

        let world = data.world;
        initialise_camera(world);

        let tile_sprite_sheet = load_sprite_sheet(world , "Tile.png","Tile.ron");

        let map = TileMap::<SnakeGameTile, MortonEncoder>::new(
            Vector3::new(40, 40, 2),
            Vector3::new(32, 32, 2),
            Some(tile_sprite_sheet),
        );

        world.create_entity()
            .with(map)
            .with(Transform::default())
            .build();

        world.insert(Snake::default());
    }
}

fn load_sprite_sheet(world: &mut World, png_path: &str, ron_path: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(png_path, ImageFormat::default(), (), &texture_storage)
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    let cam = world
        .create_entity()
        .with(Camera::standard_2d(1280.0, 1280.0))
        .with(transform)
        .build();

    let mut act_cam = world.write_resource::<ActiveCamera>();
    (*act_cam).entity = Some(cam);
}

enum Direction{
    Up,
    Down,
    Left,
    Right
}

struct Snake{
    snake : VecDeque<Point2<u32>>,
    direction : Direction,
    points_to_add : u32,
}

impl Default for Snake {
    fn default() -> Self {

        let mut snake = VecDeque::new();
        snake.push_back(Point2::new(20,20));
        snake.push_back(Point2::new(20,21));
        snake.push_back(Point2::new(20,22));
        snake.push_back(Point2::new(20,23));
        Snake{snake,direction: Direction::Up,points_to_add: 10}

    }
}


#[derive(Default, Clone)]
struct SnakeGameTile;
impl Tile for SnakeGameTile {
    fn sprite(&self, point : Point3<u32>, world : &World) -> Option<usize> {
        if point.z == 0 {
            Some(0)
        }
        else
        {
            let snake  = world.fetch::<Snake>();

            let pos = snake.snake.iter().enumerate().find(|&(_,p)| p.x == point.x && p.y == point.y).map(|(loc,_)| loc);

            if let Some(p) = pos{
                if p == 0{
                    Some(1)
                }
                else{
                    Some(2)
                }

            }
            else{
                None
            }
        }
    }
}

struct DirectionChangeSystem {}

impl<'s> System<'s> for DirectionChangeSystem {
    type SystemData = (
        WriteExpect<'s, Snake>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut snake, input): Self::SystemData) {
        if input.key_is_down(VirtualKeyCode::Up) {
            let new_point = Point2::new(snake.snake.get(0).unwrap().x ,snake.snake.get(0).unwrap().y -1);
            if new_point != *snake.snake.get(1).unwrap(){
                snake.direction = Direction::Up;
            }
        }
        if input.key_is_down(VirtualKeyCode::Down) {
            let new_point = Point2::new(snake.snake.get(0).unwrap().x ,snake.snake.get(0).unwrap().y +1);
            if new_point != *snake.snake.get(1).unwrap(){
                snake.direction = Direction::Down;
            }
        }
        if input.key_is_down(VirtualKeyCode::Left) {
            let new_point = Point2::new(snake.snake.get(0).unwrap().x -1,snake.snake.get(0).unwrap().y);
            if new_point != *snake.snake.get(1).unwrap(){
                snake.direction = Direction::Left;
            }
        }
        if input.key_is_down(VirtualKeyCode::Right) {
            let new_point = Point2::new(snake.snake.get(0).unwrap().x +1,snake.snake.get(0).unwrap().y );
            if new_point != *snake.snake.get(1).unwrap(){
                snake.direction = Direction::Right;
            }
        }

    }
}

struct MoveSystem {
    time_remainder_sec : f32,
}

impl Default for MoveSystem {
    fn default() -> Self {
        Self {
            time_remainder_sec : 0.0
        }
    }
}

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteExpect<'s, Snake>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut snake, time): Self::SystemData) {
        self.time_remainder_sec += time.delta_seconds();

        if self.time_remainder_sec > 0.05{
            self.time_remainder_sec -= 0.05;

            //Move snake
            let new_point = match snake.direction{
                Direction::Up =>{
                    Point2::new(snake.snake.get(0).unwrap().x ,snake.snake.get(0).unwrap().y -1)
                }
                Direction::Down =>{
                   Point2::new(snake.snake.get(0).unwrap().x ,snake.snake.get(0).unwrap().y +1)
                }
                Direction::Left =>{
                    Point2::new(snake.snake.get(0).unwrap().x -1,snake.snake.get(0).unwrap().y )

                }
                Direction::Right =>{
                    Point2::new(snake.snake.get(0).unwrap().x +1,snake.snake.get(0).unwrap().y )

                }
            };

            snake.snake.push_front(new_point);
            if snake.points_to_add >0 {
                snake.points_to_add -=1;

            }
            else{
                snake.snake.pop_back();
            }
        }

    }
}