const defultPath = '/rs/lib.wasm';

export async function WasmModule(path = defultPath) {
  console.log(`Loading WASM Module... ${path}`);

  const decoder = new TextDecoder('utf-8');

  let memory = {
    buffer: new ArrayBuffer(),
  };

  const stringFromMemory = (ptr, len) => {
    try {
      let bytes = new Uint8Array(memory.buffer, ptr, len);
      return decoder.decode(bytes);
    } catch (e) {
      return `Failed to get string: ${ptr} —` + e;
    }
  }

  const imports = {
    env: {
      js_panic: (ptr, len) => {
        console.error('Rust panic!', stringFromMemory(ptr, len));
      },
      js_log: (ptr, len) => {
        console.log('%cRust log: ' + stringFromMemory(ptr, len), 'color:chocolate');
      }
    }
  }

  try {

    const wasm = await WebAssembly.instantiateStreaming(fetch(path), imports);
    memory = wasm.instance.exports.memory;
    console.log(wasm.instance.exports);
    return wasm.instance;

  } catch (e) {

    console.error('Failed to instantiate WebAssembly');
    console.error(e);
    return {};
  }
}


export class Renderer {
  constructor(instance, width, height) {
    this.width = width;
    this.height = height;
    this.wasm = instance.exports;
    this.ptr = this.wasm.new(width, height);
    this.data = this.initBuffer();
  }

  initBuffer() {
    let start = this.wasm.data_ptr(this.ptr);
    let size = this.wasm.data_size(this.ptr);
    return new Uint8Array(this.wasm.memory.buffer, start, size);
  }

  fill(r, g, b) {
    this.wasm.fill(this.ptr, r, g, b);
  }

  line(x1, y1, x2, y2, color) {
    this.wasm.line(this.ptr, x1, y1, x2, y2, color);
  }

  tri(x1, y1, x2, y2, x3, y3, color) {
    this.wasm.tri(this.ptr, x1, y1, x2, y2, x3, y3, color);
  }

  tri_wf(x1, y1, x2, y2, x3, y3, color) {
    this.wasm.tri_wf(this.ptr, x1, y1, x2, y2, x3, y3, color);
  }

  rect(x1, y1, x2, y2, color) {
    this.wasm.rect(this.ptr, x1, y1, x2, y2, color);
  }

  rect_wf(x1, y1, x2, y2, color) {
    this.wasm.rect_wf(this.ptr, x1, y1, x2, y2, color);
  }
}