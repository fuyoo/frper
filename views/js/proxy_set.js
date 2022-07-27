import {request, view} from "./utils";

view.isResizable = true
let timer = -1
class ProxySet extends Element {
    mode = "new"
    baseSet = [
        {
            key: "name",
            desc: "代理名称",
            required: "是",
            default: "",
            options: "",
            remark: "代理名称为必填项",
            type: "base"
        },
        {
            key: "type",
            desc: "代理类型",
            required: "是",
            default: "",
            options: "tcp, udp, http, https, stcp, sudp, xtcp, tcpmux",
            remark: "",
            type: "base"
        },
        {
            key: "use_encryption",
            desc: "是否启用加密功能",
            required: "否",
            default: "false",
            options: "true,false",
            remark: "启用后该代理和服务端之间的通信内容都会被加密传输",
            type: "base"
        },
        {
            key: "use_compression",
            desc: "是否启用压缩功能",
            required: "否",
            default: "false",
            options: "true,false",
            remark: "启用后该代理和服务端之间的通信内容都会被压缩传输",
            type: "base"
        },
        {
            key: "proxy_protocol_version",
            desc: "启用 proxy protocol 协议的版本",
            required: "否",
            default: "",
            options: "v1, v2",
            remark: "如果启用，则 frpc 和本地服务建立连接后会发送 proxy protocol 的协议，包含了原请求的 IP 地址和端口等内容",
            type: "base"
        },
        {
            key: "bandwidth_limit",
            desc: "设置单个 proxy 的带宽",
            required: "否",
            default: "",
            options: "",
            remark: "单位为 MB 或 KB，0 表示不限制，如果启用，会作用于对应的 frpc",
            type: "base"
        },
    ]
    local = [
        {
            key: "local_ip",
            desc: "本地服务 IP",
            required: "是",
            default: "",
            options: "",
            remark: "需要被代理的本地服务的 IP 地址，可以为客户端能访问到的任意 IP 地址",
            type: "local"
        },
        {
            key: "local_port",
            desc: "本地服务端口",
            required: "是",
            default: "",
            options: "",
            remark: "配合 local_ip",
            type: "local"
        },
        {
            key: "plugin",
            desc: "客户端插件名称",
            required: "否",
            default: "",
            options: "",
            remark: "用于扩展 frpc 的能力，能够提供一些简单的本地服务，如果配置了 plugin，则 local_ip 和 local_port 无效，两者只能配置一个",
            type: "local"
        },
        {
            key: "plugin_params",
            desc: "客户端插件参数",
            required: "否",
            default: "",
            options: "",
            remark: "map 结构，key 需要都以 “plugin_” 开头，每一个 plugin 需要的参数也不一样，具体见客户端插件参数中的内容",
            type: "local"
        },
    ]
    slbAndHealthCheck = [
        {
            key: "group",
            desc: "负载均衡分组名称",
            required: "否",
            default: "",
            options: "",
            remark: "用户请求会以轮询的方式发送给同一个 group 中的代理",
            type: "slb"
        },
        {
            key: "group_key",
            desc: "负载均衡分组密钥",
            required: "否",
            default: "",
            options: "",
            remark: "用于对负载均衡分组进行鉴权，group_key 相同的代理才会被加入到同一个分组中",
            type: "slb"
        },
        {
            key: "health_check_type",
            desc: "健康检查类型",
            required: "否",
            default: "",
            options: "tcp,http",
            remark: "配置后启用健康检查功能，tcp 是连接成功则认为服务健康，http 要求接口返回 2xx 的状态码则认为服务健康",
            type: "slb"
        },
        {
            key: "health_check_timeout_s",
            desc: "健康检查超时时间(秒)",
            required: "否",
            default: "3",
            options: "",
            remark: "执行检查任务的超时时间",
            type: "slb"
        },
        {
            key: "health_check_max_failed",
            desc: "健康检查连续错误次数",
            required: "否",
            default: "1",
            options: "",
            remark: "连续检查错误多少次认为服务不健康",
            type: "slb"
        },
        {
            key: "health_check_interval_s",
            desc: "健康检查周期(秒)",
            required: "否",
            default: "10",
            options: "",
            remark: "每隔多长时间进行一次健康检查",
            type: "slb"
        },
        {
            key: "health_check_url",
            desc: "健康检查的 HTTP 接口",
            required: "否",
            default: "",
            options: "",
            remark: "如果 health_check_type 类型是 http，则需要配置此参数，指定发送 http 请求的 url，例如 “/health”",
            type: "slb"
        },
    ]
    tcp = [
        {
            key: "remote_port",
            desc: "服务端绑定的端口",
            required: "是",
            default: "",
            options: "",
            remark: "用户访问此端口的请求会被转发到 local_ip:local_port",
            type: "tcp"
        }
    ]
    udp = [
        {
            key: "remote_port",
            desc: "服务端绑定的端口",
            required: "是",
            default: "",
            options: "",
            remark: "用户访问此端口的请求会被转发到 local_ip:local_port",
            type: "udp"
        },
    ]
    http = [
        {
            key: "custom_domains",
            desc: "服务器绑定自定义域名",
            required: "是",
            default: "",
            options: "",
            remark: "请求代理服务器的域名等于这个配置域名，则会被转发到配置的本地服务",
            type: "http"
        },
        {
            key: "subdomain",
            desc: "自定义子域名",
            required: "否",
            default: "",
            options: "",
            remark: "和 custom_domains 作用相同，但是只需要指定子域名前缀，会结合服务端的 subdomain_host 生成最终绑定的域名",
            type: "http"
        },
        {
            key: "locations",
            desc: "URL 路由配置 ",
            required: "否",
            default: "",
            options: "",
            remark: "用户请求匹配响应的 location 配置，则会被路由到此代理",
            type: "http"
        },
        {
            key: "route_by_http_user",
            desc: "根据 HTTP Basic Auth user 路由",
            required: "否",
            default: "",
            options: "",
            remark: "",
            type: "http"
        },
        {
            key: "http_user",
            desc: "用户名",
            required: "否",
            default: "",
            options: "",
            remark: "Http请求Basic Auth的用户名",
            type: "http"
        },
        {
            key: "http_pwd",
            desc: "密码",
            required: "否",
            default: "",
            options: "",
            remark: "结合 http_user 使用",
            type: "http"
        },
        {
            key: "host_header_rewrite",
            desc: "替换 Host header",
            required: "否",
            default: "",
            options: "",
            remark: "替换发送到本地服务 HTTP 请求中的 Host 字段",
            type: "http"
        },
        {
            key: "headers",
            desc: "替换 header",
            required: "否",
            default: "",
            options: "",
            remark: "map中的key是要替换的header的key，value是替换后的内容",
            type: "http"
        },
    ]
    https = [
        {
            key: "custom_domains",
            desc: "服务器绑定自定义域名",
            required: "是",
            default: "",
            options: "",
            remark: "用户通过 vhost_http_port 访问的 HTTP 请求如果 Host 在 custom_domains 配置的域名中，则会被路由到此代理配置的本地服务",
            type: "https"
        },
        {
            key: "subdomain",
            desc: "自定义子域名",
            required: "否",
            default: "",
            options: "",
            remark: "和 custom_domains 作用相同，但是只需要指定子域名前缀，会结合服务端的 subdomain_host 生成最终绑定的域名",
            type: "https"
        },
    ]
    stcp = [
        {
            key: "role",
            desc: "角色",
            required: "是",
            default: "",
            options: "server,visitor",
            remark: "server 表示服务端，visitor 表示访问端",
            type: "stcp"
        },
        {
            key: "sk",
            desc: "密钥",
            required: "是",
            default: "",
            options: "",
            remark: "服务端和访问端的密钥需要一致，访问端才能访问到服务端",
            type: "stcp"
        },
    ]
    sudp = [
        {
            key: "role",
            desc: "角色",
            required: "是",
            default: "",
            options: "server,visitor",
            remark: "server 表示服务端，visitor 表示访问端",
            type: "sudp"
        },
        {
            key: "sk",
            desc: "密钥",
            required: "",
            default: "",
            options: "",
            remark: "服务端和访问端的密钥需要一致，访问端才能访问到服务端",
            type: "sudp"
        },
    ]
    xtcp = [
        {
            key: "role",
            desc: "角色",
            required: "是",
            default: "",
            options: "server,visitor",
            remark: "server 表示服务端，visitor 表示访问端",
            type: "xtcp"
        },
        {
            key: "sk",
            desc: "密钥",
            required: "是",
            default: "",
            options: "",
            remark: "服务端和访问端的密钥需要一致，访问端才能访问到服务端",
            type: "xtcp"
        },
    ]
    tcpmux = [
        {
            key: "multiplexer",
            desc: "复用器类型",
            required: "是",
            default: "",
            options: "httpconnect",
            remark: "",
            type: "tcpmux"
        },
        {
            key: "custom_domains",
            desc: "服务器绑定自定义域名",
            required: "是",
            default: "",
            options: "",
            remark: "用户通过 tcpmux_httpconnect_port 访问的 CONNECT 请求如果 Host 在 custom_domains 配置的域名中，则会被路由到此代理配置的本地服务",
            type: "tcpmux"
        },
        {
            key: "subdomain",
            desc: "自定义子域名",
            required: "否",
            default: "",
            options: "",
            remark: "和 custom_domains 作用相同，但是只需要指定子域名前缀，会结合服务端的 subdomain_host 生成最终绑定的域名",
            type: "tcpmux"
        },
        {
            key: "route_by_http_user",
            desc: "根据 HTTP Basic Auth user 路由",
            required: "否",
            default: "",
            options: "",
            remark: "",
            type: "tcpmux"
        },
    ]
    tab = 'base'
    types = ["tcp", "udp", "http", "https", "stcp", "sudp", "xtcp", "tcpmux"]
    config = {
        base: {},
        local: {},
        slb: {},
        tcp: {},
        udp: {},
        http: {},
        https: {},
        stcp: {},
        sudp: {},
        xtcp: {},
        tcpmux: {}
    }

