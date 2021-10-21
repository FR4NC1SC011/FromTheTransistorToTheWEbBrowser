// #[macro_export]
// macro_rules! GROW_CAPACITY {
//     ($capacity:expr) => {
//         (($capacity) < 8 ? 8 : ($capacity) * 2)
//         // if ($capacity) < 8 {
//         //     return 8;
//         // } else {
//         //     return $capacity * 2;
//         // }
//     };
// }

pub fn grow_capacity(capacity: usize) -> usize {
    if capacity < 8 {
        return 8;
    } else {
        return capacity * 2;
    }
}
