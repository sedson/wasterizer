const vert = `
#version 300 es

precision highp float;

out vec2 texCoord;

void main(void) {
    float x = float((gl_VertexID & 1) << 2);
    float y = float((gl_VertexID & 2) << 1);
    texCoord.x = x * 0.5;
    texCoord.y = 1.0 - (y * 0.5);
    gl_Position = vec4(x - 1.0, y - 1.0, 0, 1);
}
`.trim();


const frag = `
#version 300 es

precision highp float;

uniform sampler2D uTex;

in vec2 texCoord;
out vec4 fragColor;

void main(void) {
    fragColor = texture(uTex, texCoord);
}
`.trim();


export class GLRenderer {
    constructor(canvas) {
        this.canvas = canvas;
        this.gl = canvas.getContext('webgl2', {
            preserveDrawingBuffer: true
        });
        this.gl.viewport(0, 0, canvas.width, canvas.height);
        this.program = this.createProgram(vert, frag);
        this.gl.useProgram(this.program);
        this.tex = null;
        this.setTex = this.program.createUniform('1i', 'uTex');
    }

    createProgram(vertex, fragment) {
        const gl = this.gl;
        const program = gl.createProgram();
        gl.attachShader(program, this.createShader(vertex, gl.VERTEX_SHADER));
        gl.attachShader(program, this.createShader(fragment, gl.FRAGMENT_SHADER));
        gl.linkProgram(program);

        program.createUniform = function (type, name) {
            const location = gl.getUniformLocation(program, name);
            return function (...args) {
                gl['uniform' + type](location, ...args);
            }
        };

        return program;
    }

    createShader(source, type) {
        const gl = this.gl;
        const shader = gl.createShader(type);
        gl.shaderSource(shader, source);
        gl.compileShader(shader);
        if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS))
            throw new Error(gl.getShaderInfoLog(shader));
        return shader;
    }

    texture(data) {
        const gl = this.gl;
        if (this.tex === null) {
            this.tex = gl.createTexture();
            gl.bindTexture(gl.TEXTURE_2D, this.tex);
            gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
            gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
            gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
            gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
        }
        const level = 0;
        const format = gl.RGBA;
        const width = this.canvas.width;
        const height = this.canvas.height;
        const border = 0;
        const srcType = gl.UNSIGNED_BYTE;
        gl.texImage2D(gl.TEXTURE_2D, level, format, width, height, border, format, srcType, data);
    }


    render() {
        this.setTex(0);
        this.gl.drawArrays(this.gl.TRIANGLE_FAN, 0, 3);
    }
}