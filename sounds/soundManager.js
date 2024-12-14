
export const SoundManager = {
  sounds: {
    sand: new Audio("./sounds/sand.wav"),
  },
  play(soundName) {
    const sound = this.sounds[soundName];
    if (sound) {
      sound.currentTime = 0; 
			sound.play();
    } else {
      console.error(`Sound "${soundName}" not found.`);
    }
  },
};

