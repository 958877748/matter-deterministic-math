# matter-deterministic-math

Deterministic 2D-vector math for [Matter.js](https://github.com/liabru/matter-js), compiled to WebAssembly.

## Usage (browser)

```html
<script type="module">
  import init, { Vec2 } from 'https://unpkg.com/@<你>/matter-deterministic-math/pkg/matter_deterministic_math.js';
  await init();
  console.log(Vec2.new(1, 2).add(Vec2.new(3, 4))); // {x: 4, y: 6}
</script>