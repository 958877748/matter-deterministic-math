# matter-deterministic-math

Deterministic 2D-vector math for [Matter.js](https://github.com/liabru/matter-js), compiled to WebAssembly.

## Usage (browser)

```html
<script type="module">
    import init, { Vec2 } from './wasm-pkg/matter_deterministic_math.js';
    await init();

    const a = new Vec2(1, 2);   // ← 去掉 .new
    const b = new Vec2(3, 4);
    const c = a.add(b);

    console.assert(c.x === 4 && c.y === 6, 'Vec2.add failed');
    document.body.innerHTML = '✅ 浏览器测试通过';
</script>