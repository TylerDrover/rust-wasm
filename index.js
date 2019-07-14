const WASM = import("./pkg/wasm.js");

WASM.then(WASM => {
    const width = 800;
    const height = 600;

    var start = null;
    var print = document.getElementById('print');
    var canvas = document.getElementById('canvas');
    const context = canvas.getContext('webgl');

    WASM.init(width, height);

    document.addEventListener('keydown', function(event) {
        WASM.key_down(event.keyCode);
    });

    document.addEventListener('keyup', function(event) {
        WASM.key_up(event.keyCode);
    });
    
    function render(timestamp) {
        var dt;
        if (start === null) start = timestamp;
        dt = timestamp - start;
        print.innerText = 'FPS: ' + (1000/dt);
        WASM.tick(dt, context);
        start = timestamp
        requestAnimationFrame(render);
    };
    requestAnimationFrame(render);
    
}).catch(console.error);;
