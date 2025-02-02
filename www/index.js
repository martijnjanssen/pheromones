import { Universe, Cell } from "pheromones";
import { memory } from "pheromones/pheromones_bg";

const CELL_SIZE = 7;
const GRID_COLOR = "#CCCCCC";
const GROUND_COLOR = "#FFFFFF";
const WALL_COLOR = "#000000";
const START_COLOR = "#FF0000";
const END_COLOR = "#00FF00";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

let animationId = null;
const playPauseButton = document.getElementById("play-pause");
const setStartButton = document.getElementById("set-start");
const setEndButton = document.getElementById("set-end");

let placing = Cell.Wall;


const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = getColor(cells[idx]);

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

const getColor = (cell) => {
  switch (cell) {
  case Cell.Ground:
    return GROUND_COLOR;
  case Cell.Wall:
    return WALL_COLOR;
  case Cell.Start:
    return START_COLOR;
  case Cell.End:
    return END_COLOR;
  default:
    return "#FFFF00";
  }
}

const isPaused = () => {
    return animationId === null;
}

const play = () => {
    playPauseButton.textContent = "pause";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "play";
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

setStartButton.addEventListener("click", event => {
  placing = Cell.Start;
});

setEndButton.addEventListener("click", event => {
  placing = Cell.End;
});

canvas.addEventListener("click", event => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  if (placing === Cell.Wall) {
    universe.toggle_wall(row, col);
  } else if (placing === Cell.Start) {
    universe.set_start(row, col);
    placing = Cell.Wall;
  } else if (placing === Cell.End) {
    universe.set_end(row, col);
    placing = Cell.Wall;
  }

  drawGrid();
  drawCells();
});

const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
};

drawGrid();
drawCells();
play();