    constructor() {
        super();
        const params = view.parameters
        if (params) {
            this.mode = "edit"

            Object.keys(view.parameters)
                .map(key => {
                    switch (key) {
                        case "name":
                            this.config.name = params[key]
                            break
                        case "proxy_type":
                            this.config.type = params[key]
                            break
                        case "proxy_type_value":
                            this.config[params["proxy_type"]] = JSON.parse(params["proxy_type_value"])
                            break
                        case "id":
                            this.config.id = params[key]
                            break
                        default:
                            this.config[key] = JSON.parse(params[key])
                    }
                })
        }

    }

    renderType() {
        if (this.types.indexOf(this.config.type) === -1) {
            return ""
        }
        return (<div onClick={() => this.tabChangeFn(this.config.type)} class={
            this.tab === this.config.type
                ? "proxy-set-tab-item proxy-set-tab-active"
                : 'proxy-set-tab-item'}>
            
            <span class="is-required">
                *
            </span> {this.config.type}
        </div>)

    }

    getDataByType(type) {
        let data = []
        switch (type) {
            case 'base':
                data = this.baseSet
                break
            case 'local':
                data = this.local
                break
            case 'slb':
                data = this.slbAndHealthCheck
                break
            case 'tcp':
                data = this.tcp
                break
            case 'udp':
                data = this.udp
                break
            case 'http':
                data = this.http
                break
            case 'https':
                data = this.https
                break
            case 'stcp':
                data = this.stcp
                break
            case 'sudp':
                data = this.sudp
                break
            case 'xtcp':
                data = this.xtcp
                break
            case 'tcpmux':
                data = this.tcpmux
                break
        }
        return data
    }

