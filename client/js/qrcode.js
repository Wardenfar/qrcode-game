import wasmModule from 'rqrr-wasm';

export async function display_video(videoElem) {
    videoElem.srcObject = await navigator.mediaDevices.getUserMedia({video: true, audio: false});
}

export async function read_qrcode(videoElem) {
    await wasmModule.init()
    const canvas = document.createElement('canvas');
    canvas.width = 300;
    canvas.height = 300;

    canvas.getContext('2d').drawImage(videoElem, 0, 0, canvas.width, canvas.height);
    canvas.toBlob(blob => {
        const reader = new FileReader();

        reader.addEventListener('loadend', () => {
            const arrayBuffer = reader.result;
            const output = wasmModule.decode(new Uint8Array(arrayBuffer));

            console.log("DECODED QR: ", output); // the string output here!
        });

        reader.readAsArrayBuffer(blob);
    });
}