import { GLRenderer } from "./gl-render.js";
import { WasmModule, Renderer } from './render-interface.js';
import { Color } from "./color.js";
import { load_obj } from "./model-loader.js";


const torus = await (load_obj('/models/monkey.obj'));
console.log(torus)

const bg = (new Color(200, 200, 200)).to_i32();
const fg = (new Color(240, 80, 20, 255)).to_i32();
const blue = (new Color(60, 60, 240, 255)).to_i32();
const black = (new Color(0, 0, 0, 255)).to_i32();

const wasmModule = await WasmModule();

const canvasElem = document.getElementById("canvas");

const { width, height } = canvasElem;

const glRenderer = new GLRenderer(canvasElem);

const canvas = new Renderer(wasmModule, width, height);

const count = 1000;
let randomValues = new Array(count).fill(0).map(n => {
  return Math.round(Math.random() * (n % 2 === 0 ? width : height));
});


function demo_1() {
  let time = performance.now();

  for (let i = 0; i < width; i += 5) {

    let y1 = height * 0.2 * Math.sin((i) * 0.02 + time * 0.001);
    let y2 = height * 0.4 * Math.cos((i + 5) * 0.013 + time * 0.002);
    let x1 = i;
    let x2 = i + 100 * Math.cos(i * 0.01 + time * 0.0006);

    canvas.line(x1, y1 + height / 2, x2, y2 + height / 2, fg);
  }
}

function demo_2() {
  let c = 0;
  for (let i = 0; i < 100; i++) {
    const x1 = randomValues[c++ % count];
    const y1 = randomValues[c++ % count];
    const x2 = randomValues[c++ % count];
    const y2 = randomValues[c++ % count];
    const x3 = randomValues[c++ % count];
    const y3 = randomValues[c++ % count];

    const col = new Color(randomValues[c++ % count], randomValues[c++ % count], randomValues[c++ % count]);
    canvas.tri(x1, y1, x2, y2, x3, y3, col.to_i32());

    const col2 = new Color(randomValues[c++ % count], randomValues[c++ % count], randomValues[c++ % count]);
    canvas.tri_wf(x1, y1, x2, y2, x3, y3, black);
  }
}


function demo_3() {
  const center = [width / 2, height / 2];
  const s = width / 2;

  let c = 0;
  for (let face of torus.faces) {

    let x0 = torus.positions[face[0]][0] * s + center[0];
    let y0 = torus.positions[face[0]][2] * s + center[1];

    let x1 = torus.positions[face[1]][0] * s + center[0];
    let y1 = torus.positions[face[1]][2] * s + center[1];

    let x2 = torus.positions[face[2]][0] * s + center[0];
    let y2 = torus.positions[face[2]][2] * s + center[1];

    const col = new Color(randomValues[c++ % count], randomValues[c++ % count], randomValues[c++ % count]);
    canvas.tri(x0, y0, x1, y1, x2, y2, col.to_i32());
    canvas.tri_wf(x0, y0, x1, y1, x2, y2, black);
  }
}



function loop() {
  const start = performance.now();

  canvas.fill(bg);

  demo_3();

  glRenderer.texture(canvas.data);
  glRenderer.render();

  const end = performance.now();
  document.getElementById('timer').innerText = end - start;
  requestAnimationFrame(loop);
}

loop();