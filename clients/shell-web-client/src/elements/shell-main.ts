import commonStyles from "../utils/common-styles";
import { css, customElement, html, LitElement } from "lit-element";

@customElement("shell-main")
export class ShellMain extends LitElement {
	static styles = [
		commonStyles,
		css`
			:host {
				display: block;
				width: 100%;
				height: 100%;
			}

			footer {
				border-bottom-left-radius: 0;
				border-bottom-right-radius: 0;
				bottom: 0;
			}

			footer nav {
				display: flex;
				align-items: center;
				justify-content: space-around;
				width: 100%;
			}

			footer,
			header {
				display: flex;
				align-items: center;
				padding: 1em;
				position: fixed;
				width: calc(100% - 2px);
				height: calc(80px - 2px);
				box-sizing: border-box;
				margin: 1px;
			}

			header {
				border-top-left-radius: 0;
				border-top-right-radius: 0;
				top: 0;
			}

			header nav {
				margin-right: auto;
				display: grid;
				grid-auto-flow: column;
				grid-gap: 12px;
			}

			main {
				box-sizing: border-box;
				min-height: 100vh;
				padding-block: calc(80px + 10px);
			}
		`,
	];

	tryAnchorNavigate(e: Event) {
		if (!(e.target instanceof HTMLAnchorElement)) return;

		const elementName = e.target.dataset.elementName;
		if (!elementName) return;

		e.preventDefault();
		ShellMain.navigate(elementName, e.target.href);
	}

	customElementNavigate(e: Event | CustomEvent) {
		// Work around TypeScript to recognize CustomEvent
		if (!(e instanceof CustomEvent)) return;

		ShellMain.navigate(e.detail.elementName, e.detail.href);
	}

	static navigate(elementName: string, href: string) {
		if (!customElements.get(elementName)) {
			const clientName = elementName.split("-")[0];
			const clientAddress = getClientAddress(clientName);
			const elementFileAddress = `${clientAddress}/elements/${elementName}.js`;
			const loadElementScript = document.createElement("script");
			loadElementScript.src = elementFileAddress;
			document.head.appendChild(loadElementScript);
		}

		document
			.querySelector("shell-main")!
			.shadowRoot!.querySelector(
				"main"
			)!.innerHTML = `<${elementName}></${elementName}>`;
		window.history.pushState(null, elementName, href);

		function getClientAddress(clientName: string) {
			const clientsAddresses = JSON.parse(
				document.getElementById("clients-addresses")!.innerHTML
			);
			return clientsAddresses[clientName];
		}
	}

	connectedCallback() {
		super.connectedCallback();

		this.shadowRoot!.addEventListener("click", this.tryAnchorNavigate);
		this.shadowRoot!.addEventListener("navigate", this.customElementNavigate);
	}

	firstUpdated() {
		const currentURL = new URL(window.location.href);
		(this.shadowRoot!.querySelector(
			`a[href='${
				currentURL.pathname.length > 1 ? currentURL.pathname : "/frame/index"
			}']`
		)! as HTMLAnchorElement).click();
	}

	render() {
		return html`
			<header class="primary">
				<h1>اپتیک مارت</h1>
				<nav>
					<a href="/user/login" data-element-name="user-login">
						<box-icon
							color="currentColor"
							type="solid"
							name="arrow-to-left"
						></box-icon>
						<span>ورود</span>
					</a>
					<a href="/user/register" data-element-name="user-register">
						<box-icon color="currentColor" name="user-plus"></box-icon>
						<span>ثبت نام</span>
					</a>
				</nav>
			</header>
			<main></main>
			<footer class="outline-primary">
				<nav>
					<a href="/frame/index" data-element-name="frame-index">
						<box-icon
							color="currentColor"
							name="user-plus"
							title="عینک"
						></box-icon>
						<span>عینک</span>
					</a>
					<a href="/user/register" data-element-name="user-register">
						<box-icon
							color="currentColor"
							name="user-plus"
							title="پروفایل"
						></box-icon>
						<span>پروفایل</span>
					</a>
				</nav>
			</footer>
		`;
	}
}
