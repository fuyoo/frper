import {request,toPx} from "./utils";

const view = Window.this
let inputTimer = -1
export default class Set extends Element {
    baseParams = []
    authParams = []
    uiParams = []
    tab = "base"

    componentDidMount() {
        request("/settings")
            .then(res => {
                let base = []
                let auth = []
                let ui = []
                res.data.map(e => {
                    e.options = e.options.split(",")
                    if (e.setType === "common") {
                        base.push(e)
                    }
                    if (e.setType === "auth") {
                        auth.push(e)
                    }
                    if (e.setType === "ui") {
                        ui.push(e)
                    }
                })
                this.componentUpdate({
                    baseParams: base,
                    authParams: auth,
                    uiParams: ui
                })
            })
    }

    inputChangeFn(evt, index, type) {
        clearTimeout(inputTimer)
        inputTimer = setTimeout(() => {
            let data = {}
            switch (type) {
                case 'base':
                    data = Object.assign(data, this.baseParams[index])
                    break
                case 'auth':
                    data = Object.assign(data, this.authParams[index])
                    break
                case 'ui':
                    data = Object.assign(data, this.uiParams[index])
                    break
                default:
                    data = null
            }
            if (data === null) {
                return
            }
            data.value = evt.target.value
            data.options = data.options.join(",")
            request("/setting/update", data)
                .then(res => {
                    console.log(JSON.stringify(res))
                })
        }, 800)
    }

    changeTabFn(tab) {
        this.componentUpdate({
            tab
        })
    }

    renderTabContent() {
        let data
        switch (this.tab) {
            case "base":
                data = this.baseParams
                break
            case "auth":
                data = this.authParams
                break
            case "ui":
                data = this.uiParams
                break
        }
        return data.map((item, index) => {
            return <div class="tr">
                <div class="td">
                    {item.key}
                </div>
                <div class="td desc">
                    {item.desc}
                    {
                        item.remark ? <div class="help">
                            <span>?</span>
                            <span class="remark">{item.remark}</span>
                        </div> : ""
                    }
                </div>
                <div class="td">
                    {item.defaultValue}
                </div>
                <div class="td" >
                    <input type="text"
                           placeholder={"不填使用默认值"}
                           value={item.value}
                           onInput={evt => this.inputChangeFn(evt, index, this.tab)}/>
                   
                </div>
                <div class="td">
                    {item.options.join(",")}
                </div>
                {/*<div class="td">*/}
                {/*    {item.remark}*/}
                {/*</div>*/}
            </div>
        })
    }

    render() {
        return <div class="set-ctx">
            <div class="title">
                <span>Frpc配置</span>
            </div>
            <div class={"main-set-bar"}>
                <div class={this.tab == "base" ? "bar-item bar-item-active" : "bar-item"}
                     onClick={() => this.changeTabFn("base")}>基本配置
                </div>
                <div class={this.tab == "auth" ? "bar-item bar-item-active" : "bar-item"}
                     onClick={() => this.changeTabFn("auth")}>权限验证配置
                </div>
                <div class={this.tab == "ui" ? "bar-item bar-item-active" : "bar-item"}
                     onClick={() => this.changeTabFn("ui")}>WEB UI 配置
                </div>
            </div>
            <div style="padding:15px 15px 0 15px">
                <div class="table">
                    <div class="tr">
                        <div class="th">参数</div>
                        <div class="th">说明</div>
                        <div class="th">默认值</div>
                        <div class="th">设定值</div>
                        <div class="th">可选值</div>
                        {/*<div class="th">备注</div>*/}
                    </div>
                </div>
            </div>
            <div class="body">
                <div class="table">
                    {this.renderTabContent()}
                </div>
            </div>
        </div>
    }
}