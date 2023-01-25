use std::collections::HashMap;

let mut abilities = HashMap::from([
    ("raw attack", abilities::Abilities::new("raw attack", Physical, 0, 1.5, 0)),
    ("raw magic", abilities::Abilities::new("raw magic", Magic, 0, 0, 1.5)),
    ("magic attack", abilities::Abilities::new("magic attack", Hybrid, 0, 1.25, 1.25))
]);


