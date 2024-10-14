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
    debug : bool,
    #[export]
    player: Option<Gd<CharacterBody2D>>,
    base: Base<Node2D>,
    pub tile_collide : Vec<i32>,
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
        
        self.tile_collide = Vec::new();

        let mut p1 = tile_map_layer.local_to_map(position);

        /*
        detection following

        *  *
        *  *
         * 

        counter-clockwise

        */

       let sizegrid = 32; 

       let mod_vector : Vec<Vector2i> = vec![
        Vector2i::new(-1,-1),
        Vector2i::new(-1,0),
        Vector2i::new(0,1),
        Vector2i::new(1,0),
        Vector2i::new(1,-1),
       ];

       mod_vector.iter().for_each(|v| {
        let target = p1+ *v;
           let mut tile = tile_map_layer.get_cell_atlas_coords(0,target); // derefn v before adding
           if self.debug {godot_print!("Tile: {:?} at x: {}, y:{}" ,tile, target.x, target.y);}
           self.tile_collide.push(tile.x + tile.y*sizegrid);
       });

        /*
        p1.x -= 1;
        p1.y -= 1;
        let mut tile = tile_map_layer.get_cell_atlas_coords(0,p1);
         if self.debug {godot_print!("Tile: {:?} at x: {}, y:{}" ,tile, p1.x, p1.y);}
           self.tile_collide.push(tile.x + tile.y*sizegrid);
        
        p1.y += 1;
        let mut tile = tile_map_layer.get_cell_atlas_coords(0,p1);
         if self.debug {godot_print!("Tile: {:?} at x: {}, y:{}" ,tile, p1.x, p1.y);}
         self.tile_collide.push(tile.x + tile.y*sizegrid);
        
        p1.y +=1;
        p1.x += 1;
        tile = tile_map_layer.get_cell_atlas_coords(0,p1);
        if self.debug {   godot_print!("TileR: {:?} at x: {}, y:{}" ,tile, p1.x, p1.y);}
        self.tile_collide.push(tile.x + tile.y*sizegrid);

        p1.x += 1;
        p1.y -= 1 ;
        tile = tile_map_layer.get_cell_atlas_coords(0,p1);
        if self.debug {   godot_print!("TileL: {:?} at x: {}, y:{}" ,tile, p1.x, p1.y);}
        self.tile_collide.push(tile.x + tile.y*sizegrid);        
        
        p1.y -= 1 ;
        tile = tile_map_layer.get_cell_atlas_coords(0,p1);
        if self.debug {   godot_print!("TileL: {:?} at x: {}, y:{}" ,tile, p1.x, p1.y);}
        self.tile_collide.push(tile.x + tile.y*sizegrid);
        */
    }
}