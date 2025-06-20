// Minimal Macroquad WebGL loader for WASM

class WebGLRenderer {
    constructor(canvas) {
        this.canvas = canvas;
        this.gl = canvas.getContext('webgl') || canvas.getContext('experimental-webgl');
        if (!this.gl) {
            throw new Error('WebGL not supported');
        }
    }
}

function register_plugin(plugin) {
    // Plugin registration - not needed for basic functionality
}

function miniquad_add_plugin(imports, plugin) {
    // Add plugin imports to WASM
    Object.assign(imports.env, plugin);
}

let wasm;
let renderer;

async function load_wasm(wasm_path) {
    const canvas = document.getElementById("glcanvas");
    renderer = new WebGLRenderer(canvas);
    
    const imports = {
        env: {
            console_debug: (ptr, len) => {
                const str = get_string_from_wasm(ptr, len);
                console.log(str);
            },
            console_log: (ptr, len) => {
                const str = get_string_from_wasm(ptr, len);
                console.log(str);
            },
            console_warn: (ptr, len) => {
                const str = get_string_from_wasm(ptr, len);
                console.warn(str);
            },
            console_error: (ptr, len) => {
                const str = get_string_from_wasm(ptr, len);
                console.error(str);
            },
            set_cursor_grab: () => {},
            show_mouse: () => {},
            set_mouse_cursor: () => {},
            canvas_width: () => canvas.width,
            canvas_height: () => canvas.height,
            dpi_scale: () => window.devicePixelRatio || 1.0,
            high_dpi: () => false,
            screen_width: () => window.screen.width,
            screen_height: () => window.screen.height,
            
            // WebGL functions (minimal set)
            glClearColor: (r, g, b, a) => renderer.gl.clearColor(r, g, b, a),
            glClear: (mask) => renderer.gl.clear(mask),
            glViewport: (x, y, width, height) => renderer.gl.viewport(x, y, width, height),
            
            // Math functions
            rand: () => Math.random(),
            now: () => Date.now() / 1000.0,
            
            // Storage (minimal implementation)
            file_loaded: () => true,
            file_load_text: () => 0,
            file_load_text_cb: () => {},
        }
    };

    const wasm_module = await WebAssembly.instantiateStreaming(fetch(wasm_path), imports);
    wasm = wasm_module.instance;
    
    // Set up memory access
    setup_memory_access();
    
    // Set canvas size for high DPI
    const dpr = window.devicePixelRatio || 1;
    canvas.width = canvas.clientWidth * dpr;
    canvas.height = canvas.clientHeight * dpr;
    
    // Initialize and start the game
    if (wasm.exports.main) {
        wasm.exports.main();
    }
    
    return wasm_module;
}

let memory_u8;
let memory_u32;

function setup_memory_access() {
    memory_u8 = new Uint8Array(wasm.exports.memory.buffer);
    memory_u32 = new Uint32Array(wasm.exports.memory.buffer);
}

function get_string_from_wasm(ptr, len) {
    const bytes = memory_u8.subarray(ptr, ptr + len);
    return new TextDecoder('utf-8').decode(bytes);
}

// Export for global access
window.register_plugin = register_plugin;
window.miniquad_add_plugin = miniquad_add_plugin;
window.load_wasm = load_wasm;

export default load_wasm;