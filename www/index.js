import { Universe, Glider } from "rust-wasm-game-of-life-coatsd";
import { memory } from "rust-wasm-game-of-life-coatsd/rust_wasm_game_of_life_coatsd_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const universe = Universe.new(64, 64);
const width = universe.width();
const height = universe.height();
const canvas = document.getElementById("game-of-life-canvas");
const playPauseButton = document.getElementById("play-pause");
const gliderBox = document.getElementById("glider-select");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
let animationId = null;

const ctx = canvas.getContext('2d');

const genOptionHtml = () => {
	let htmlString = "";
	for (let i = 0; i < 4; i++) {
		htmlString += `<option value=${i}>${Glider[i]}</option>`
	}
	return htmlString;
};

gliderBox.innerHTML = genOptionHtml();

const drawGrid = () => {
	ctx.beginPath();
	ctx.strokeStyle = GRID_COLOR;

	for (let i = 0; i <= width; i++) {
		ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
		ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
	}

	for (let j = 0; j <= height; j++) {
		ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
		ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
	}

	ctx.stroke();
}

const getIndex = (row, col) => {
	return row * width + col;
}

const drawCells = () => {
	const cellsPtr = universe.cells();
	const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

	ctx.beginPath();

	for (let row = 0; row < height; row++) {
		for (let col = 0; col < width; col ++) {
			const idx = getIndex(row, col);

			ctx.fillStyle = cells[idx] ?
				ALIVE_COLOR : DEAD_COLOR;

			ctx.fillRect(
				col * (CELL_SIZE + 1) + 1,
				row * (CELL_SIZE + 1) + 1,
				CELL_SIZE,
				CELL_SIZE
			);
		}
	}
	ctx.stroke();
}

const isPaused = () => {
	return animationId === null;
}

const play = () => {
	playPauseButton.textContent = "⏸";
	renderLoop();
}

const pause = () => {
	playPauseButton.textContent = "▶";
	cancelAnimationFrame(animationId);
	animationId = null;
}

playPauseButton.addEventListener('click', e => {
	if (isPaused()) {
		play()
	} else {
		pause()
	}
})

const renderLoop = () => {
	universe.tick();

	drawGrid();
	drawCells();

	animationId = requestAnimationFrame(renderLoop);
}

let ctrl = false;
let shift = false;

$(document).keydown((e) => {
	shift = e.which == "16";
	ctrl = e.which == "17";
	if (e.which == "49") gliderBox.value = 0;
	if (e.which == "50") gliderBox.value = 1;
	if (e.which == "51") gliderBox.value = 2;
	if (e.which == "52") gliderBox.value = 3;
	if (e.which == "80") (isPaused()) ? play() : pause();
});

canvas.addEventListener('click', e => {
	const boundingRect = canvas.getBoundingClientRect();

	const scaleX = canvas.width / boundingRect.width;
	const scaleY = canvas.height / boundingRect.height;

	const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
	const canvasTop = (event.clientY - boundingRect.top) * scaleY;

	const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
	const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

	if (ctrl) {
		universe.create_glider(row, col, gliderBox.value);
	} else if (shift) {
		universe.create_pulsar(row, col);
	} else {
		universe.toggle_cell(row, col);
	}

	drawGrid();
	drawCells();
});

drawGrid();
drawCells();
play();