import init, { 
	get_and_give_f64, 
	set_image,
} from "./pkg/game_canvas.js";
import { images } from "./images/images.js";
import { processImages } from "./images/processImages.js";

let wasm;

async function start() {
	wasm = await init();
	document.addEventListener('keydown', handleKeyDown);
	document.addEventListener('keyup', handleKeyUp);
	wasm.initialize()

	processImages(images, set_image)
	console.log("test")

	const mobileJump = document.getElementById('mobile-jump')
	const mobileCling = document.getElementById('mobile-cling')
	const mobileLeft = document.getElementById('mobile-left')
	const mobileRight = document.getElementById('mobile-right')

	const listeners = [
		{ element: mobileLeft, wasmNumber: 0 },
		{ element: mobileRight, wasmNumber: 1 },
		{ element: mobileCling, wasmNumber: 3 },
		{ element: mobileJump, wasmNumber: 2 },
	]
	const events = [ 
		{ type: 'touchstart', action: wasm.movement },
		{ type: 'touchend', action: wasm.stop_movement }
	]
	listeners.forEach(listener => {
		events.forEach(e => {
			listener.element.addEventListener(e.type, () => {
				e.action(listener.wasmNumber)
			})
		})
	})
	mobileCling.addEventListener('touchend', ()=>{
		wasm.stop_movement(3)
	})

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

	requestAnimationFrame(gameloop);
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
		canvas.height = bigScreen ? 640 : screenHeight;
		canvas.width = bigScreen ? 640 : screenHeight;
	} else {
		const bigScreen = screenWidth > 800
		canvas.width = bigScreen ? 640 : screenWidth
		canvas.height = bigScreen ? 640 : screenWidth
	}
	canvas.style.width = `${canvas.width}px`;
	canvas.style.height = `${canvas.height}px`;
}

window.addEventListener('resize', () => resizeCanvas(document.getElementById('gameCanvas')));
window.addEventListener('load', () => resizeCanvas(document.getElementById('gameCanvas')));
