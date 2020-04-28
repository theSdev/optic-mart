import commonStyles from "../utils/common-styles";
import { css, customElement, html, LitElement } from "lit-element";
import { parseJwt, initElementIfUninit } from "../utils/helpers";

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

			box-icon[name="list-ul"] {
				transform: rotateY(0.5turn);
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

	loggedInUsername: string | null = null;
	loggedInUserId: string | null = null;

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

	logout(e: Event) {
		e.preventDefault();
		localStorage.removeItem("bearer");
		window.location.reload();
	}

	static navigate(elementName: string, href: string, pushState = true) {
		initElementIfUninit(elementName);

		document
			.querySelector("shell-main")!
			.shadowRoot!.querySelector(
				"main"
			)!.innerHTML = `<${elementName}></${elementName}>`;
		pushState && window.history.pushState(null, elementName, href);
	}

	connectedCallback() {
		super.connectedCallback();

		this.shadowRoot!.addEventListener("click", this.tryAnchorNavigate);
		this.shadowRoot!.addEventListener("navigate", this.customElementNavigate);

		const token = localStorage.getItem("bearer");
		if (token) {
			try {
				const parsedToken = parseJwt(token);
				this.loggedInUserId = parsedToken.id;
				this.loggedInUsername = parsedToken.sub;
			} catch (e) {
				console.error(e);
			}
		}
	}

	firstUpdated() {
		this.currentUrlNavigate();
		window.addEventListener("popstate", this.currentUrlNavigate);
	}

	currentUrlNavigate() {
		const currentURL = new URL(window.location.href);
		if (currentURL.pathname.length <= 1) {
			ShellMain.navigate("shell-welcome", "/", false);
			return;
		}

		const elementName = currentURL.pathname.split("/").splice(1, 2).join("-");
		ShellMain.navigate(elementName, currentURL.href, false);
	}

	render() {
		return html`
			<header class="primary">
				<h1><a href="/" data-element-name="shell-main">اپتیک مارت</a></h1>
				<nav>
					${!this.loggedInUsername
						? html`
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
						  `
						: html`
								<a
									href="/user/view?id=${this.loggedInUserId}"
									data-element-name="user-view"
								>
									<box-icon
										color="currentColor"
										type="solid"
										name="user"
									></box-icon>
									<span>${this.loggedInUsername}</span>
								</a>

								<a href="/user/logout" @click=${this.logout}>
									<box-icon
										color="currentColor"
										type="solid"
										name="arrow-to-left"
									></box-icon>
									<span>خروج</span>
								</a>
						  `}
				</nav>
			</header>
			<main></main>
			${this.loggedInUsername
				? html`
						<footer class="outline-primary">
							<nav>
								<a href="/frame/index" data-element-name="frame-index">
									<box-icon
										color="currentColor"
										name="glasses-alt"
										title="عینک"
									></box-icon>
									<span>عینک ها</span>
								</a>
								<a href="/order/index" data-element-name="order-index">
									<box-icon
										color="currentColor"
										name="list-ul"
										title="سفارش"
									></box-icon>
									<span>سفارشات</span>
								</a>
							</nav>
						</footer>
				  `
				: null}
		`;
	}
}
