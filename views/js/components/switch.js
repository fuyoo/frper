export default class Switch extends Element {
    props = {}
    open = false

    constructor(props) {
        super();
        this.open = props.value
        this.props = props
    }

    ['on click at .switch-container']() {

        this.componentUpdate({
            open: !this.open
        })
        this.postEvent(new CustomEvent("switch",{bubbles:true,a:123,data:{
                data:this.props.data,
                value:this.open
            }}))
    }

    render() {
        return <div styleset="/css/framework.css#switch">
            <div class={this.open === true
                ? `switch-container switch-container-open`
                : `switch-container switch-container-close`}>
                <div class={
                    this.open === true
                        ? `switch-action action-btn-open `
                        : `switch-action action-btn-close"}`}></div>
            </div>
            <span style={`color:${ !this.open ? this.props.disabledColor || "var(--n5)" : this.props.enabledColor || "var(--mb)"}`}>{this.open ? this.props.enabledText || '' : this.props.disabledText || ''}</span>
        </div>
    }
}