    renderCtx() {
        return this.getDataByType(this.tab).map((item, index) => {
            return <div class="tr">
                <div class="td">
                    <span class="is-required">
                        {item.required === "是" && "*"}
                    </span>
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
                    {item.default}
                </div>
                <div class="td">
                      <span class="is-required">
                        {item.required === "是" && "*"}
                    </span>
                    {
                        item.key === 'type' ||  item.key === 'name'
                            ? <input type="text"
                                     placeholder={item.required ==='是'? '此项为必填项': "不填使用默认值"}
                                     value={this.config[item.key] || ""}
                                     onInput={evt => this.inputChangeFn(evt, index, item.type)}/>
                            : <input type="text"
                                     placeholder={item.required ==='是'? '此项为必填项': "不填使用默认值"}
                                     value={this.config[item.type][item.key] || ""}
                                     onInput={evt => this.inputChangeFn(evt, index, item.type)}/>
                    }

                </div>
                <div class="td">
                    {item.options}
                </div>
                {/*<div class="td">*/}
                {/*    {item.remark}*/}
                {/*</div>*/}
            </div>
        })
    }

    inputChangeFn(evt, index, type) {
        clearTimeout(timer)
        timer = setTimeout(() => {
            let data = this.getDataByType(type)
            let val = data[index]
            if (val.key === "name") {
                this.config[val.key] = evt.target.value
            }
            if (val.key === "type") {
                this.config[val.key] = evt.target.value
            } else {
                this.config[type][val.key] = evt.target.value
            }
            this.componentUpdate()
        }, 100)
    }

