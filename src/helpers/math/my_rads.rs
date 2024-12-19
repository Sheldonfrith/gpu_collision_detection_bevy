use std::f32::consts::PI;

use bevy::math::{Quat, Vec2, Vec3};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RotationDirection {
    Clockwise,
    Counterclockwise,
}
#[derive(Debug, Clone, Copy)]

pub struct MyRadsRange {
    start: MyRads,
    end: MyRads,
    direction: RotationDirection,
}
impl MyRadsRange {
    pub fn new(start: MyRads, end: MyRads, direction: RotationDirection) -> MyRadsRange {
        MyRadsRange {
            start: start,
            end: end,
            direction: direction,
        }
    }
    pub fn from_center_point_and_max_deviation(
        center: MyRads,
        max_deviation_rads: f32,
    ) -> MyRadsRange {
        MyRadsRange {
            start: MyRads::new(center.rads() - max_deviation_rads),
            end: MyRads::new(center.rads() + max_deviation_rads),
            direction: RotationDirection::Counterclockwise,
        }
    }
    pub fn is_in_range(&self, rads: MyRads) -> bool {
        let rads = rads.rads();
        // println!("is in range");
        // println!("rads: {}", rads);
        // println!("range: {:?}", self);

        if self.direction == RotationDirection::Counterclockwise {
            if rads >= self.start.rads() && rads <= self.end.rads() {
                return true;
            }
        } else {
            if rads <= self.start.rads() || rads >= self.end.rads() {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MyRads {
    rads: f32,
}
impl MyRads {
    pub fn rads(&self) -> f32 {
        self.rads
    }
    pub fn to_quat(&self) -> Quat {
        Quat::from_rotation_z(self.rads)
    }
    pub fn to_vec2(&self) -> Vec2 {
        Vec2::new(-self.rads.sin(), self.rads.cos())
    }
    pub fn from_unit_vec2(vec: Vec2) -> MyRads {
        // let angle = vec.y.atan2(vec.x);
        let angle = -vec.x.atan2(vec.y);
        if angle == -PI {
            return MyRads::new(PI);
        }
        return MyRads::new(angle);
    }
    pub fn add(&mut self, rads: f32) {
        self.rads = MyRads::limit_rads(self.rads + rads);
    }
    pub fn slerp(&self, target: MyRads, t: f32) -> MyRads {
        let mut angle = target.rads() - self.rads();
        if angle.abs() > PI {
            if angle > 0.0 {
                angle -= 2.0 * PI;
            } else {
                angle += 2.0 * PI;
            }
        }
        return MyRads::new(self.rads() + angle * t);
    }
    pub fn get_closest_angle_within_range(&self, range: MyRadsRange) -> MyRads {
        if range.is_in_range(*self) {
            // println!("in range already : {}, {:?}", self.rads(), range);
            return *self;
        }
        let dist_start = self.get_closest_distance_abs_radians(range.start);
        let dist_end = self.get_closest_distance_abs_radians(range.end);
        // println!("self rads: {}", self.rads());
        // println!("dist_start: {}", dist_start);
        // println!("dist_end: {}", dist_end);
        if dist_start < dist_end {
            return range.start;
        } else {
            return range.end;
        }
    }

    pub fn rotate(&mut self, rads: f32, direction: RotationDirection) {
        match direction {
            RotationDirection::Counterclockwise => self.add(rads),
            RotationDirection::Clockwise => self.add(-rads),
        }
    }

    pub fn get_perpendiculars(&self) -> (MyRads, MyRads) {
        let perpendicular1 = MyRads::new(self.rads() + PI / 2.0);
        let perpendicular2 = MyRads::new(self.rads() - PI / 2.0);
        return (perpendicular1, perpendicular2);
    }

    pub fn get_most_direct_rotation_direction(&self, target: MyRads) -> RotationDirection {
        let diff = target.rads() - self.rads();
        // Determine the shortest direction
        if diff.abs() <= PI {
            if diff >= 0.0 {
                RotationDirection::Counterclockwise
            } else {
                RotationDirection::Clockwise
            }
        } else {
            if diff > 0.0 {
                RotationDirection::Clockwise
            } else {
                RotationDirection::Counterclockwise
            }
        }
    }
    pub fn are_equivalent(&self, target: MyRads) -> bool {
        if self.rads() == target.rads()
            || self.rads() == PI && target.rads() == -PI
            || self.rads() == -PI && target.rads() == PI
            || self.rads() == 0.0 && target.rads() == -0.0
            || self.rads() == -0.0 && target.rads() == 0.0
        {
            return true;
        }
        self.rads() == target.rads()
    }
    pub fn get_closest_distance_abs_radians(&self, target: MyRads) -> f32 {
        if self.are_equivalent(target) {
            return 0.0;
        }
        let direction = self.get_most_direct_rotation_direction(target);
        // println!("direction in get closest dist: {:?}", direction);
        let source = self.rads();
        let target = target.rads();
        // source  = -1 / 1.1;
        // target  = 3/4;
        // result should be 1/4 + 1 - 1/1.1
        // which would be (pi - target) - (pi - source)
        match direction {
            RotationDirection::Counterclockwise => {
                if target > source {
                    (source - target).abs()
                } else {
                    ((-PI - target) - (PI - source)).abs()
                }
            }
            RotationDirection::Clockwise => {
                if target < source {
                    (source - target).abs()
                } else {
                    ((PI - target) - (-PI - source)).abs()
                }
            }
        }
    }
    pub fn opposite(&mut self) -> Self {
        self.add(PI);
        return *self;
    }
    pub fn from_between_points(from: Vec2, to: Vec2) -> MyRads {
        let vec_to_player = (to - from).normalize();
        let quat = Quat::from_rotation_arc(Vec3::Y, vec_to_player.extend(0.));
        return MyRads::from_quat(quat);
    }
    pub fn new(rads: f32) -> MyRads {
        return MyRads {
            rads: MyRads::limit_rads(rads),
        };
    }
    fn limit_rads(rads: f32) -> f32 {
        // important thing is that the absolute value can never exceed PI
        // println!("rads: {}", rads);
        let rads = rads % (2.0 * PI);
        // println!("rads: {}", rads);

        let adjusted_rads = if rads > PI {
            rads - 2.0 * PI
        } else if rads < -PI {
            rads + 2.0 * PI
        } else {
            rads
        };
        return adjusted_rads;
    }

    pub fn from_quat(q: Quat) -> MyRads {
        //* Testing has determined if the x and y axes of quat.to_axis_angle are negative or positive has zero effect in 2d */
        //* however the z axis being negative reverses the rotation direction in 2d */
        //* additionally, the 0 rad position is directly up, along the y axis */
        //* and positive RAD + positive Z axis = counterclockwise turn, and vice versa */
        let (axis, angle) = q.to_axis_angle();
        let angle_negative = angle < 0.0;
        let z_negative = axis.z < 0.0;
        let counterclockwise = angle_negative == z_negative;
        // println!("angle: {}", angle);
        // println!("angle_negative: {}", angle_negative);
        // println!("z_negative: {}", z_negative);
        // println!("counterclockwise: {}", counterclockwise);
        if counterclockwise {
            MyRads::new(angle.abs())
        } else {
            MyRads::new(-angle.abs())
        }
    }
}

//  implement tests for MyRads
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_new() {
        assert!(0.0001 > (MyRads::new(PI).rads() - PI).abs(),);
        assert!(0.0001 > (MyRads::new(-PI).rads() - -PI).abs());
        assert!(0.0001 > (MyRads::new(PI * 2.0).rads() - 0.0).abs());
        assert!(0.0001 > (MyRads::new(-PI * 2.0).rads() - 0.0).abs());
        assert!(0.0001 > (MyRads::new(PI * 3.0).rads() - PI).abs());
        assert!(0.0001 > (MyRads::new(-PI * 3.0).rads() - -PI).abs());
        assert!(0.0001 > (MyRads::new(PI * 4.5).rads() - PI / 2.0).abs());
        assert!(0.0001 > (MyRads::new(-PI * 4.5).rads() - -PI / 2.0).abs());
    }
    #[test]

    fn test_from_quat() {
        assert!(
            0.0001
                > (MyRads::from_quat(Quat::from_axis_angle(Vec3::new(-0.0, 0.0, 1.0), PI)).rads()
                    - PI)
                    .abs()
        );
        assert!(
            0.0001
                > (MyRads::from_quat(Quat::from_axis_angle(Vec3::new(-0.0, 0.0, -1.0), PI)).rads()
                    - -PI)
                    .abs()
        );
        assert!(
            0.0001
                > (MyRads::from_quat(Quat::from_axis_angle(Vec3::new(-0.0, 0.0, 1.0), -PI * 2.0))
                    .rads()
                    - 0.0)
                    .abs()
        );
        assert!(
            0.0001
                > (MyRads::from_quat(Quat::from_axis_angle(Vec3::new(0.0, -0.0, -1.0), -PI * 3.0))
                    .rads()
                    - -PI)
                    .abs()
        );
        assert!(
            0.0001
                > (MyRads::from_quat(Quat::from_axis_angle(Vec3::new(0.0, -0.0, 1.0), -PI * 4.5))
                    .rads()
                    - (-PI / 2.0))
                    .abs()
        );
    }
    #[test]
    fn test_most_direct_direction() {
        let rad1 = MyRads::new(PI / 2.0);
        let rad2 = MyRads::new(-PI / 2.0);
        assert!(rad1.get_most_direct_rotation_direction(rad2) == RotationDirection::Clockwise);
        let rad1 = MyRads::new(-PI / 3.0);
        let rad2 = MyRads::new(PI / 3.0);
        assert!(
            rad1.get_most_direct_rotation_direction(rad2) == RotationDirection::Counterclockwise
        );
        let rad1 = MyRads::new(PI / 3.0);
        let rad2 = MyRads::new(-PI / 3.0);
        assert!(rad1.get_most_direct_rotation_direction(rad2) == RotationDirection::Clockwise);
        assert_eq!(
            MyRads::new(PI / 1.1).get_most_direct_rotation_direction(MyRads::new(-PI / 1.1)),
            RotationDirection::Counterclockwise
        );
        assert!(
            MyRads::new(-PI / 1.1).get_most_direct_rotation_direction(MyRads::new(PI / 1.1))
                == RotationDirection::Clockwise
        );
    }
    #[test]
    fn test_closest_distance() {
        let rad1 = MyRads::new(PI);
        let rad2 = MyRads::new(-PI);
        assert_eq!(rad1.get_closest_distance_abs_radians(rad2), 0.);
        let rad2 = MyRads::new(0.);
        assert_eq!(rad1.get_closest_distance_abs_radians(rad2), PI);
        let rad1 = MyRads::new(-PI / 2.0);
        let rad2 = MyRads::new(-PI / 4.);
        assert_eq!(rad1.get_closest_distance_abs_radians(rad2), PI / 4.);
        let rad1 = MyRads::new(PI / 2.0);
        let rad2 = MyRads::new(-PI / 4.);
        assert_eq!(rad1.get_closest_distance_abs_radians(rad2), PI * (3. / 4.));
        let rad1 = MyRads::new(-PI / 1.1);
        let rad2 = MyRads::new(PI * (3. / 4.));
        assert_eq!(
            rad1.get_closest_distance_abs_radians(rad2),
            PI / 4. + (PI - (PI / 1.1))
        );
        assert_eq!(
            MyRads::new(PI / 1.1).get_closest_distance_abs_radians(MyRads::new(-PI * (3. / 4.))),
            PI / 4. + (PI - (PI / 1.1))
        );
        assert_eq!(
            MyRads::new(-PI / 10.).get_closest_distance_abs_radians(MyRads::new(PI / 10.)),
            2. * PI / 10.
        );
        assert_eq!(
            MyRads::new(PI / 10.).get_closest_distance_abs_radians(MyRads::new(-PI / 10.)),
            2. * PI / 10.
        );
        assert_eq!(
            MyRads::new(PI / 3.).get_closest_distance_abs_radians(MyRads::new(PI / 6.)),
            PI / 6.
        );
    }
    #[test]
    fn test_closest_angle_within_range() {
        let rad1 = MyRads::new(0.0);
        let range = MyRadsRange::new(
            MyRads::new(-PI / 2.0),
            MyRads::new(PI / 2.0),
            RotationDirection::Clockwise,
        );
        assert!(
            rad1.get_closest_angle_within_range(range).rads() == PI / 2.0,
            "actual result = {}",
            rad1.get_closest_angle_within_range(range).rads()
        );
        let range = MyRadsRange::new(
            MyRads::new(-PI / 2.0),
            MyRads::new(PI / 2.0),
            RotationDirection::Counterclockwise,
        );
        assert!(rad1.get_closest_angle_within_range(range).rads() == rad1.rads());
        let range = MyRadsRange::new(
            MyRads::new(-PI / 3.0),
            MyRads::new(PI / 2.0),
            RotationDirection::Clockwise,
        );
        assert!(rad1.get_closest_angle_within_range(range).rads() == -PI / 3.);
        let rad1 = MyRads::new(PI);
        let range = MyRadsRange::new(
            MyRads::new(-PI / 2.0),
            MyRads::new(PI / 2.0),
            RotationDirection::Counterclockwise,
        );
        assert!(rad1.get_closest_angle_within_range(range).rads() == PI / 2.0);
        let rad1 = MyRads::new(PI / 2.0);
        let range = MyRadsRange::new(
            MyRads::new(PI / 3.0),
            MyRads::new(PI / 1.5),
            RotationDirection::Counterclockwise,
        );
        assert!(rad1.get_closest_angle_within_range(range).rads() == PI / 2.0);
        let range = MyRadsRange::new(
            MyRads::new(PI / 1.5),
            MyRads::new(-PI / 1.01),
            RotationDirection::Counterclockwise,
        );
        assert!(rad1.get_closest_angle_within_range(range).rads() == PI / 1.5);
        let rad1 = MyRads::new(-PI / 1.1);
        let range: MyRadsRange =
            MyRadsRange::from_center_point_and_max_deviation(MyRads::new(PI / 4.), PI / 2.0);
        // println!("range = {:?}", range);
        assert_eq!(
            rad1.get_most_direct_rotation_direction(MyRads::new(PI / 4.)),
            RotationDirection::Clockwise
        );
        assert!(
            rad1.get_closest_angle_within_range(range).rads() == PI * (3. / 4.),
            "actual result = {}", // - 1/4 PI
            rad1.get_closest_angle_within_range(range).rads()
        );
        let rad1 = MyRads::new(PI / 1.1);
        assert!(rad1.get_closest_angle_within_range(range).rads() == PI * (3. / 4.));
    }
    #[test]
    fn test_slerp() {
        let rad1 = MyRads::new(PI / 2.0);
        let rad2 = MyRads::new(-PI / 2.0);
        assert!(rad1.slerp(rad2, 0.5).rads() == 0.0);
        let rad1 = MyRads::new(0.0);
        let rad2 = MyRads::new(-PI);
        assert!(rad1.slerp(rad2, 0.5).rads() == -PI / 2.0);
        let rad1 = MyRads::new(0.0);
        let rad2 = MyRads::new(PI);
        assert!(rad1.slerp(rad2, 1. / 3.).rads() == PI / 3.);
        let rad1 = MyRads::new(0.0);
        let rad2 = MyRads::new(-PI);
        assert!(rad1.slerp(rad2, 1. / 3.).rads() == -PI / 3.);
    }
    #[test]
    fn test_is_in_range() {
        assert_eq!(
            MyRadsRange::from_center_point_and_max_deviation(MyRads::new(0.), PI / 2.)
                .is_in_range(MyRads::new(PI / 3.)),
            true
        );
        assert_eq!(
            MyRadsRange::from_center_point_and_max_deviation(MyRads::new(0.), PI / 2.)
                .is_in_range(MyRads::new(PI / 1.1)),
            false
        );
        assert_eq!(
            MyRadsRange::from_center_point_and_max_deviation(MyRads::new(0.), PI / 2.)
                .is_in_range(MyRads::new(-PI / 4.)),
            true
        );
        assert_eq!(
            MyRadsRange::from_center_point_and_max_deviation(MyRads::new(0.), PI / 2.)
                .is_in_range(MyRads::new(-PI / 1.1)),
            false
        );
        assert_eq!(
            MyRadsRange::from_center_point_and_max_deviation(MyRads::new(-PI / 2.), PI / 4.)
                .is_in_range(MyRads::new(-PI / 4.)),
            true
        );
        assert_eq!(
            MyRadsRange::from_center_point_and_max_deviation(MyRads::new(-PI / 2.), PI / 4.)
                .is_in_range(MyRads::new(PI / 4.)),
            false
        );
        assert_eq!(
            MyRadsRange::from_center_point_and_max_deviation(MyRads::new(-PI / 2.), PI / 4.)
                .is_in_range(MyRads::new(-PI / 1.1)),
            false
        );
    }
    #[test]
    fn test_to_vec2_and_from_vec2() {
        let tests = vec![
            (MyRads::new(0.0), Vec2::new(0.0, 1.0)),
            (MyRads::new(PI / 2.0), Vec2::new(-1.0, 0.0)),
            (MyRads::new(PI), Vec2::new(0.0, -1.0)),
            (MyRads::new(-PI / 2.0), Vec2::new(1.0, 0.0)),
            (MyRads::new(0.0), Vec2::ZERO),
        ];
        let max_diff = 0.0001;
        for (rads, vec) in tests {
            let res = rads.to_vec2();
            let res_rads = MyRads::from_unit_vec2(vec);
            if (res_rads.rads() - rads.rads()).abs() > max_diff {
                panic!("expected: {:?}, got: {:?}", rads.rads(), res_rads.rads(),);
            }
        }
    }
}
