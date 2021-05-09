use bevy::prelude::*;
mod resources;
use resources::game::Game;


fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "不积跬步无以至千里".to_string(),
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(Game::new())
        .add_startup_system(init_ui.system())
        .add_system(button_system.system())
        .add_system(update_ui.system())
        .run();
}

fn update_ui(
    mut game:ResMut<Game>,
    mut text_query: Query<&mut Text>,
) {
    let is_need_update = game.is_need_update();
    if !is_need_update {
        return;
    }
    let mut i = 0;
    for mut text in text_query.iter_mut() {

        match i {
            0 => {//level
                let mut section = &mut text.sections[1];
                section.value = game.current_level().to_string();
            },
            1 => {//question
                let mut section = &mut text.sections[1];
                section.value = game.question().to_string();
            },
            2 => {//time
                let mut section = &mut text.sections[1];
                section.value = "30".to_string();
            },
            3 => {//scores
                let mut section = &mut text.sections[1];
                section.value = game.scores().to_string();
            },
            _ => {//options
                let mut section = &mut text.sections[0];
                section.value = game.options()[i - 4].to_string();
            }
        }
        i+=1;
    }

    game.update_complete();

}
fn button_system(
    mut game:ResMut<Game>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input:Res<Input<MouseButton>>,
    mut interaction_query: Query<(
        &Button,
        &Interaction,
        &mut Handle<ColorMaterial>,
        &Children,
    )>,
    text_query: Query<&Text>,
) {

    for (_button, interaction, mut material, children) in interaction_query.iter_mut() {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked if keyboard_input.just_pressed(MouseButton::Left) => {
                *material = materials.add(Color::BLUE.into());
                //button_press(&mut calc,text.value.clone());
                let option = text.sections[0].value.clone();
                println!("click:{:?}", option);
                game.answer(option);
            }
            Interaction::Hovered => {
                *material = materials.add(Color::BLACK.into());
            }
            Interaction::None => {
                *material = materials.add(Color::GREEN.into());
            }
            _ => {

            }
        }

    }
}
pub fn init_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game: Res<Game>

) {
    println!("level:{}", game.current_level());
    let level = game.current_level();
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn()
        .insert_bundle(NodeBundle {//头部关卡　问题　得分　显示区域
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(70.0)),
                position_type: PositionType::Absolute,
                position: Rect{left:Val::Px(0.0),top:Val::Px(0.0),..Default::default()},
                border: Rect::all(Val::Px(2.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::BLACK.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn().insert_bundle(TextBundle {
                style: Style {
                    ..Default::default()
                },
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Level:".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 60.0,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        },
                        TextSection {
                            value: level.to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 60.0,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            });

            parent
                .spawn()
                .insert_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text:Text {
                        sections: vec![
                            TextSection {
                                value: "Qustion:".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 60.0,
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                            },
                            TextSection {
                                value: game.question().to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 60.0,
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                });
           parent
                .spawn()
                .insert_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text:Text {
                        sections: vec![
                            TextSection {
                                value: "Time:".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 60.0,
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                            },
                            TextSection {
                                value: "30".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 60.0,
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                });
            parent
                .spawn()
                .insert_bundle(TextBundle {
                    style: Style {
                        ..Default::default()
                    },
                    text:Text {
                        sections: vec![
                            TextSection {
                                value: "Scores:".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 60.0,
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                            },
                            TextSection {
                                value: game.scores().to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 60.0,
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                            },
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                });
        });

    commands.spawn().insert_bundle(NodeBundle {//answer option 答案选项区域
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            flex_direction:FlexDirection::Row,
            flex_wrap:FlexWrap::Wrap,
            align_self:AlignSelf::Center,
            justify_content:JustifyContent::SpaceAround,
            align_content:AlignContent::SpaceAround,
            ..Default::default()
        },
        material: materials.add(Color::GREEN.into()),
        ..Default::default()
    }).with_children(|parent| {
        let btn_symbols = game.options();
        for i in &btn_symbols{
            parent
                .spawn().insert_bundle(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Percent(33.3), Val::Px(25.0)),
                        // center button
                        padding:Rect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: materials.add(Color::GREEN.into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn().insert_bundle(TextBundle {
                        text:Text {
                            sections: vec![
                                TextSection {
                                    value: i.to_string(),
                                    style: TextStyle {
                                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                        font_size: 60.0,
                                        color: Color::WHITE,
                                        ..Default::default()
                                    },
                                }
                            ],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        }
    });

}