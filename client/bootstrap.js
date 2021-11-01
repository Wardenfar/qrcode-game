import './css/main.css'
// import {init_webcam} from "./js/qrcode";

import("./pkg").then(module => {
    fetch("/game.toml")
        .then(r => r.text())
        .then(val => {
            global.game_toml_val = val;
            module.run_app();
        })
});

// global.init_webcam = init_webcam;