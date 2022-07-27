import {devicePixels} from "@sciter"
import * as env from "@env"

const view = Window.this;
const initWidth = 1024
const initHeight = 640
const minWidth = 620
const minHeight = 480
move_to_center()
reject_window_close_event()
set_main()
;(async () => {
    await set_try_icon()
})()

window.app_data_path = function () {
    return env.path("appdata")
}

function move_to_center() {
    const [width, height] = view.screenBox("workarea", "dimension")
    const x = devicePixels(width / 2 - initWidth / 2)
    const y = devicePixels(height / 2 - initHeight / 2)
    view.move(x, y, devicePixels(initWidth), devicePixels(initHeight))
}

function reject_window_close_event() {
    document.oncloserequest = function (evt) {
        evt.preventDefault()
        view.state = Window.WINDOW_HIDDEN
    }
}

function set_main() {
    view.minSize = [devicePixels(minWidth), devicePixels(minHeight)]
}

async function set_try_icon() {
    let icon = env.PLATFORM == 'Windows' ? 'app-b.svg' : 'app.svg'
    view.trayIcon({
        image: await Graphics.Image.load(`this://app/icon/${icon}`),
        text: "frper"
    });

    view.on("trayiconclick", evt => {
        view.state = Window.WINDOW_SHOWN
        view.isTopmost = true
        setTimeout(() => {
            view.isTopmost = false
        }, 50)
    });
}