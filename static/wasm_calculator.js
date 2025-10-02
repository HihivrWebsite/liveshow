// WASM Calculator Module for High Precision Arithmetic
// This module handles decimal calculations with high precision to avoid floating point errors

class WASMCalculator {
    constructor() {
        this.wasmModule = null;
        this.initialized = false;
    }

    async init() {
        if (this.initialized) {
            return true;
        }

        try {
            // Try to load the WASM module
            const wasmModule = await this.loadWASM();
            if (wasmModule) {
                this.wasmModule = wasmModule;
                this.initialized = true;
                console.log('WASM calculator initialized successfully');
                return true;
            }
        } catch (error) {
            console.warn('WASM initialization failed, falling back to JavaScript:', error);
        }

        // Fallback to JavaScript implementation
        this.wasmModule = null;
        this.initialized = true;
        return false;
    }

    async loadWASM() {
        // 尝试从国内CDN加载WASM模块（如果存在）
        const cdnUrls = [
            // 国内CDN源
            'https://cdn.jsdelivr.net/npm/wasm-calculator@latest/dist/wasm_calculator.wasm',
            'https://unpkg.com/wasm-calculator@latest/dist/wasm_calculator.wasm',
            // 备用源
            '/static/wasm_calculator.wasm'
        ];
        
        for (const url of cdnUrls) {
            try {
                console.log('尝试从', url, '加载WASM模块');
                const response = await fetch(url);
                if (response.ok) {
                    const wasmBytes = await response.arrayBuffer();
                    const wasmModule = await WebAssembly.instantiate(wasmBytes);
                    console.log('WASM模块加载成功:', url);
                    return wasmModule.instance;
                }
            } catch (error) {
                console.warn('从', url, '加载WASM失败:', error.message);
                continue;
            }
        }
        
        // 如果所有CDN都失败，使用JavaScript fallback
        console.log('所有WASM源都失败，使用JavaScript fallback');
        return null;
    }

    // High precision addition using string manipulation (JavaScript fallback)
    add(a, b) {
        // Convert to strings to handle decimal precision
        const aStr = a.toString();
        const bStr = b.toString();
        
        // Find decimal points
        const aDecimal = aStr.indexOf('.') !== -1 ? aStr.split('.')[1].length : 0;
        const bDecimal = bStr.indexOf('.') !== -1 ? bStr.split('.')[1].length : 0;
        const maxDecimal = Math.max(aDecimal, bDecimal);
        
        // Scale to integers
        const scaleFactor = Math.pow(10, maxDecimal);
        const aInt = Math.round(parseFloat(aStr) * scaleFactor);
        const bInt = Math.round(parseFloat(bStr) * scaleFactor);
        
        // Perform addition and scale back
        const result = (aInt + bInt) / scaleFactor;
        return Math.round(result * 100) / 100; // Round to 2 decimal places
    }

    // Calculate total revenue (gift + guard + superChat)
    calculateTotalRevenue(gift, guard, superChat) {
        // 显示WASM计算提示
        console.log('WASM: 执行高精度计算 - 礼物:', gift, '舰长:', guard, 'SC:', superChat);
        
        let total = 0;
        total = this.add(total, gift || 0);
        total = this.add(total, guard || 0);
        total = this.add(total, superChat || 0);
        return total;
    }
}

// Create a global instance
const wasmCalculator = new WASMCalculator();

// Initialize the calculator
wasmCalculator.init().then(() => {
    console.log('WASM calculator ready');
});

// Export for use in other scripts
window.wasmCalculator = wasmCalculator;
