use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::Vec3;

/// Caja alineada a los ejes (AABB), definida por sus esquinas mínima y máxima.
pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, origin: &Vec3, direction: &Vec3) -> Option<Intersect> {
        let mut t_near = f32::NEG_INFINITY;
        let mut t_far = f32::INFINITY;
        let mut near_normal = Vec3::zeros();
        let mut far_normal = Vec3::zeros();

        for axis in 0..3 {
            if direction[axis].abs() < 1.0e-6 {
                if origin[axis] < self.min[axis] || origin[axis] > self.max[axis] {
                    return None;
                }
                continue;
            }

            let mut t1 = (self.min[axis] - origin[axis]) / direction[axis];
            let mut t2 = (self.max[axis] - origin[axis]) / direction[axis];
            let mut n1 = Vec3::zeros();
            let mut n2 = Vec3::zeros();
            n1[axis] = -1.0;
            n2[axis] = 1.0;

            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
                std::mem::swap(&mut n1, &mut n2);
            }
            if t1 > t_near {
                t_near = t1;
                near_normal = n1;
            }
            if t2 < t_far {
                t_far = t2;
                far_normal = n2;
            }
            if t_near > t_far {
                return None;
            }
        }

        let (distance, normal) = if t_near > 1.0e-4 {
            (t_near, near_normal)
        } else if t_far > 1.0e-4 {
            (t_far, far_normal)
        } else {
            return None;
        };

        Some(Intersect {
            point: origin + direction * distance,
            normal,
            distance,
            material: self.material,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    fn cube() -> Cube {
        Cube {
            min: Vec3::new(-1.0, -1.0, -1.0),
            max: Vec3::new(1.0, 1.0, 1.0),
            material: Material {
                diffuse: Color::new(220, 30, 30),
            },
        }
    }

    #[test]
    fn hits_front_face_with_correct_distance_and_normal() {
        let hit = cube()
            .ray_intersect(&Vec3::new(0.0, 0.0, 5.0), &Vec3::new(0.0, 0.0, -1.0))
            .unwrap();
        assert!((hit.distance - 4.0).abs() < 1.0e-5);
        assert_eq!(hit.normal, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn hits_each_face_with_outward_normal() {
        let tests = [
            (
                Vec3::new(3.0, 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            (
                Vec3::new(-3.0, 0.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
            ),
            (
                Vec3::new(0.0, 3.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            (
                Vec3::new(0.0, -3.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, -1.0, 0.0),
            ),
            (
                Vec3::new(0.0, 0.0, -3.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, -1.0),
            ),
        ];
        for (origin, direction, expected) in tests {
            assert_eq!(
                cube().ray_intersect(&origin, &direction).unwrap().normal,
                expected
            );
        }
    }

    #[test]
    fn misses_cube_and_handles_ray_from_inside() {
        assert!(cube()
            .ray_intersect(&Vec3::new(2.0, 2.0, 5.0), &Vec3::new(0.0, 0.0, -1.0))
            .is_none());
        let hit = cube()
            .ray_intersect(&Vec3::zeros(), &Vec3::new(1.0, 0.0, 0.0))
            .unwrap();
        assert_eq!(hit.normal, Vec3::new(1.0, 0.0, 0.0));
        assert!((hit.distance - 1.0).abs() < 1.0e-5);
    }
}
