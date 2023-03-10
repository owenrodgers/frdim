# frdim
Project for CSC-400
3d Graphics written entirely from scratch without the use of OPENGL and other rendering libraries.\
Only hard dependency is Rust-SDL2 which is used for color and window.\
https://github.com/Rust-SDL2/rust-sdl2 

## Hypercone-Hyperplane Intersection
Produces 3d cross-section below.
```rust
    let hc = HyperCone::new(2.0, 5.0);
    let hp = HyperPlane::new(0.0, 1.5, 1.0, 2.0);

    let mut hyp_c = HyperConic::new(hc, hp);
    let mut hyperconic_model = JiveModel::new(vertices_from_hyperconic(&hyp_c));
```

https://user-images.githubusercontent.com/74275129/224402910-4eeb6954-9390-4ff0-b2f2-67a509a0eeeb.mp4


