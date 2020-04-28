import { Universe } from "wasm-game-of-life";


let speed = 1 / 15;

const CELL_SIZE = 1; // px

const width = 380 * 3;
const height = 250 * 3;
// Construct the universe, and get its width and height.
const universe = Universe.new(width, height);

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE) * height;
canvas.width = (CELL_SIZE) * width;

const gl = canvas.getContext('webgl2');

function render() {
  drawScene();
  universe.tick();
}

const renderLoop = () => {
  let frame;
  let accumulator = 0;

  const loop = () => {
    accumulator += speed;
    if (accumulator >= 1) {
      render();
      accumulator = 0;
    }
    frame = requestAnimationFrame(loop);
  }

  loop();

  return () => {
    cancelAnimationFrame(frame);
    accumulator = 0;
  };
};

function compileShader(source, type) {
  const shader = gl.createShader(type);

  // Send the source to the shader object
  gl.shaderSource(shader, source);

  // Compile the shader program
  gl.compileShader(shader);

  // See if it compiled successfully
  if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
    console.error('An error occurred compiling the shaders: ' + gl.getShaderInfoLog(shader));
    gl.deleteShader(shader);
    return null;
  }

  return shader;
}

function createProgram(shaders) {
  const shaderProgram = gl.createProgram();

  shaders.forEach(shader => {
    gl.attachShader(shaderProgram, shader);
  });

  linkProgram(shaderProgram);

  return shaderProgram;
}

function linkProgram(shaderProgram) {
  gl.linkProgram(shaderProgram);

  // If creating the shader program failed, alert

  if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
    throw new Error('Unable to initialize the shader program: ' + gl.getProgramInfoLog(shaderProgram));
  }

  return shaderProgram;
}

function createLineProgram() {
  const vertexShader = compileShader(`
    attribute vec2 position;
    
    void main() {
      gl_Position = vec4(position.x, position.y, 0.0, 1.0);
    }`,
    gl.VERTEX_SHADER
  );
  const fragmentShader = compileShader(
    `void main() {
      gl_FragColor = vec4(0.8, 0.8, 0.8, 1.0);
    }`,
    gl.FRAGMENT_SHADER);

  return createProgram([vertexShader, fragmentShader]);
}

function createCellProgram() {
  const vertexShader = compileShader(`
    attribute vec2 position;
    uniform float pointSize;
    
    void main() {
      gl_Position = vec4(position.x, position.y, 1.0, 1.0);
      gl_PointSize = pointSize;
    }`,
    gl.VERTEX_SHADER
  );
  const fragmentShader = compileShader(
    `void main() {
      gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    }`,
    gl.FRAGMENT_SHADER);

  return createProgram([vertexShader, fragmentShader]);
}

function makeBuffer(data, usage) {
  const glBuffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, glBuffer);
  gl.bufferData(gl.ARRAY_BUFFER, data, usage);

  return glBuffer;
}

function makeLinesBuffer() {
  return makeBuffer(universe.get_gl_line_buffer(), gl.STATIC_DRAW);
}

function makeCellsBufferAndLength() {
  const glCellsBuffer = universe.get_gl_cells_buffer();
  const webGLBuffer = makeBuffer(glCellsBuffer, gl.DYNAMIC_DRAW);

  return [webGLBuffer, glCellsBuffer.length / 2];
}

const linesBuffer = makeLinesBuffer();
const lineProgram = createLineProgram();
const lineVertexPosition = gl.getAttribLocation(lineProgram, 'position');

const cellProgram = createCellProgram();
const cellVertexPosition = gl.getAttribLocation(cellProgram, 'position');
const cellPointSitePosition = gl.getUniformLocation(cellProgram, 'pointSize');

function renderLines() {
  gl.bindBuffer(gl.ARRAY_BUFFER, linesBuffer);
  gl.vertexAttribPointer(
    lineVertexPosition,
    2,
    gl.FLOAT,
    false,
    0,
    0,
  );
  gl.enableVertexAttribArray(lineVertexPosition);

  gl.useProgram(lineProgram);
  gl.drawArrays(gl.LINES, 0, universe.get_gl_line_vertex_count());
}

function renderCells() {
  const [buffer, length] = makeCellsBufferAndLength();
  gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
  gl.vertexAttribPointer(
    cellVertexPosition,
    2,
    gl.FLOAT,
    false,
    0,
    0,
  );
  gl.enableVertexAttribArray(cellVertexPosition);

  gl.useProgram(cellProgram);

  gl.uniform1f(cellPointSitePosition, CELL_SIZE);
  gl.drawArrays(gl.POINTS, 0, length);
}

function drawScene() {
  gl.clearColor(1, 1, 1, 1.0);  // Clear to black, fully opaque
  gl.clearDepth(1);                 // Clear squares
  gl.enable(gl.DEPTH_TEST);           // Enable depth testing
  gl.depthFunc(gl.LEQUAL);            // Near things obscure far things

  // Clear the canvas before we start drawing on it.

  gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT);
  // renderLines();
  renderCells();
}

render();
let pause;
window.start = () => pause = renderLoop();
window.pause = () => pause && pause();
window.reset = () => {
  universe.reset();
  window.pause();
  render();
}
window.setSpeed = (vel) => speed = vel;
