import init, { HeatHazeEngine } from './backend/pkg/backend.js';

let wasm = null;
let animationFrameId = null;

async function initWasm() {
    if (!wasm) {
        wasm = await init();
    }
    return wasm;
}

async function startAnimation(imageFile) {
    // Show loading indicator
    const loadingEl = document.getElementById('loading');
    const canvasContainer = document.getElementById('canvasContainer');
    loadingEl.style.display = 'block';
    canvasContainer.style.display = 'none';

    // Cancel any existing animation
    if (animationFrameId) {
        cancelAnimationFrame(animationFrameId);
    }

    // 1. Initialize WASM and get access to its memory
    const wasmModule = await initWasm();

    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext('2d', { willReadFrequently: true });

    // Load the uploaded image
    const img = new Image();
    const imageUrl = URL.createObjectURL(imageFile);

    img.onload = async () => {
        URL.revokeObjectURL(imageUrl); // Clean up the object URL

        canvas.width = img.width;
        canvas.height = img.height;
        ctx.drawImage(img, 0, 0);

        // 2. Extract initial pixels and send to Rust constructor
        const initialData = ctx.getImageData(0, 0, canvas.width, canvas.height);
        const engine = new HeatHazeEngine(canvas.width, canvas.height, initialData.data);

        // 3. Create a view into WASM memory at the pointer location
        // This points directly to self.dst_buffer in Rust
        const memoryView = new Uint8ClampedArray(
            wasmModule.memory.buffer,
            engine.get_buffer_ptr(),
            canvas.width * canvas.height * 4
        );

        // Hide loading, show canvas
        loadingEl.style.display = 'none';
        canvasContainer.style.display = 'block';

        function animate(time) {
            // 4. Update the effect in Rust
            // time / 1000 converts milliseconds to seconds for smoother math
            engine.apply_effect(time / 1000, 10.0);

            // 5. Wrap the WASM memory in ImageData and draw it
            // Because we used the pointer, memoryView always contains the latest frame
            const outputImageData = new ImageData(memoryView, canvas.width, canvas.height);
            ctx.putImageData(outputImageData, 0, 0);

            animationFrameId = requestAnimationFrame(animate);
        }

        animationFrameId = requestAnimationFrame(animate);
    };

    img.onerror = () => {
        loadingEl.style.display = 'none';
        alert('Error loading image. Please try another file.');
    };

    img.src = imageUrl;
}

// Set up file input handler
document.addEventListener('DOMContentLoaded', () => {
    const fileInput = document.getElementById('fileInput');
    const fileNameDisplay = document.getElementById('fileName');

    fileInput.addEventListener('change', (event) => {
        const file = event.target.files[0];
        if (file) {
            fileNameDisplay.textContent = `Selected: ${file.name}`;
            startAnimation(file);
        }
    });
});
