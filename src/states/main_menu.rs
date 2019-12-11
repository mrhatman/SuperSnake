use amethyst::{
    assets::Loader,
    prelude::*,
    core::{
        ecs::prelude::*,
        Time,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform,UiButtonBuilder, UiButton,UiEvent,UiEventType},
};
use crate::states::{LoadState, CreditsState, SettingsState};

pub struct MainMenuState{
    text_entity: Option<Entity>,
    play_button_entity: Option<UiButton>,
    load_button_entity: Option<UiButton>,
    credits_button_entity: Option<UiButton>,
    settings_button_entity: Option<UiButton>,
}

impl MainMenuState{
    pub fn new() -> Self {
        MainMenuState{text_entity: None,play_button_entity:None,load_button_entity:None,credits_button_entity:None,settings_button_entity:None,}
    }
}

impl SimpleState for MainMenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        //Setup UI
        let font = world.read_resource::<Loader>().load(
            "Poppins-Black.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        let big_text_transform = UiTransform::new(
            "Amethyst Text".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            500.,
            1.,
            2000.,
            50.,
        );
        self.text_entity = Some(world
            .create_entity()
            .with(big_text_transform)
            .with(UiText::new(
                font.clone(),
                "Super Snake".to_string(),
                [1.0, 1.0, 1.0, 1.0],
                50.,
            ))
            .build());

        self.play_button_entity = Some(UiButtonBuilder::<(), u32>::new("Play")
            .with_size(200.0, 36.0)
            .with_anchor(Anchor::Middle)
            .with_font(font.clone())
            .with_id(0)
            .with_position(0.0, 200.0)
            .with_font_size(24.0f32)
            .with_text_color([1.0f32, 1.0, 1.0, 1.0])
            .with_hover_text_color([1.0f32, 0.0f32, 0.0f32, 1.0f32])
            .build_from_world(&world).1);

        self.load_button_entity = Some(UiButtonBuilder::<(), u32>::new("Load")
            .with_size(200.0, 36.0)
            .with_anchor(Anchor::Middle)
            .with_font(font.clone())
            .with_id(1)
            .with_position(0.0, 100.0)
            .with_font_size(24.0f32)
            .with_text_color([1.0f32, 1.0, 1.0, 1.0])
            .with_hover_text_color([1.0f32, 0.0f32, 0.0f32, 1.0f32])
            .build_from_world(&world).1);

        self.settings_button_entity = Some(UiButtonBuilder::<(), u32>::new("Settings")
            .with_size(200.0, 36.0)
            .with_anchor(Anchor::Middle)
            .with_font(font.clone())
            .with_id(2)
            .with_position(0.0, 000.0)
            .with_font_size(24.0f32)
            .with_text_color([1.0f32, 1.0, 1.0, 1.0])
            .with_hover_text_color([1.0f32, 0.0f32, 0.0f32, 1.0f32])
            .build_from_world(&world).1);

        self.credits_button_entity = Some(UiButtonBuilder::<(), u32>::new("Load")
            .with_size(200.0, 36.0)
            .with_anchor(Anchor::Middle)
            .with_font(font.clone())
            .with_id(3)
            .with_position(0.0, -100.0)
            .with_font_size(24.0f32)
            .with_text_color([1.0f32, 1.0, 1.0, 1.0])
            .with_hover_text_color([1.0f32, 0.0f32, 0.0f32, 1.0f32])
            .build_from_world(&world).1);



    }

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        match event {
            StateEvent::Ui(ui_event) => {
                if ui_event.event_type  == UiEventType::ClickStart{
                    if ui_event.target == self.play_button_entity.as_ref().unwrap().image_entity{

                    }
                    if ui_event.target == self.load_button_entity.as_ref().unwrap().image_entity{
                        return Trans::Switch(Box::new(LoadState::new()));
                    }
                    if ui_event.target == self.settings_button_entity.as_ref().unwrap().image_entity{
                        return Trans::Switch(Box::new(SettingsState::new()));
                    }
                    if ui_event.target == self.credits_button_entity.as_ref().unwrap().image_entity{
                        return Trans::Switch(Box::new(CreditsState::new()));
                    }
                }
            }
            _ => {}
        }

        Trans::None
    }


    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        //Delete the text entity
        data.world.delete_entity(self.text_entity.unwrap());
        data.world.delete_entity(self.play_button_entity.as_ref().unwrap().image_entity);
        data.world.delete_entity(self.play_button_entity.as_ref().unwrap().text_entity);
        data.world.delete_entity(self.load_button_entity.as_ref().unwrap().image_entity);
        data.world.delete_entity(self.load_button_entity.as_ref().unwrap().text_entity);
        data.world.delete_entity(self.credits_button_entity.as_ref().unwrap().image_entity);
        data.world.delete_entity(self.credits_button_entity.as_ref().unwrap().text_entity);
        data.world.delete_entity(self.settings_button_entity.as_ref().unwrap().image_entity);
        data.world.delete_entity(self.settings_button_entity.as_ref().unwrap().text_entity);

    }


}