use math::*;

#[derive(Clone, Copy)]
struct Film {
    distance: f32,
    half_width: f32,
    half_height: f32,
    center: Vector3,
}

#[derive(Clone, Copy)]
pub struct Camera {
    position: Vector3,
    x_axis: Vector3,
    y_axis: Vector3,
    z_axis: Vector3,

    film: Film,
}

impl Camera {
    pub fn new(image_width: u32, image_height: u32, film_distance: f32) -> Camera {
        let fw: f32;
        let fh: f32;

        if image_width > image_height {
            fw = 1.0;
            fh = fw * (image_height as f32) / (image_width as f32);
        } else if image_width < image_height {
            fh = 1.0;
            fw = fh * (image_width as f32) / (image_height as f32);
        } else {
            fw = 1.0;
            fh = 1.0;
        }

        let film = Film {
            distance: film_distance,
            half_width: fw * 0.5,
            half_height: fh * 0.5,
            center: Vector3::new(0.0, 0.0, 0.0),
        };

        Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            x_axis: Vector3::new(0.0, 0.0, 0.0),
            y_axis: Vector3::new(0.0, 0.0, 0.0),
            z_axis: Vector3::new(0.0, 0.0, 0.0),
            film: film,
        }
    }

    pub fn look_at(&mut self, position: Vector3, target: Vector3) {
        self.position = position;
        self.z_axis = safe_normalize(self.position - target);
        self.x_axis = safe_normalize(cross(Vector3::unit_z(), self.z_axis));
        self.y_axis = safe_normalize(cross(self.x_axis, self.z_axis));

        self.film.center = self.position - self.film.distance * self.z_axis;
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let x_axis = u * self.film.half_width * self.x_axis;
        let y_axis = v * self.film.half_height * self.y_axis;
        let film_position = self.film.center + x_axis + y_axis;

        Ray::new(self.position, film_position - self.position)
    }
}