
export const SoundManager = {
  sounds: {
    sand: new Audio("./sounds/sand.wav"),
    jump: new Audio("./sounds/jump.wav"),
    jump2: new Audio("./sounds/jump2.wav"),
    cling: new Audio("./sounds/cling.wav"),
    coin: new Audio("./sounds/coin.wav"),
    ground: new Audio("./sounds/ground2.wav"),
  },
  play(soundName) {
    const sound = this.sounds[soundName];
    if (sound) {
      sound.currentTime = 0; 
      switch (soundName) {
        case "sand":
          sound.volume = 0.2; 
          break;
        case "jump":
          sound.volume = 0.4; 
          break;
        case "jump2":
          sound.volume = 1; 
          break;
        case "coin":
          sound.volume = 0.5; 
          break;
        case "ground":
          sound.volume = 0.5; 
          break;
        default:
          sound.volume = 0.5; 
      }
			sound.play();
    } else {
      console.error(`Sound "${soundName}" not found.`);
    }
  },
};

