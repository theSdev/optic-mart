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

			header {
				padding: 1em;
				display: flex;
				align-items: center;
				border-top-left-radius: 0;
				border-top-right-radius: 0;
			}

			header nav {
				margin-right: auto;
				display: grid;
				grid-auto-flow: column;
				grid-gap: 12px;
			}
		`,
	];

	tryNavigate(e: Event) {
		if (!(e.target instanceof HTMLAnchorElement)) return;

		const elementName = e.target.dataset.element;
		if (!elementName) return;

		if (!customElements.get(elementName)) {
			const clientName = elementName.split("-")[0];
			const clientAddress = getClientAddress(clientName);
			const elementFileAddress = `${clientAddress}/elements/${elementName}.js`;
			const loadElementScript = document.createElement("script");
			loadElementScript.src = elementFileAddress;
			document.head.appendChild(loadElementScript);
		}

		this.querySelector("main")!.innerHTML = `<${elementName}></${elementName}>`;

		e.preventDefault();

		function getClientAddress(clientName: string) {
			const clientsAddresses = JSON.parse(
				document.getElementById("clients-addresses")!.innerHTML
			);
			return clientsAddresses[clientName];
		}
	}

	connectedCallback() {
		super.connectedCallback();

		this.shadowRoot!.addEventListener("click", this.tryNavigate);
	}

	firstUpdated() {
		this.shadowRoot!.querySelector("a")!.click();
	}

	render() {
		return html`
			<header class="primary">
				<h1>اپتیک مارت</h1>
				<nav>
					<a href="/user/register" data-element="user-login">
						<box-icon
							color="currentColor"
							type="solid"
							name="arrow-to-left"
						></box-icon>
						<span>ورود</span>
					</a>
					<a href="/user/register" data-element="user-register">
						<box-icon color="currentColor" name="user-plus"></box-icon>
						<span>ثبت نام</span>
					</a>
				</nav>
			</header>
			<main></main>
		`;
	}
}
