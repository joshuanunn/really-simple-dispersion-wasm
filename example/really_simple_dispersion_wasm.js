
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
/**
*/
export function main() {
    wasm.main();
}

/**
*/
export class MetHour {

    static __wrap(ptr) {
        const obj = Object.create(MetHour.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_methour_free(ptr);
    }
    /**
    * @returns {number}
    */
    get u() {
        var ret = wasm.__wbg_get_methour_u(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set u(arg0) {
        wasm.__wbg_set_methour_u(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get phi() {
        var ret = wasm.__wbg_get_methour_phi(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set phi(arg0) {
        wasm.__wbg_set_methour_phi(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get pgcat() {
        var ret = wasm.__wbg_get_methour_pgcat(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set pgcat(arg0) {
        wasm.__wbg_set_methour_pgcat(this.ptr, arg0);
    }
}
/**
* RSDM maintains the current state
*/
export class RSDM {

    static __wrap(ptr) {
        const obj = Object.create(RSDM.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_rsdm_free(ptr);
    }
    /**
    * @returns {Source}
    */
    get source() {
        var ret = wasm.__wbg_get_rsdm_source(this.ptr);
        return Source.__wrap(ret);
    }
    /**
    * @param {Source} arg0
    */
    set source(arg0) {
        _assertClass(arg0, Source);
        var ptr0 = arg0.ptr;
        arg0.ptr = 0;
        wasm.__wbg_set_rsdm_source(this.ptr, ptr0);
    }
    /**
    * @returns {number}
    */
    get x_min() {
        var ret = wasm.__wbg_get_rsdm_x_min(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set x_min(arg0) {
        wasm.__wbg_set_rsdm_x_min(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get x_max() {
        var ret = wasm.__wbg_get_rsdm_x_max(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set x_max(arg0) {
        wasm.__wbg_set_rsdm_x_max(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y_min() {
        var ret = wasm.__wbg_get_rsdm_y_min(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set y_min(arg0) {
        wasm.__wbg_set_rsdm_y_min(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y_max() {
        var ret = wasm.__wbg_get_rsdm_y_max(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set y_max(arg0) {
        wasm.__wbg_set_rsdm_y_max(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get z_min() {
        var ret = wasm.__wbg_get_rsdm_z_min(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set z_min(arg0) {
        wasm.__wbg_set_rsdm_z_min(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get z_max() {
        var ret = wasm.__wbg_get_rsdm_z_max(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set z_max(arg0) {
        wasm.__wbg_set_rsdm_z_max(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get x_spacing() {
        var ret = wasm.__wbg_get_rsdm_x_spacing(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set x_spacing(arg0) {
        wasm.__wbg_set_rsdm_x_spacing(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y_spacing() {
        var ret = wasm.__wbg_get_rsdm_y_spacing(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set y_spacing(arg0) {
        wasm.__wbg_set_rsdm_y_spacing(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get z_spacing() {
        var ret = wasm.__wbg_get_rsdm_z_spacing(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set z_spacing(arg0) {
        wasm.__wbg_set_rsdm_z_spacing(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get x_points() {
        var ret = wasm.__wbg_get_rsdm_x_points(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set x_points(arg0) {
        wasm.__wbg_set_rsdm_x_points(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y_points() {
        var ret = wasm.__wbg_get_rsdm_y_points(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set y_points(arg0) {
        wasm.__wbg_set_rsdm_y_points(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get z_points() {
        var ret = wasm.__wbg_get_rsdm_z_points(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set z_points(arg0) {
        wasm.__wbg_set_rsdm_z_points(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get hours() {
        var ret = wasm.__wbg_get_rsdm_hours(this.ptr);
        return ret >>> 0;
    }
    /**
    * @param {number} arg0
    */
    set hours(arg0) {
        wasm.__wbg_set_rsdm_hours(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get wspd() {
        var ret = wasm.__wbg_get_rsdm_wspd(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set wspd(arg0) {
        wasm.__wbg_set_rsdm_wspd(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get wdir() {
        var ret = wasm.__wbg_get_rsdm_wdir(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set wdir(arg0) {
        wasm.__wbg_set_rsdm_wdir(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get roughness() {
        var ret = wasm.__wbg_get_rsdm_roughness(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set roughness(arg0) {
        wasm.__wbg_set_rsdm_roughness(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get pgcat() {
        var ret = wasm.__wbg_get_rsdm_pgcat(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set pgcat(arg0) {
        wasm.__wbg_set_rsdm_pgcat(this.ptr, arg0);
    }
    /**
    * @returns {RSDM}
    */
    static new() {
        var ret = wasm.rsdm_new();
        return RSDM.__wrap(ret);
    }
    /**
    * Change grid resolution
    * @param {string} value
    */
    set_resolution(value) {
        var ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.rsdm_set_resolution(this.ptr, ptr0, len0);
    }
    /**
    * Setup grids
    */
    setup_grids() {
        wasm.rsdm_setup_grids(this.ptr);
    }
    /**
    * @param {number} value
    */
    set_elevation(value) {
        wasm.__wbg_set_source_height(this.ptr, value);
    }
    /**
    * @param {number} value
    */
    set_diameter(value) {
        wasm.__wbg_set_source_diameter(this.ptr, value);
    }
    /**
    * @param {number} value
    */
    set_velocity(value) {
        wasm.__wbg_set_source_velocity(this.ptr, value);
    }
    /**
    * @param {number} value
    */
    set_temp(value) {
        wasm.__wbg_set_source_temp(this.ptr, value);
    }
    /**
    * @param {number} value
    */
    set_wdir(value) {
        wasm.__wbg_set_rsdm_wdir(this.ptr, value);
    }
    /**
    * @param {number} value
    */
    set_wspd(value) {
        wasm.__wbg_set_rsdm_wspd(this.ptr, value);
    }
    /**
    * @param {string} value
    */
    set_roughness(value) {
        var ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.rsdm_set_roughness(this.ptr, ptr0, len0);
    }
    /**
    * @param {string} value
    */
    set_pgcat(value) {
        var ptr0 = passStringToWasm0(value, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        wasm.rsdm_set_pgcat(this.ptr, ptr0, len0);
    }
    /**
    * @returns {number}
    */
    width() {
        var ret = wasm.__wbg_get_rsdm_x_points(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    height() {
        var ret = wasm.__wbg_get_rsdm_y_points(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    altitude() {
        var ret = wasm.__wbg_get_rsdm_z_points(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    r_grid() {
        var ret = wasm.rsdm_r_grid(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    h_grid() {
        var ret = wasm.rsdm_h_grid(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    r_grid_max() {
        var ret = wasm.rsdm_r_grid_max(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    h_grid_max() {
        var ret = wasm.rsdm_h_grid_max(this.ptr);
        return ret;
    }
    /**
    */
    clear_grids() {
        wasm.rsdm_clear_grids(this.ptr);
    }
    /**
    * @param {number} hours
    */
    iter_disp(hours) {
        wasm.rsdm_iter_disp(this.ptr, hours);
    }
    /**
    */
    update_png() {
        wasm.rsdm_update_png(this.ptr);
    }
    /**
    * @param {boolean} random
    * @returns {MetHour}
    */
    gen_met(random) {
        var ret = wasm.rsdm_gen_met(this.ptr, random);
        return MetHour.__wrap(ret);
    }
}
/**
* Source defines the stack (emission source) parameters
*/
export class Source {

    static __wrap(ptr) {
        const obj = Object.create(Source.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_source_free(ptr);
    }
    /**
    * @returns {number}
    */
    get x() {
        var ret = wasm.__wbg_get_methour_u(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set x(arg0) {
        wasm.__wbg_set_methour_u(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get y() {
        var ret = wasm.__wbg_get_methour_phi(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set y(arg0) {
        wasm.__wbg_set_methour_phi(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get height() {
        var ret = wasm.__wbg_get_source_height(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set height(arg0) {
        wasm.__wbg_set_source_height(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get diameter() {
        var ret = wasm.__wbg_get_source_diameter(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set diameter(arg0) {
        wasm.__wbg_set_source_diameter(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get velocity() {
        var ret = wasm.__wbg_get_source_velocity(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set velocity(arg0) {
        wasm.__wbg_set_source_velocity(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get temp() {
        var ret = wasm.__wbg_get_source_temp(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set temp(arg0) {
        wasm.__wbg_set_source_temp(this.ptr, arg0);
    }
    /**
    * @returns {number}
    */
    get emission() {
        var ret = wasm.__wbg_get_source_emission(this.ptr);
        return ret;
    }
    /**
    * @param {number} arg0
    */
    set emission(arg0) {
        wasm.__wbg_set_source_emission(this.ptr, arg0);
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {

        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {

        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbg_new_3a746f2619705add = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_f54d3a6dadb199ca = function(arg0, arg1) {
        var ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
        var ret = getObject(arg0) === getObject(arg1);
        return ret;
    };
    imports.wbg.__wbg_self_ac379e780a0d8b94 = function(arg0) {
        var ret = getObject(arg0).self;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_crypto_1e4302b85d4f64a2 = function(arg0) {
        var ret = getObject(arg0).crypto;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_getRandomValues_1b4ba144162a5c9e = function(arg0) {
        var ret = getObject(arg0).getRandomValues;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_require_6461b1e9a0d7c34a = function(arg0, arg1) {
        var ret = require(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getRandomValues_1ef11e888e5228e9 = function(arg0, arg1, arg2) {
        getObject(arg0).getRandomValues(getArrayU8FromWasm0(arg1, arg2));
    };
    imports.wbg.__wbg_randomFillSync_1b52c8482374c55b = function(arg0, arg1, arg2) {
        getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

export default init;

