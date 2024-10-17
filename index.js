import init, { get_and_give_f64, set_tile_image } from "./pkg/game_canvas.js";

let wasm;

async function start() {
	wasm = await init();
	document.addEventListener('keydown', handleKeyDown);
	document.addEventListener('keyup', handleKeyUp);
	wasm.initialize()

	const img = new Image();
	const loadImage = new Promise((resolve, reject) => {
			img.onload = () => resolve(img);
			img.onerror = reject;
	});
	
	img.src = './assets/Tile1.png';
	try {
		await loadImage;
		set_tile_image(img)
	} catch(error) {
		console.error("Oops!", error)
	}

	console.log(img)
	requestAnimationFrame(gameloop);
}

function getKeyCode(eventCode) {
	let keyCode;
	switch(eventCode) {
			case 'ArrowLeft': keyCode = 0; break;
			case 'ArrowRight': keyCode = 1; break;
			case 'KeyX': keyCode = 2; break;
			case 'KeyZ': keyCode = 3; break;
			default: keyCode = -1;
	}
	return keyCode
}

function handleKeyDown(event) {
	event.preventDefault();
	if (event.code === 'KeyX' && event.repeat) return;
	wasm.movement(getKeyCode(event.code));
}
function handleKeyUp(event) {
	event.preventDefault();
	wasm.stop_movement(getKeyCode(event.code));
}



let lastTimestamp = performance.now();
let fps = 0;
let frameCount = 0;
let fpsInterval = 1000; // Calculate FPS every second
let lastFpsUpdate = performance.now();

function gameloop(timestamp) {
	lastTimestamp = timestamp;
	frameCount++;

	if (timestamp - lastFpsUpdate >= fpsInterval) {
		fps = frameCount;
		frameCount = 0;  // Reset frame count
		lastFpsUpdate = timestamp;
	}
	get_and_give_f64(fps)
	try {
			wasm.render(); 
	} catch (error) {
			console.error('Error in game loop:', error);
	} 
	requestAnimationFrame(gameloop); 
}

start();

function resizeCanvas(canvas) {
	let screenWidth = window.innerWidth;
	let screenHeight = window.innerHeight;
	if (screenWidth / screenHeight > 1) {
		const bigScreen = screenHeight > 800
		canvas.height = bigScreen ? 800 : screenHeight;
		canvas.width = bigScreen ? 800 : screenHeight;
	} else {
		const bigScreen = screenWidth > 800
		canvas.width = bigScreen ? 800 : screenWidth
		canvas.height = bigScreen ? 800 : screenWidth
	}
	canvas.style.width = `${canvas.width}px`;
	canvas.style.height = `${canvas.height}px`;
}

window.addEventListener('resize', () => resizeCanvas(document.getElementById('gameCanvas')));
window.addEventListener('load', () => resizeCanvas(document.getElementById('gameCanvas')));

const mobileJump = document.getElementById('mobile-jump')
const mobileCling = document.getElementById('mobile-cling')
const mobileLeft = document.getElementById('mobile-left')
const mobileRight = document.getElementById('mobile-right')

mobileJump.addEventListener('touchstart', ()=>{
	wasm.movement(2)
})
mobileCling.addEventListener('touchmove', ()=>{
	console.log('touchmove')
	wasm.movement(3)
})
mobileCling.addEventListener('touchstart', ()=>{
	console.log('touchstart')
	wasm.movement(3)
})
mobileLeft.addEventListener('touchstart', ()=>{
	wasm.movement(0)
})
mobileRight.addEventListener('touchstart', ()=>{
	wasm.movement(1)
})
mobileJump.addEventListener('touchend', ()=>{
	wasm.stop_movement(2)
})
mobileLeft.addEventListener('touchend', ()=>{
	wasm.stop_movement(0)
})
mobileRight.addEventListener('touchend', ()=>{
	wasm.stop_movement(1)
})
mobileCling.addEventListener('touchend', ()=>{
	wasm.stop_movement(3)
})



