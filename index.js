import init, { 
	get_and_give_f64, 
	set_player_image, 
	set_player_image_cling, 
	set_player_image_cling_left, 
	set_player_image_left, 
	set_player_sheet_run_left, 
	set_player_sheet_run_right, 
	set_tile_image,
	set_lava_sheet,
	set_death_sheet,
	set_image,
} from "./pkg/game_canvas.js";

let wasm;

async function start() {
	wasm = await init();
	document.addEventListener('keydown', handleKeyDown);
	document.addEventListener('keyup', handleKeyUp);
	wasm.initialize()

	const images = [
		{ 
			src: './assets/Tile 8x8 v1.4-1.png.png' ,
			action: set_tile_image,
			name: 'tile',
			isSheet: false,
		},
		{ 
			src: './assets/Player 8x8 v2.0_tr-1.png.png',
			action: set_player_image,
			name: 'player',
			isSheet: false,
		},
		{ 
			src: './assets/Player 8x8 v2.0_tr_L-1.png.png',
			action: set_player_image_left,
			name: 'player_left',
			isSheet: false,
		},
		{ 
			src: './assets/player_2_0_cling_R-1.png.png',
			action: set_player_image_cling,
			name: 'player_cling',
			isSheet: false,
		},
		{ 
			src: './assets/player_2_0_cling_L-1.png.png',
			action: set_player_image_cling_left,
			name: 'player_cling_left',
			isSheet: false,
		},
		{
			src: './assets/run_R.png',
			action: set_player_sheet_run_right,
			name: 'player_run_right',
			isSheet: true,
		},
		{
			src: './assets/run_L.png',
			action: set_player_sheet_run_left,
			name: 'player_run_left',
			isSheet: true,
		},
		{
			src: './assets/Lava_1_3.png',
			action: set_lava_sheet,
			name: 'lava',
			isSheet: true,
		},
		{
			src: './assets/Death_1.png',
			action: set_death_sheet, 
			name: 'death',
			isSheet: true,
		},
	]
	async function processImages() {
		for (const image of images) {
			const img = new Image();
			try {
				await new Promise((resolve, reject) => {
					img.onload = () => resolve(img);
					img.onerror = () => reject(new Error(error));
					img.src = image.src;
				});
				try {
					set_image(image.name, image.isSheet, img)
				} catch(error) {
					console.error("Oops: Rust side error!", error, image.name)
				}
			} catch(error) {
				console.error("Oops! Image not loaded!", error, image.name)
			}
		}
	}
	processImages()
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
