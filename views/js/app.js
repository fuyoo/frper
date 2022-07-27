import Logger from "./logger.js";
import Layout from "./layout.js";
import Set from "./set.js";
import {log, request, toPx, view} from "./utils";

class App extends Element {
    tab = 1;

    ["on click at #restart"]() {
        Window.this.xcall("fetch", "restart", "", data => {
            console.log(JSON.stringify(data))
        })
    }

    ["on click at #home"]() {
        Window.post(new Event("logger-hide"))
        this.componentUpdate({
            tab: 1
        })
    }

    ["on click at #set"]() {
        Window.post(new Event("logger-hide"))
        this.componentUpdate({
            tab: 2
        })
    }

    ["on click at .info"]() {

    }

    ["on click at .exit"]() {
        view.trayIcon("remove")
        view.xcall("exit")
    }

    ["on click at #about"]() {
        request("/version")
            .then(res => {
                view.modal({
                    height: toPx(400),
                    width: toPx(300),
                    url: "about.html",
                    parameters: {
                        version: res.data[0]
                    }
                })
            })
    }

    ["on click at #export"]() {
        let folder = view.selectFolder()
        if (folder) {
            folder = folder.replaceAll("file://", "")
            request("/export", folder)
                .then(res => {
                    if (res.code == 200) {
                        log("info", `config file has been saved to ${res.data}`)
                    } else {
                        view.modal(<error>{res.msg}</error>)
                    }
                })
        }
    }

    renderLayout() {
        if (this.tab == 1) {
            return <Layout/>
        }
        if (this.tab == 2) {
            return <Set/>
        }
    }

    render() {
        return <div class="app">
            <div class="tab">
                <img class="icon" src="/icon/app.svg" width="40dip" height="40dip"/>
                <div class="app-tab-item">
                    <img id="home" src="/icon/home.svg" class={this.tab == 1 ? "tab-btn tab-btn-active" : "tab-btn"}/>
                    <div class="tip">主页</div>
                </div>
                <div class="app-tab-item">
                    <img id="set" src="/icon/settings.svg"
                         class={this.tab == 2 ? "tab-btn tab-btn-active" : "tab-btn"}/>
                    <div class="tip">设置</div>
                </div>
                <div class="app-tab-item">
                    <img id={"about"} src="/icon/info.svg" class={"tab-btn about"}/>
                    <div class="tip">关于</div>
                </div>
                <div class="app-tab-item">
                    <img id={"export"} src="/icon/export.svg" class={"tab-btn about"}/>
                    <div class="tip">导出配置文件</div>
                </div>
                <div class="app-tab-item">
                    <img src="/icon/exit.svg" class={"tab-btn exit"}/>
                    <div class="tip">退出</div>
                </div>
            </div>
            <div class="content">
                <div class="content-layout">
                    {this.renderLayout()}
                </div>
                <div class="logger-place-holder-bar"></div>
                <Logger></Logger>
            </div>
        </div>
    }
}

document.body.patch(<App/>)