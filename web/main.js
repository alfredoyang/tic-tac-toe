import init, { WasmBoard } from '../wasm/pkg/tic_tac_toe_wasm.js';

async function run() {
  await init();
  const board = new WasmBoard();
  const canvas = document.getElementById('boardCanvas');
  const gl = canvas.getContext('webgl');

  if (!gl) {
    console.error('WebGL not supported');
    return;
  }

  const vsSource = `
    attribute vec2 a_position;
    uniform vec2 u_resolution;
    void main() {
      vec2 zeroToOne = a_position / u_resolution;
      vec2 clipSpace = zeroToOne * 2.0 - 1.0;
      gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
    }
  `;

  const fsSource = `
    precision mediump float;
    uniform vec4 u_color;
    void main() {
      gl_FragColor = u_color;
    }
  `;

  function createShader(gl, type, source) {
    const shader = gl.createShader(type);
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    return shader;
  }

  function createProgram(gl, vsSource, fsSource) {
    const vs = createShader(gl, gl.VERTEX_SHADER, vsSource);
    const fs = createShader(gl, gl.FRAGMENT_SHADER, fsSource);
    const program = gl.createProgram();
    gl.attachShader(program, vs);
    gl.attachShader(program, fs);
    gl.linkProgram(program);
    return program;
  }

  const program = createProgram(gl, vsSource, fsSource);
  const positionLocation = gl.getAttribLocation(program, 'a_position');
  const resolutionLocation = gl.getUniformLocation(program, 'u_resolution');
  const colorLocation = gl.getUniformLocation(program, 'u_color');
  const buffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
  gl.enableVertexAttribArray(positionLocation);
  gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0);

  const cellSize = canvas.width / 3;

  function checkGameEnd() {
    const winner = board.check_winner();
    const cells = board.get_cells();
    if (winner === 1) {
      alert('You win!');
      board.reset();
      drawBoard(board.get_cells());
      return true;
    }
    if (winner === 2) {
      alert('Computer wins!');
      board.reset();
      drawBoard(board.get_cells());
      return true;
    }
    if (!cells.includes(0)) {
      alert('Draw!');
      board.reset();
      drawBoard(board.get_cells());
      return true;
    }
    return false;
  }

  function drawLines(vertices, color, mode) {
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(vertices), gl.STATIC_DRAW);
    gl.uniform4fv(colorLocation, color);
    gl.drawArrays(mode, 0, vertices.length / 2);
  }

  function drawBoard(cells) {
    gl.viewport(0, 0, canvas.width, canvas.height);
    gl.clearColor(1, 1, 1, 1);
    gl.clear(gl.COLOR_BUFFER_BIT);
    gl.useProgram(program);
    gl.uniform2f(resolutionLocation, canvas.width, canvas.height);

    drawLines([
      cellSize, 0, cellSize, canvas.height,
      cellSize * 2, 0, cellSize * 2, canvas.height,
      0, cellSize, canvas.width, cellSize,
      0, cellSize * 2, canvas.width, cellSize * 2,
    ], [0, 0, 0, 1], gl.LINES);

    for (let i = 0; i < cells.length; i++) {
      const row = Math.floor(i / 3);
      const col = i % 3;
      const cx = col * cellSize + cellSize / 2;
      const cy = row * cellSize + cellSize / 2;
      const half = cellSize / 3;
      if (cells[i] === 1) {
        drawLines([
          cx - half, cy - half, cx + half, cy + half,
          cx + half, cy - half, cx - half, cy + half,
        ], [1, 0, 0, 1], gl.LINES);
      } else if (cells[i] === 2) {
        const verts = [];
        const radius = half;
        const seg = 24;
        for (let j = 0; j <= seg; j++) {
          const a = (j / seg) * Math.PI * 2;
          verts.push(cx + Math.cos(a) * radius, cy + Math.sin(a) * radius);
        }
        drawLines(verts, [0, 0, 1, 1], gl.LINE_STRIP);
      }
    }
  }

  canvas.addEventListener('click', (e) => {
    const rect = canvas.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    const col = Math.floor(x / cellSize);
    const row = Math.floor(y / cellSize);
    const idx = row * 3 + col;
    if (board.make_move(idx, 1)) {
      const ai = board.best_move(2);
      if (ai !== undefined && ai !== null) {
        board.make_move(ai, 2);
      }
    }
    drawBoard(board.get_cells());
    checkGameEnd();
  });

  drawBoard(board.get_cells());
  console.log('WASM initialized', board.get_cells());
}

run();
