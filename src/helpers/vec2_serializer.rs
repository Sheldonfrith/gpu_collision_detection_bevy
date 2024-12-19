use bevy::math::Vec2;

pub fn serialize_vec2(v: Vec2) -> String {
    format!("{} {}", v.x, v.y)
}
pub fn deserialize_vec2(v: String) -> Vec2 {
    let mut split = v.split_whitespace();
    let x = split.next().unwrap().parse::<f32>().unwrap();
    let y = split.next().unwrap().parse::<f32>().unwrap();
    Vec2::new(x, y)
}

//  implement tests for MyRads
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_vec2_serialize() {
        let vec = Vec2::new(1.0, 2.0);
        let serialized = serialize_vec2(vec);
        let deserialized = deserialize_vec2(serialized);
        assert_eq!(vec, deserialized);
    }
}
