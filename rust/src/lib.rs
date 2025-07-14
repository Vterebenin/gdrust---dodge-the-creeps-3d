use std::f64::consts::PI;

use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;
use godot::classes::Input;
use godot::classes::PathFollow3D;
use godot::global::randf;
use godot::global::randf_range;
use godot::global::randi_range;
use godot::obj::WithBaseField;
use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    pub speed: f32,
    pub fall_acceleration: f64,
    pub target_velocity: Vector3,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            speed: 14.,
            fall_acceleration: 75.,
            target_velocity: Vector3::ZERO,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut direction = Vector3::ZERO;

        let input = Input::singleton();
        if input.is_action_pressed("move_right") {
            direction.x += 1.;
        }
        if input.is_action_pressed("move_left") {
            direction.x -= 1.;
        }
        if input.is_action_pressed("move_back") {
            direction.z += 1.;
        }
        if input.is_action_pressed("move_forward") {
            direction.z -= 1.;
        }

        if direction != Vector3::ZERO {
            direction = direction.normalized();
            let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
            pivot.set_basis(Basis::looking_at(direction, Vector3::UP, false));
        }

        self.target_velocity.x = direction.x * self.speed;
        self.target_velocity.z = direction.z * self.speed;
        if !self.base().is_on_floor() {
            self.target_velocity.y -= (self.fall_acceleration * delta) as f32;
        }
        let target_vel = self.target_velocity;
        self.base_mut().set_velocity(target_vel);
        self.base_mut().move_and_slide();
    }
}

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Mob {
    pub min_speed: i8,
    pub max_speed: i8,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Mob {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            base,
            min_speed: 10,
            max_speed: 14,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Mob {
    #[func]
    pub fn initialize(&mut self, start_position: Vector3, player_position: Vector3) {
        self.base_mut()
            .look_at_from_position(start_position, player_position);

        let divider = 4.;
        let angle = PI / divider;
        let rand_item = randf_range(-angle, angle) as f32;
        self.base_mut().rotate_y(rand_item);

        let random_speed = randi_range(self.min_speed.into(), self.max_speed.into());

        self.base_mut()
            .set_velocity(Vector3::FORWARD * random_speed as f32);

        let velocity = self
            .base()
            .get_velocity()
            .rotated(Vector3::UP, self.base().get_rotation().y);

        self.base_mut().set_velocity(velocity);
    }

    #[func]
    pub fn _on_visible_on_screen_notifier_3d_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
struct Main {
    base: Base<Node>,
    #[export]
    mob_scene: Option<Gd<PackedScene>>,
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            mob_scene: None,
        }
    }
}

#[godot_api]
impl Main {
    #[func]
    pub fn _on_mob_timer_timeout(&mut self) {
        let mob_scene = self.mob_scene.as_ref().unwrap();
        let mut mob_instance = mob_scene.instantiate_as::<Mob>();
        
        {
            let mut mob = mob_instance.bind_mut();
            let mut mob_spawn_location = self
                .base()
                .get_node_as::<PathFollow3D>("SpawnPath/SpawnLocation");
            mob_spawn_location.set_progress_ratio(randf() as f32);

            let mut player = self
                .base()
                .get_node_as::<Player>("Player");
            let player_position = player.get_position();
            mob.initialize(mob_spawn_location.get_position(), player_position);
        }
        
        self.base_mut().add_child(&mob_instance);
    }
}

