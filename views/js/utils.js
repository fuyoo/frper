import {devicePixels} from "@sciter"
export const view = Window.this


export function varType(val) {
    return Object.prototype.toString.call(val).slice(8, -1)
}

export function request(path, data, parent) {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            try {
                data = (varType(data) === "String" ? data : JSON.stringify(data)) || ""
            } catch (e) {
                data = data || ""
            }
            let v = view
            if (parent) {
                v = view.parent
            }
            v.xcall("fetch", path, data, data => {
                try {
                    resolve(JSON.parse(data))
                } catch (e) {
                    reject(data)
                }
            })
        })
    })
}

export function log(type, data) {
    let date = new Date();
    const add0 = t => t > 9 ? '' + t : "0" + t
    const time = `${date.getFullYear()}/${add0(date.getMonth() + 1)}/${add0(date.getDate())} ${add0(date.getHours())}:${add0(date.getMinutes())}:${add0(date.getSeconds())}`
    switch (type) {
        case "warning":
            type = "[SW]"
            break
        case "error":
            type = "[SE]"
            break
        default:
            type = "[SI]"
    }
    const log = `${time} ${type} ${data}`
    Window.post(new Event("log", {data: log}))
}

export function toPx(val) {
    return devicePixels(val)
}