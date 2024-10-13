use godot::{
    builtin::Vector2,builtin::Vector2i,
    classes::{ ProjectSettings,INode2D,Node2D,CharacterBody2D,TileMap},
    global::{godot_print, move_toward},
    obj::{Base, WithBaseField,Gd},
    prelude::{godot_api, GodotClass},
};

#[derive(GodotClass)]
#[class(init,base=Node2D)]
pub struct NodeManager{
    #[export]
    player: Option<Gd<CharacterBody2D>>,
    base: Base<Node2D>,
    tile_down : Vector2i,
    tile_left : Vector2i,
    tile_right : Vector2i,
}

#[godot_api]
impl INode2D for NodeManager {
   /* fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Initialise player Rust class");
        Self {
            speed: 130.0,
            jump_velocity: -300.0,

            base,
        }
    }*/

    fn physics_process(&mut self, delta: f64) {

        // Collision detetction
        let position = self.player.as_ref().unwrap().get_global_position();
		//let tile = self.base().get_tree().unwrap().get_current_scene().unwrap().get_node_as("TileMapLayer").get_cellv(self.base().get_tree().unwrap().get_current_scene().unwrap().get_node("TileMapLayer").world_to_map(position));
        
        /*
        let tile_map_layer = self
            .base()
            .get_tree()
            .unwrap()
            .get_current_scene()
            .unwrap()
            .get_node_as::<TileMap>("TileMap");*/
        let tile_map_layer = self
            .base()
            .get_node_as::<TileMap>("TileMap");

       // let p : Vector2i = Vector2i::new(position.x as i32, position.y as i32);
        
        let mut p1 = tile_map_layer.local_to_map(position);
        
        if let tile = tile_map_layer.get_cell_atlas_coords(0,p1) {
            godot_print!("Tile: {:?} at x: {}, y:{}" ,tile, p1.x, p1.y);
            self.tile_down= tile;
        }
        p1.x += 1;
        if let tile = tile_map_layer.get_cell_atlas_coords(0,p1) {
            godot_print!("TileR: {:?} at x: {}, y:{}" ,tile, p1.x, p1.y);
            self.tile_right= tile;
        }
        p1.x -= 2;
        if let tile = tile_map_layer.get_cell_atlas_coords(0,p1) {
            godot_print!("TileL: {:?} at x: {}, y:{}" ,tile, p1.x, p1.y);
            self.tile_left= tile;
        }
        
    }
}