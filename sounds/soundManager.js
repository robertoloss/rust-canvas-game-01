
export const SoundManager = {
  sounds: {
    sand: new Audio("./sounds/sand.wav"),
    jump: new Audio("./sounds/jump.wav"),
    cling: new Audio("./sounds/cling.wav"),
  },
  play(soundName) {
    const sound = this.sounds[soundName];
    if (sound) {
      sound.currentTime = 0; 
      switch (soundName) {
        case "sand":
          sound.volume = 0.3; 
          break;
        case "jump":
          sound.volume = 0.4; 
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

