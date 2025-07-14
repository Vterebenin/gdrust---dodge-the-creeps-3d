use godot::classes::CharacterBody3D;
use godot::classes::ICharacterBody3D;
use godot::classes::Input;
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

    base: Base<CharacterBody3D>
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

