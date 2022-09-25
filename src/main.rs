// use std::io;
use std::env;
mod port;
use port::console::*;

const MOVE_ANGLE:f32 = 0.05;
const MOVE_POSITION:f32 = 0.1;

const RAYSTEP:f32 = 0.02;
const RAYSTEPS:i32 = 5000;

// Direction ===============================================
#[derive(Clone)]
struct Vect
{
    x:f32,
    y:f32,
    z:f32
}

impl Vect
{
	pub fn new(x:f32, y:f32, z:f32) -> Vect
	{
		Vect
		{
			x,
			y,
			z
		}
	}

    fn normalize(&mut self) 
    {
        let len:f32 = self.length();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    fn length(&self) -> f32 
    {
        return ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();
    }

    fn add(&mut self, v:&Vect) 
    {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }

    fn scale(&mut self, s:f32) 
    {
        self.x *= s;
        self.y *= s;
        self.z *= s;
    }

    fn scaled(&self, s:f32) -> Vect 
    {
        return Vect {x:self.x*s, y:self.y*s, z:self.z*s};
    }

    fn dist(&self, other:&Vect) -> f32 
    {
        return ((self.x-other.x)*(self.x-other.x) + (self.y-other.y)*(self.y-other.y) + (self.z-other.z)*(self.z-other.z)).sqrt();
    }

    fn dot(&self, other:&Vect) -> f32 
    {
        return self.x*other.x + self.y*other.y + self.z*other.z;
    }

}


// Direction ===============================================
#[derive(Clone)]
struct Direction 
{
    ang_v:f32,
    ang_h:f32
}

impl Direction
{
    fn to_unit(&self) -> Vect
    {
        return Vect {x:self.ang_v.cos() * self.ang_h.cos(), y:self.ang_v.cos() * self.ang_h.sin(), z:self.ang_v.sin()};
    }
}

// Ball ===============================================
struct Ball 
{
    center:Vect,
    radius:f32
}

impl Ball
{
	pub fn new(x:f32, y:f32, z:f32, r:f32) -> Ball
    {
        Ball {
			center : Vect::new(x, y, z),
			radius : r
        }
    }

    fn reflect(&mut self, mut incoming:Vect, mov:&Vect) -> Vect
    {
        self.center.scale(-1.0);
        incoming.add(&self.center);
        self.center.scale(-1.0);
        incoming.normalize();

        incoming.scale(-2.0 * incoming.dot(&mov));
        let mut new_move:Vect = mov.clone();
        new_move.add(&incoming);

        return new_move;
    }
}

struct Game 
{
    balls:Vec<Ball>,
    pos:Vect,
    dir:Direction,
    width:f32,
    height:f32,
    xres:i32,
    yres:i32,
    display_str:String
}    

impl Game
{
    pub fn new(start_pos:Vect, start_dir:Direction, width:f32, height:f32, xres:i32, yres:i32) -> Self
    {
        Game {
            width : width,
            height : height,
            pos : start_pos,
            dir : start_dir,
            xres : xres,
            yres : yres,
            balls : Vec::new(),
            display_str : String::new()
        }
    }

	fn add_ball(&mut self, b:Ball) 
    {
		self.balls.push(b);
	}

	fn make_pic(&mut self) 
    {
		// rays through equidistant points on width*height rectangle with distance 1 from viewer
		
		let v1:Vect = self.dir.to_unit();
		// v2 points from middle of the rectangle to upper edge
		let mut v2:Vect = Vect {
            x: -(self.dir.ang_v).tan() * v1.x, 
            y: -(self.dir.ang_v).tan() * v1.y, 
            z: self.dir.ang_v.cos() 
        };
	  	v2.scale(self.height / 2.0);
		// v3 points from middle of rectangle to left edge
 		let mut v3:Vect = Vect {x:-v1.y, y:v1.x, z:0.0};
 		v3.normalize();
 		v3.scale(self.width / 2.0);
		self.display_str.clear();

		for row in 0..self.yres 
        {
			for col in 0..self.xres
            {
				let up_offset:f32 = -(row as f32 / (self.yres as f32 - 1.0) - 0.5f32);
				let left_offset:f32 = col as f32 / (self.xres as f32 - 1.0) - 0.5f32;
				let mut mov:Vect = v1.clone();
				mov.add(&v2.scaled(up_offset));
				mov.add(&v3.scaled(left_offset));
				mov.normalize();
				mov.scale(RAYSTEP);

				let mut ray:Vect = self.pos.clone();
				// trace ray
				let mut dists_to_balls: Vec<f32> = Vec::new();
				for _i in 0..self.balls.len()
                {
					dists_to_balls.push(0.0);
				}
				
                let mut times_reflected = 0;
				let mut index:i32 = 0;
				while index < RAYSTEPS
                {
					if ray_done(&mut ray) 
                    {
						break;
					}
					
					let mut ball_index:usize = 0;
					for b in self.balls.iter_mut()
                    {
						let d:f32 = ray.dist(&b.center) - b.radius;
						dists_to_balls[ball_index] = d;
						if d < 0.0 
                        {
							mov = b.reflect(ray.clone(), &mov);
							times_reflected += 1;
						}
						ball_index += 1;
					}

					// optimization: test if all distances are large enough to make
					// multiple steps at once
					let mut min_dist:f32 = ray.z;
					for f in dists_to_balls.iter() 
                    {
						if *f < min_dist
                        {
							min_dist = *f;
						}
					}

					if min_dist > RAYSTEP
					{
						let possible_steps:i32 = (min_dist / RAYSTEP) as i32;
						index += possible_steps - 1;	// -1 because of default increment
						ray.add(&mov.scaled(possible_steps as f32));
					}

					else {
						ray.add(&mov);
					}
					index += 1;
				}
				self.display_str += &ray_char(&mut ray, times_reflected);
			}
			self.display_str += &setc(row, 0);
		}
		print!("{}", self.display_str);
	}

