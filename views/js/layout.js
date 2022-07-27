import Switch from "./components/switch";
import {request, view} from "./utils";
import {devicePixels} from "@sciter"

import {launch} from "@env"

export default class Layout extends Element {
    constructor() {
        super();
        this.fetchList()
    }

    tableList = []

    componentDidMount() {
    }

    fetchList() {
        request("/proxy")
            .then(res => {
                if (res.code == 200) {
                    this.componentUpdate({
                        tableList: res.data
                    })
                } else {
                    view.modal(<error>{res.msg}</error>)
                }
            })
    }

    ["on switch"](evt) {
        let data = evt.data
        this.tableList[data.data].enabled = data.value
        request("/proxy/enable", this.tableList[data.data])
            .then(res => {
                if (res.code == 200) {
                    this.fetchList()
                } else {
                    view.modal(<error>{res.msg}</error>)
                }
            })
    }

    ["on click at .add"]() {
        let res = view.modal({
            width: devicePixels(800),
            height: devicePixels(430),
            url: "proxy_set.html"
        })
        if (res) {
            this.fetchList()
        }
    }

    editRecord(data) {
        let res = view.modal({
            width: devicePixels(1024),
            height: devicePixels(640),
            url: "proxy_set.html",
            parameters: data
        })
        if (res) {
            this.fetchList()
        }
    }

    deleteRecord(id) {
        let res = view.modal(<question caption="警告">删除后无法恢复，确定要删除吗？</question>)
        if (res == 'yes') request("/proxy/delete", id)
            .then(res => {
                if (res.code == 200) {
                    this.fetchList()
                } else {
                    view.model(<error>{res.msg}</error>)
                }
            })
    }

    copyFn(t) {
        let text = ""
        if (t.subdomain) {
            text = t.subdomain + '.'
        }
        text = text + t.custom_domains
        console.log("http://" + text)
        launch("http://" + text)
        return
        if (Clipboard.writeText(text)) {
            view.modal(<info caption="提示">复制成功！</info>)
        }
    }

    render() {
        return <div class="layout">
            <div class="layout-action">
                <span>代理列表</span>
                <button class="btn btn-primary add">添加</button>
            </div>
            <div class="layout-datas">
                <div class="table">
                    <div class="tr">
                        <div class="th">代理名称</div>
                        <div class="th">代理类型</div>
                        <div class="th">代理IP</div>
                        <div class="th">代理端口</div>
                        <div class="th">映射域名</div>
                        <div class="th action">状态</div>
                        <div class="th action">操作</div>
                    </div>
                    {this.tableList.length == 0 ? <div class="tr" style={"width:1*;text-align:center;"}>
                        <div class={"td"} style={"padding:20dip"}>
                            <span class={"no-data"}>暂无数据</span>
                        </div>
                    </div> : ""}
                    {this.tableList.map((e, index) => {
                        let local = JSON.parse(e.local)
                        let t = JSON.parse(e.proxy_type_value)
                        return <div class="tr">
                            <div class="td">{e.name}</div>
                            <div class="td">{e.proxy_type}</div>
                            <div class="td">{local.local_ip}</div>
                            <div class="td">{local.local_port}</div>
                            <div class="td">
                                <span class="copy" onClick={() => {
                                    this.copyFn(t)
                                }}>
                                    {t.subdomain && t.subdomain + '.'}{t.custom_domains || "无"}
                                </span>
                            </div>
                            <div class="td action">
                                <Switch data={index} value={e.enabled}></Switch>
                            </div>
                            <div class="td action">
                                <button class="btn btn-primary edit" onClick={() => this.editRecord(e)}>编辑</button>
                                <span style={`display:inline-block;padding:2dip`}></span>
                                <button class="btn btn-error" onClick={() => this.deleteRecord(e.id)}>删除</button>
                            </div>
                        </div>
                    })}
                </div>
            </div>
        </div>
    }
}