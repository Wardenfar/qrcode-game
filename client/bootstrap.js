import {read_qrcode, display_video} from "./js/qrcode";
import './css/main.css'

import("./pkg").then(module => {
    module.run_app();
});

global.read_qrcode = read_qrcode
global.display_video = display_video