	fn start(&mut self) {
		let keys:[KeyCode;4] = [KeyCode::KEY_UP, KeyCode::KEY_DOWN, KeyCode::KEY_LEFT, KeyCode::KEY_RIGHT];
		loop 
        {
			self.make_pic();
			for key in &keys
            {
				if key_is_pressed(key)
                {
					if key_is_pressed(&KeyCode::KEY_SHIFT_L)
                    {
						self.move_view(key);
					}
					else 
                    {
						self.move_position(key);
					}
				}
			}
		}
	}

	fn move_view(&mut self, key:&KeyCode) 
	{
		if *key == KeyCode::KEY_UP 
		{
			self.dir.ang_v += MOVE_ANGLE;
		}
		else if *key == KeyCode::KEY_DOWN
		{
			self.dir.ang_v -= MOVE_ANGLE;
		}
		if *key == KeyCode::KEY_LEFT 
		{
			self.dir.ang_h -= MOVE_ANGLE;
		}
		if *key == KeyCode::KEY_RIGHT 
		{
			self.dir.ang_h += MOVE_ANGLE;
		}
	}

	fn move_position(&mut self, key:&KeyCode) 
	{
		let dir_vect:Vect = self.dir.to_unit().clone();
		let mut xmov:f32 = dir_vect.x;
		let mut ymov:f32 = dir_vect.y;
		let scale:f32 = 1.0 / ((xmov*xmov + ymov*ymov).sqrt());
		xmov *= scale;
		ymov *= scale;
		xmov *= MOVE_POSITION;
		ymov *= MOVE_POSITION;
		if *key == KeyCode::KEY_UP 
		{
			// move forward
			self.pos.x += xmov;
			self.pos.y += ymov;
		}
		else if *key == KeyCode::KEY_DOWN
		{
			// move back
			self.pos.x -= xmov;
			self.pos.y -= ymov;
		}
		if *key == KeyCode::KEY_LEFT
		{
			// move left
			self.pos.x += ymov;
			self.pos.y -= xmov;
		}
		if *key == KeyCode::KEY_RIGHT
		{
			// move right
			self.pos.x -= ymov;
			self.pos.y += xmov;
		}
	}

}

// for setting position of cursor in terminal window
fn setc(row:i32, col:i32) -> String
{
    return  "\x1b[".to_string() + &row.to_string() + ";" + &col.to_string() + "H";
}

// ray ends when it hits the floor at z = 0
fn ray_done(ray:&mut Vect) -> bool
{
	return ray.z <= 0.0;
}

// determines character to be printed for finished ray
fn ray_char(ray:&mut Vect, refl:i32) -> String
{
	// let chars = ['.', '-', ','];

	if ray.z <= 0.0 && ( ( (ray.x.floor() as i32 - ray.y.floor() as i32).abs() % 2) == 0)
	{
		return '#'.to_string();
	}
	else if refl > 0 
	{
		match refl-1 {
			0 => return '.'.to_string(),	
			1 => return '-'.to_string(),
			2 => return ','.to_string(),
			_ => return '+'.to_string()
		};
	}
	else 
	{
		return ' '.to_string();
	}
}

fn main() 
{
	let args: Vec<String> = env::args().collect();
	let start_pos:Vect = Vect {x:0.0, y:0.0, z:1.0};
	let start_dir:Direction = Direction {ang_v:-0.2, ang_h:0.0};

	// no window sizes given, defaults to 200x100
	let mut width:i32 = 100;
	let mut height:i32 = 50;
	
	if args.len() > 1
	{
		if let Ok(n) = args[1].trim().parse()
		{
			height = n;
		}
		if let Ok(n) = args[2].trim().parse()
		{
			width = n;
		}
	}
	let mut game:Game = Game::new(start_pos, start_dir, 2.0, 2.0, width, height);
	// add some balls and start the "game"
	let b:Ball = Ball::new(10.0, 0.0, 2.0, 2.0);
	game.add_ball(b);
	let c:Ball = Ball::new(20.0, 10.0, 2.0, 2.0);
	game.add_ball(c);
	let d:Ball = Ball::new(7.5, 0.0, 8.0, 4.0);
	game.add_ball(d);
	game.start();
}

