use godot::{
    builtin::Vector2,
    classes::{Node2D, CharacterBody2D,Sprite2D, ICharacterBody2D, Input, ProjectSettings,AnimationPlayer},
    global::{godot_print, move_toward},
    obj::{Base, WithBaseField,Gd},
    prelude::{godot_api, GodotClass},
};

use crate::tile_map_rules;

#[derive(GodotClass)]
#[class(init,base=CharacterBody2D)]
struct PlayerAdvance{
    #[export]
    debug : bool,
    #[export]
    speed: f64,
    #[export]
    jump_velocity: f64,
    #[export]
    life : f64,
    base: Base<CharacterBody2D>,
    #[export]
    node_manager: Option<Gd<Node2D>>,
    status : PlayerState,
    animation : String,
    direction : f32,
    pub tile_collide : Vec<i32>,
}

#[derive(Clone, Copy,PartialEq)]
enum MovementDirection {
    Left,
    Neutral,
    Right,
}

impl Default for MovementDirection {
    fn default() -> Self { MovementDirection::Neutral }
}

#[derive(Clone, Copy,PartialEq)]
enum PlayerState {
    Idle,
    Walking,
    Jumping,
    RollingStart,
    Rolling,
    RollingEnd,
    Grabbing,
    Holding,
    Releasing,
}

impl Default for PlayerState {
    fn default() -> Self { PlayerState::Idle }
}

#[godot_api]
impl ICharacterBody2D for PlayerAdvance {
   /* fn init(base: Base<CharacterBody2D>) -> Self {
        godot_print!("Initialise player Rust class");
        Self {
            speed: 130.0,
            jump_velocity: -300.0,

            base,
        }
    }*/

    fn ready(&mut self) {

        self.status = PlayerState::Idle;

        godot_print!("Player ready");
    }

