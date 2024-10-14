use godot::{
    builtin::Vector2,builtin::Vector2i,
    classes::{AnimatedSprite2D, Node2D, CharacterBody2D,Sprite2D, ICharacterBody2D, Input, ProjectSettings,AnimationPlayer},
    global::{godot_print, move_toward},
    obj::{Base, WithBaseField,Gd},
    prelude::{godot_api, GodotClass},
};

use crate::tileMapRules;

#[derive(GodotClass)]
#[class(init,base=CharacterBody2D)]
struct Player{
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
}

enum MovementDirection {
    Left,
    Neutral,
    Right,
}

enum PlayerState {
    Idle,
    Walking,
    Jumping,
    RollingStart,
    Rolling,
    RollingEnd,
}

impl Default for PlayerState {
    fn default() -> Self { PlayerState::Idle }
}

#[godot_api]
impl ICharacterBody2D for Player {
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


        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let mut animated_player =  self
            .base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer");
        
        let mut sprite = self
            .base()
            .get_node_as::<Sprite2D>("Sprite2D");

        // handle jump and gravity
        let new_velocity_y = if self.base().is_on_floor() {
            if input.is_action_pressed("jump".into()) {
                #[allow(clippy::cast_possible_truncation)]
                {
                    self.jump_velocity as f32
                }
            } else {
                velocity_y
            }
        } else {
            let gravity = ProjectSettings::singleton()
                .get_setting("physics/2d/default_gravity".into())
                .try_to::<f64>()
                .expect("Should be able to represent default gravity as a 32-bit float");
            #[allow(clippy::cast_possible_truncation)]
            {
                velocity_y + (gravity * delta) as f32
            }
        };

        // Get input direction
        let direction = input.get_axis("move_left".into(), "move_right".into());
        let movement_direction = match direction {
            val if val < -f32::EPSILON => MovementDirection::Left,
            val if (-f32::EPSILON..f32::EPSILON).contains(&val) => MovementDirection::Neutral,
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
            
        let val  = nm.get_node_as::<tileMapRules::NodeManager>("NodeManager").bind_mut().tile_collide.clone();
    
        let block = 3;

        if self.debug {godot_print!("{} {}   {}    {} {}", val[0], val[1], val[2],val[3],val[4]);}
    
        let mut block = if (val[1] == block) || (val[3] == block) {
            godot_print!("Block");
            true
        } else {
            false
        };

        // Play animation
        let animation = if self.base().is_on_floor() {
            match movement_direction {
                MovementDirection::Neutral => "Default",
                MovementDirection::Left | MovementDirection::Right => {
                    match block {
                        true => "rollupstart",
                        false => "walk",
                    }
                },
            }
        } else {
            "jump"
        };
        
        //self.animated_sprite.play_ex().name(animation.into()).done();

        animated_player.set_current_animation(animation.into());
        animated_player.set_speed_scale(3.0);
        animated_player.play();
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
            y: new_velocity_y,
        });

        self.base_mut().move_and_slide();
    }
}
