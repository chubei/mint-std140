This library contains a single trait [AsStd140] which is implemented for [mint] types that can be converted to [std140] types.

# Examples

```rust
use mint_std140::AsStd140;

let mint_vector = mint::Vector2 { x: 0.0f32, y: 0.0f32 };
let std140_vector = mint_vector.as_std140();
assert_eq!(mint_vector.x, std140_vector[0]);
assert_eq!(mint_vector.y, std140_vector[1]);

let mint_matrix = mint::ColumnMatrix2 {
    x: mint::Vector2 { x: 0.0f32, y: 1.0f32 },
    y: mint::Vector2 { x: 2.0f32, y: 3.0f32 },
};
let std140_matrix = mint_matrix.as_std140();
```
