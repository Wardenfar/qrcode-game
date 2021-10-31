import {read_qrcode, display_video} from "./qrcode";

import("./pkg").then(module => {
    module.run_app();
});

global.read_qrcode = read_qrcode
global.display_video = display_video