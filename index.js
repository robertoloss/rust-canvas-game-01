import init, { get_delta_time } from "./pkg/game_canvas.js";

let wasm;

async function start() {
	wasm = await init();
	document.addEventListener('keydown', handleKeyDown);
	document.addEventListener('keyup', handleKeyUp);
	wasm.initialize()
	requestAnimationFrame(gameloop);
}

function getKeyCode(eventCode) {
	let keyCode;
	switch(eventCode) {
			case 'ArrowLeft': keyCode = 0; break;
			case 'ArrowRight': keyCode = 1; break;
			case 'KeyX': keyCode = 2; break;
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

function isMobile() {
    return /Mobi|Android/i.test(navigator.userAgent) || window.matchMedia("(max-width: 768px)").matches;
}

//let lastTimestamp = 0
function gameloop(
	//timestamp
) {
	//let deltaTime = (timestamp - lastTimestamp) * 10
	//lastTimestamp = timestamp;
	//console.log("get_delta_time: ", get_delta_time())
	try {
			wasm.render(
				isMobile()
				//deltaTime
			); 
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
const mobileLeft = document.getElementById('mobile-left')
const mobileRight = document.getElementById('mobile-right')

mobileJump.addEventListener('touchstart', ()=>{
	wasm.movement(2)
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



