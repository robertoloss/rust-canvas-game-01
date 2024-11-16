
export async function processImages(images, set_image) {
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
