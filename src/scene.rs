use super::renderables::SceneObject;
use super::rays::Ray;
use super::color::Color;

pub struct Scene {
    pub objects: Vec<Box<dyn SceneObject>>
}

impl Scene {
    pub fn new(objs: Vec<Box<dyn SceneObject>>) -> Scene {
        Scene {
            objects: objs
        }
    }

    pub fn find_nearest_obj(&self, ray: &Ray) -> Option<&Box<dyn SceneObject>> {
        let mut intersections: Vec<(&Box<dyn SceneObject>, f64)> = vec!();
        for obj in &self.objects {
            let intersect_times = obj.calc_intersect_time(ray);
            match intersect_times {
                None => continue,
                Some(times) => {
                    let mut lowest_time = -1.;
                    for time in times {
                        if time > lowest_time && time > 0. {
                            lowest_time = time;
                        }
                    }

                    intersections.insert(0, (obj, lowest_time));
                }
            }            
        }

        if intersections.len() == 0 {
            return None;
        }

        intersections.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        return Some(intersections[0].0);  // Return ref to scene obj closest to cam in view
    }

    pub fn calc_bg_color(img_height: u32, img_width: u32, pixel_y: u32) -> Color {
        return Color::new(
            (0. + 255. * (pixel_y as f64 / img_height as f64)) as u8,
            (104. + 151. * (pixel_y as f64 / img_height as f64)) as u8,
            (195. + 60. * (pixel_y as f64 / img_height as f64)) as u8
        );
    }
}