use bevy::prelude::*;
use sickle_ui::prelude::*;

const TOP_SLOT_LEVEL: u8 = 0;
const SELECTED_COLOR: Color = Color::linear_rgb(1.0, 1.0, 1.0);
const NORMAL_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.0);

#[derive(PartialEq, Debug)]
pub enum CurrentMenu {
    None,
    SideBars
}

#[derive(Resource, Debug)]
pub struct MenuRes {
    pub current_menu: CurrentMenu,
    pub pos: UiPos
}

#[derive(PartialEq, Debug)]
pub struct UiPos {
    x: u8,
    y: u8
}


pub fn setup_ui_system(
    mut commands: Commands
) {
    // Top bar.
    commands.ui_builder(UiRoot)
        .row(|row| {
            
            row.style()
                .width(Val::Percent(100.0))
                .height(Val::Percent(12.5))
                .justify_content(JustifyContent::FlexStart)
                .background_color(Color::linear_rgba(1.0, 1.0, 1.0, 0.25));
            
            
            for n in 0..10 {
                add_slot(row, n);
            }
    });
}

fn add_slot(mut row: &mut UiBuilder<Entity>, id: u8) {
    row.container((NodeBundle::default(), ObjectSlot::new(id), Name::new(format!("Object slot {id}"))), |slot|{
        slot.style()
            .height(Val::Percent(75.0))
            .aspect_ratio(Some(1.0))
            .margin(UiRect::left(Val::Vh(1.0)))
            .background_color(Color::linear_rgb(0.5, 0.5, 0.5))
            .border(UiRect::all(Val::Vh(0.5)))
            .border_color(Color::linear_rgba(0.0, 0.0, 0.0, 1.0));
    });
}

pub fn ui_selection_system(
    mut query: Query<(&mut UiSlot, &mut BorderColor)>,
    menu_res: Res<MenuRes>
) {
    // Figure out which one is selected.
    match menu_res.current_menu {
        CurrentMenu::None => {
            for (mut ui_slot, mut border_color) in query.iter_mut() {
                ui_slot.selected = false;
                border_color.0 = NORMAL_COLOR;
            }
        }
        CurrentMenu::SideBars => {
            for (mut ui_slot, mut border_color) in query.iter_mut() {
                ui_slot.selected = ui_slot.pos == menu_res.pos;
                border_color.0 = if ui_slot.selected { SELECTED_COLOR } else { NORMAL_COLOR };
            }
        }
    }
}


#[derive(Component)]
struct ObjectSlot {
    object_id: Option<usize>
}

#[derive(Component)]
pub struct UiSlot {
    pos: UiPos,
    selected: bool
}

impl ObjectSlot {
    fn empty() -> ObjectSlot {
        ObjectSlot { object_id: None }
    }
    fn new(id: u8) -> (ObjectSlot, UiSlot) {
        (ObjectSlot::empty(), UiSlot::new(id, TOP_SLOT_LEVEL))
    }
}

impl MenuRes {
    pub fn none() -> MenuRes {
        MenuRes {
            current_menu: CurrentMenu::None,
            pos: UiPos::zero()
        }
    }

    pub fn set_to_sidebars(&mut self) {
        self.current_menu = CurrentMenu::SideBars;
    }
}

impl UiSlot {
    fn new(x: u8, y: u8) -> UiSlot { UiSlot{ pos: UiPos::new(x, y), selected: false } }
}

impl UiPos {
    fn zero() -> UiPos { UiPos { x: 0, y: 0 } }
    fn new (x: u8, y: u8) -> UiPos { UiPos { x, y } }
}