    //fn physics_process(&mut self, delta: f64) {
    fn process(&mut self, delta: f64) {
        let Vector2 {
            x: velocity_x,
            y: velocity_y,
        } = self.base().get_velocity();

        let input = Input::singleton();

        let mut animated_player =  self
            .base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer");
        
        let mut sprite = self
            .base()
            .get_node_as::<Sprite2D>("Sprite2D");

        let mut new_velocity_y = velocity_y;

        // player input ==> what he

        // handle jump and gravity if not grabbing
       // if self.status != PlayerState::Grabbing 
       // {
            new_velocity_y = if self.base().is_on_floor() {
                if input.is_action_pressed("jump".into()) {
                    #[allow(clippy::cast_possible_truncation)]
                    {
                        self.jump_velocity as f32
                    }
                } else {
                    velocity_y
                }
            } else if self.status == PlayerState::Grabbing &&  input.is_action_pressed("jump".into()) {
                    self.status == PlayerState::Releasing;
                    0.0
                }
            else {
                let gravity = ProjectSettings::singleton()
                    .get_setting("physics/2d/default_gravity".into())
                    .try_to::<f64>()
                    .expect("Should be able to represent default gravity as a 32-bit float");
                #[allow(clippy::cast_possible_truncation)]
                {
                    velocity_y + (gravity * delta) as f32
                }
            };
        //}

        // Get input direction
        let mut direction = input.get_axis("move_left".into(), "move_right".into());
        let mut movement_direction = match direction {
            val if val < -f32::EPSILON => MovementDirection::Left,
            val if (-f32::EPSILON..f32::EPSILON).contains(&val) => { 
                    // if rolling and No move continue move
                    if self.status == PlayerState::RollingStart || self.status == PlayerState::Rolling || self.status == PlayerState::RollingEnd {
                        direction = self.direction;
                        match self.direction {
                            1.0 =>  MovementDirection::Right,
                            _ =>  MovementDirection::Left
                        }                            
                    }
                    else {
                        MovementDirection::Neutral
                    }
            },
            val if val >= f32::EPSILON => MovementDirection::Right,
            _ => unreachable!(),
        };

        // Flip the sprite to match movement direction
        match movement_direction {
            MovementDirection::Right => sprite.set_flip_h(true),
            MovementDirection::Neutral => {}
            MovementDirection::Left => sprite.set_flip_h(false),
        }

        // TILE INTERACTION
        // Node manager 
        let nm = self.
            node_manager.
            as_ref().
            unwrap();
            
        let val  = nm.get_node_as::<tile_map_rules::NodeManager>("NodeManager").bind_mut().tile_collide.clone();
        let block = 3;

        if self.tile_collide != val {godot_print!("-------\n{} {} {}\n{} {} {}\n{} {} {}\n{} {} {}\n", val[3], val[7], val[11],val[2],val[6],val[10],val[1],val[5],val[9],val[0],val[4],val[8]);}

        self.tile_collide = val.clone();

        let mut grab = false;
        if velocity_y >= 0.0 {
            if (val[1] == 1 && val[0] == -33)|| (val[10]  == 1 && val[11] == -33) {
               grab =  false // uniactivate
            }
            else {
                grab =  false
            } 
        } else {
            grab =  false
        }

       // if self.debug {godot_print!("grab : {} {}",grab,velocity_y);}

        let block = if val[5] == block {
            //godot_print!("Block");
            true
        } else {
            false
        };

        // Play animation
        let status = if self.status == PlayerState::RollingStart && block {      // still on glissade
                if !animated_player.is_playing() {
                    PlayerState::Rolling
                }   
                else {
                    PlayerState::RollingStart
                }
            }
            else if self.status == PlayerState::Rolling && block { // end of glissade
                PlayerState::Rolling
            }
            else if self.status == PlayerState::Rolling && !block { // end of glissade
                PlayerState::RollingEnd
            }
            else if self.status == PlayerState::RollingEnd { // end of glissade
                if !animated_player.is_playing() { PlayerState::Idle }
                else { PlayerState::RollingEnd }
            }
            else if self.status== PlayerState::Jumping && block { // jumping and follung on swipe
                if sprite.is_flipped_h() { 
                    movement_direction=MovementDirection::Right;direction = 1.0; 
                 }
                else { 
                    movement_direction= MovementDirection::Left;direction = -1.0;
                }
                //godot_print!("Jumping and following on swipe");
                PlayerState::RollingStart
            }
            else if self.status == PlayerState::Jumping && grab { // jumping and not following on swipe
                PlayerState::Grabbing
            }
            else if self.status == PlayerState::Grabbing && !grab { // end of grabbing
                PlayerState::Jumping
            }
            else if self.status == PlayerState::Releasing { // end of releasing
                PlayerState::Releasing
            }
            /*else if self.status == PlayerState::Holding && !block { // end of holding
                PlayerState::Jumping
            }*/
            else if self.base().is_on_floor() {
                match movement_direction {
                    MovementDirection::Neutral => PlayerState::Idle,
                    MovementDirection::Left | MovementDirection::Right => {
                        match block {
                            true => PlayerState::RollingStart,
                            false => PlayerState::Walking,
                    }
                }
            }} else {
                PlayerState::Jumping
            };

        let animation = match status {
            PlayerState::Idle => "Default",
            PlayerState::Walking => "walk",
            PlayerState::Jumping => "jump",
            PlayerState::RollingStart => "rollupstart",
            PlayerState::Rolling => "rolling",
            PlayerState::RollingEnd => "rollupend",
            PlayerState::Grabbing => "jump",
            PlayerState::Holding => "jump",
            PlayerState::Releasing => "jump",
        };

        // backup status
        self.status = status;
        
      //  if !animated_player.is_playing() {
            //self.animated_sprite.play_ex().name(animation.into()).done();
            if animation != "" {
                animated_player.set_current_animation(animation.into());
                //animated_player.set_speed_scale(5.0);
                animated_player.play();
            }
            //if self.debug { godot_print!("Animation {} new {}", self.animation,animation); }
      //  }

        // ENd Tile Interaction

        // Apply movement
        #[allow(clippy::cast_possible_truncation)]

        let new_velocity_x = match movement_direction {
            MovementDirection::Neutral => {
                move_toward(f64::from(velocity_x), 0.0, self.speed) as f32
            }
            MovementDirection::Left | MovementDirection::Right => direction * (self.speed) as f32,
        };

        self.base_mut().set_velocity(Vector2 {
            x: new_velocity_x,
            y: if !grab {new_velocity_y} else {0.0},
        });

        self.base_mut().move_and_slide();

        self.animation = animation.to_string();
        self.direction = direction;

    }
}