    tabChangeFn(tab) {
        this.componentUpdate({
            tab
        })
    }

    ["on click at #save"]() {
        if (!this.config.name) {
            return view.modal(<error>基础配置中name字段为必填项</error>)
        }
        if (this.types.indexOf(this.config.type) == -1) {
            return view.modal(<error>基础配置中type字段输入错误</error>)
        }
        if (!this.config["local"].local_ip) {
            return view.modal(<error>本地服务配置中local_ip字段不能为空</error>)
        }
        if (!this.config["local"].local_port) {
            return view.modal(<error>本地服务配置中local_port字段不能为空</error>)
        }
        this[this.config.type].map(e => {
            if (e.required == "是" && !this.config[this.config.type][e.key]) {
                return view.modal(<error>{this.config.type}配置中{e.key}字段不能为空</error>)
            }
        })
        let data = {
            name: this.config.name,
            base: JSON.stringify(this.config.base),
            local: JSON.stringify(this.config.local),
            slb: JSON.stringify(this.config.slb),
            proxy_type: this.config.type,
            proxy_type_value: JSON.stringify(this.config[this.config.type]),
            enabled: true
        }
        let req
        if (this.mode == "new") {
            req = request("/proxy/add", data, true)

        } else {
            data.id = this.config.id
            req = request("/proxy/edit", data, true)
        }
        req.then(res => {
            if (res.code == 200) {
                view.modal(<info caption="提示">{res.msg}</info>)
                view.close(true)
            } else {
                view.modal(<error>{res.msg}</error>)
            }
        })

    }

    render() {
        return <div class={"proxy-set"}>
            <div class={"proxy-set-title"}><span>代理配置 <small>(*为必填项)</small></span>
                <button class="btn btn-primary" id="save">保存</button>
            </div>
            <div class={"proxy-set-tab"}>
                <div onClick={() => this.tabChangeFn('base')} class={this.tab == 'base'
                    ? "proxy-set-tab-item proxy-set-tab-active"
                    : 'proxy-set-tab-item'}>
                    <span class="is-required">
                        *
                    </span>
                    基础配置
                </div>
                <div onClick={() => this.tabChangeFn('local')} class={this.tab == 'local'
                    ? "proxy-set-tab-item proxy-set-tab-active"
                    : 'proxy-set-tab-item'}>
                    <span class="is-required">
                         *
                    </span>
                    本地服务配置
                </div>
                <div onClick={() => this.tabChangeFn('slb')} class={this.tab == 'slb'
                    ? "proxy-set-tab-item proxy-set-tab-active"
                    : 'proxy-set-tab-item'}>负载均衡和健康检查
                </div>
                {this.renderType()}
            </div>
            {<div class={"proxy-set-tab-content set-ctx"}>
                <div class="table">
                    <div class="tr">
                        <div class="th">参数</div>
                        <div class="th">说明</div>
                        <div class="th">默认值</div>
                        <div class="th">设定值</div>
                        <div class="th">可选值</div>
                        {/*<div class="th">备注</div>*/}
                    </div>
                    {this.renderCtx()}
                </div>
            </div>}
        </div>
    }
}

document.body.patch(<ProxySet/>)