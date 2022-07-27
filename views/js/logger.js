import {devicePixels} from "@sciter"

export default class Logger extends Element {
    colors = {
        blue: {
            match: "\u001b[1;34m",
            color: "#07c160"
        },
        yellow: {match: "\u001b[1;33m", color: "#faab0c"},
        red: {match: "\u001b[1;31m", color: "#ee0a24"},
        end: "\u001b[0m"
    }
    logs = []
    mode = 'hide'
    top = 0

    timer = -1

    constructor() {
        super();
        this.top = devicePixels(document.clientHeight - 26)
    }

    componentDidMount() {
        this.onGlobalEvent("logger-hide", () => {
            this.componentUpdate({
                mode: 'hide',
                top: devicePixels(document.clientHeight - 26)
            })
        })
        this.onGlobalEvent("log", (evt) => this.renderEventLogData(evt.data));
        document.onsizechange = ev => {
            if (this.mode === "hide") {
                clearTimeout(this.timer)
                this.timer = setTimeout(() => {

                    this.componentUpdate({
                        top: devicePixels(document.clientHeight - 26)
                    })


                }, 50)
            }
        }
    }

    renderEventLogData(data) {
        if (data.indexOf('\u001b\[1') > -1) {
            for (let color in this.colors) {
                let item = this.colors[color]
                if (data.indexOf(item.match) > -1) {
                    data = data.replace("\u001b[0m", "")
                    data = data.replace(item.match, `&&`)
                    this.logs.push({
                        color: item.color,
                        data: data.split("&&")
                    })
                    break
                }
            }
        } else {
            if (data.indexOf('[S]') > -1 ||
                data.indexOf('[SW]') > -1 ||
                data.indexOf('[SE]') > -1 ||
                data.indexOf('[SI]') > -1
            ) {
                data = data.replace("[S]", `&&[S]`)
                data = data.replace("[SE]", `&&[SE]`)
                data = data.replace("[SW]", `&&[SW]`)
                data = data.replace("[SI]", `&&[SI]`)
                this.logs.push({
                    color: "#ff00ff",
                    data: data.split("&&")
                })
            }
            if (data.indexOf('[I]') > -1) {
                data = data.replace("[I]", `&&[frpc] [I]`)
                this.logs.push({
                    color: "#07c160",
                    data: data.split("&&")
                })
            }
            if (data.indexOf('[E]') > -1) {
                data = data.replace("[E]", `&&[E]`)
                this.logs.push({
                    color: "#ee0a24",
                    data: data.split("&&")
                })
            }
            if (data.indexOf('[W]') > -1) {
                data = data.replace("[W]", `&&[W]`)
                this.logs.push({
                    color: "#faab0c",
                    data: data.split("&&")
                })
            }
        }

        this.componentUpdate()
    }

    ['on click at .clear']() {
        this.componentUpdate({
            logs: []
        })
    }

    ['on click at .hide']() {
        this.componentUpdate({
            mode: 'hide',
            top: devicePixels(document.clientHeight - 26)
        })
    }

    ['on click at .show']() {

        this.componentUpdate({
            top: 0,
            mode: 'show'
        })
    }

    render() {
        return <div id="logger-ctx" style={`top:${this.top}dip; transition: top linear 168ms;`}>
            <div id="logger-ctx-action">
                <span class="logger-title"><img src="/icon/log.svg"/> 日志</span>
                {
                    this.mode === 'hide' ?
                        <button class="show btn btn-text-default">
                            <img style={" vertical-align: middle;"} src="/icon/eye-open.svg" width="12dip"
                                 height="12dip"/>
                            显示
                        </button>
                        :
                        <button class="hide btn btn-text-default">
                            <img style={" vertical-align: middle;"} src="/icon/eye-off.svg" width="12dip"
                                 height="12dip"/>
                            隐藏
                        </button>
                }
                {
                    this.mode === 'show' && <button class="clear btn btn-text-default">
                        <img src="/icon/log-clear.svg" width="12dip" height="12dip"/>
                        清空
                    </button>
                }

            </div>
            <div id="logger-ctx-content">
                {
                    this.logs.map(e => {
                        return <p class="logger-ctx-content-row">
                            <span style="margin-right:4dip;display:inline-block">{e.data[0]}</span>
                            <span style={"color:" + e.color}>{e.data[1]}</span>
                        </p>
                    })
                }
            </div>
        </div>;
    }
}