// use std::collections::HashMap;
// use std::hash::Hash;
//
// pub trait HashMapExt<K: Eq + Hash, V>: IntoIterator<Item = (K, V)> {
//     fn fold_hashmap<KF, VF>(self, kf: KF, vf: VF) -> HashMap<K, V>
//     where
//         KF: Fn(K) -> K,
//         VF: Fn(V) -> V,
//         Self: Sized,
//     {
//         self.into_iter().fold(HashMap::new(), |mut acc, (key, value)| {
//             acc.insert(kf(key), vf(value));
//             acc
//         })
//     }
// }

// impl<K: Eq + Hash, V> HashMapExt1<K, V> for HashMap<K, V> {
//     fn fold_hashmap<KF, VF>(self, kf: KF, vf: VF) -> HashMap<K, V>
//     where
//         KF: Fn(K) -> K,
//         VF: Fn(V) -> V,
//     {
//         self.into_iter().fold(HashMap::new(), |mut acc, (key, value)| {
//             acc.insert(kf(key), vf(value));
//             acc
//         })
//     }
// }
//
// pub trait HashMapExt<K: Eq + Hash, V> {
//     fn fold_hashmap<KF, VF, I>(self, kf: KF, vf: VF) -> HashMap<K, V>
//     where
//         Self: Iterator<Item = (K, V)> + Sized,
//         KF: Fn(K) -> K,
//         VF: Fn(V) -> V,
//     {
//         self.fold(HashMap::new(), |mut acc, (key, value)| {
//             acc.insert(kf(key), vf(value));
//             acc
//         })
//     }
// }
