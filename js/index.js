import Fps from "./fps";

const run = async () => {
  const { Universe } = await import("../crate/pkg");
  const { memory } = await import("../crate/pkg/mywasmrs_bg");

  const CELL_SIZE = 5; // px
  const GRID_COLOR = "#CCCCCC";
  const DEAD_COLOR = "#FFFFFF";
  const ALIVE_COLOR = "#000000";

  const width = 64;
  const height = 64;
  const universe = Universe.new(height, width);

  const canvas = document.getElementById("game-of-life-canvas");
  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;

  canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
  });

  const ctx = canvas.getContext("2d");

  let animationId = null;
  const fps = new Fps();

  const renderLoop = () => {
    fps.render();
    universe.tick();

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
  };

  const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // vertical lines
    for (let i = 0; i <= width; i++) {
      ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
      ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // horizontal lines
    for (let j = 0; j <= height; j++) {
      ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
      ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
  };

  const getIndex = (row, col) => {
    return row * width + col;
  };

  const isAlive = (row, col, arr) => {
    const idx = getIndex(row, col);
    const byte_idx = Math.floor(idx / 8);
    const mask = 1 << idx % 8;
    return (arr[byte_idx] & mask) === mask;
  };

  const drawCells = () => {
    const cellsPtr = universe.cells_ptr();
    const cells = new Uint8Array(memory.buffer, cellsPtr, (width * height) / 8);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
      for (let col = 0; col < width; col++) {
        ctx.fillStyle = isAlive(row, col, cells) ? ALIVE_COLOR : DEAD_COLOR;

        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        );
      }
    }

    ctx.stroke();
  };

  const isPaused = () => {
    return animationId === null;
  };

  const playPauseButton = document.getElementById("play-pause");

  const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
  };

  const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
  };

  playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
      play();
    } else {
      pause();
    }
  });

  play();
};

run();
