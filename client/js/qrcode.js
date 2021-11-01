import wasmModule from 'rqrr-wasm';

global.rqrr_ready = false;

export function init_webcam() {
    let videoElem = document.querySelector('#webcam');
    if (global.stream) {
        videoElem.srcObject = window.stream
    } else {
        navigator.mediaDevices
            .getUserMedia({video: true, audio: false})
            .then(stream => {
                global.stream = stream;
                videoElem.srcObject = stream
            })
    }
}

function _read_qrcode_inner(canvas, callback) {
    let videoElem = document.querySelector('#webcam');
    canvas.getContext('2d').drawImage(videoElem, 0, 0, canvas.width, canvas.height);
    canvas.toBlob(blob => {
        const reader = new FileReader();

        reader.addEventListener('loadend', () => {
            const arrayBuffer = reader.result;
            const output = wasmModule.decode(new Uint8Array(arrayBuffer));

            if(output.includes("[Error]")) {
                setTimeout(() => _read_qrcode_inner(canvas, callback), 75)
                console.log(output);
            } else {
                callback(output)
            }
        });

        reader.readAsArrayBuffer(blob);
    });
}

export function read_qrcode() {
    const canvas = document.createElement('canvas');
    canvas.width = 300;
    canvas.height = 300;

    return new Promise((resolve, reject) => {
        let sub_callback = (value) => {
            resolve(value)
        }
        if (!global.rqrr_ready) {
            wasmModule.init().then(() => {
                global.rqrr_ready = true
                _read_qrcode_inner(canvas, sub_callback)
            })
        } else {
            _read_qrcode_inner(canvas, sub_callback)
        }
    })
}