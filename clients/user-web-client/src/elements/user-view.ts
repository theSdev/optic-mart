import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";
import { initElementIfUninit, parseJwt } from "../utils/helpers";

@customElement("user-view")
export class UserView extends LitElement {
	@property({ type: Object }) model = {
		name: "",
		username: "",
		email: "",
	};

	entityId: string | null = null;

	loggedInUserId: string | null = null;

	static styles = [
		commonStyles,
		css`
			h1 {
				display: flex;
			}

			h1 button {
				margin-right: auto;
			}

			dl {
				display: grid;
				grid-template-columns: auto 1fr;
				gap: 12px;
			}

			dt::after {
				content: ":";
			}
		`,
	];

	async getUser() {
		const response = await fetch(
			`${config.queryAddress}/users/${this.entityId}`,
			{
				method: "GET",
			}
		);
		this.model = await response.json();
	}

	connectedCallback() {
		super.connectedCallback();

		//redispatch anchor click event as navigate event to pass shadow boundary;
		this.shadowRoot!.addEventListener("click", (e) => {
			if (!(e.target instanceof HTMLAnchorElement)) return;

			const elementName = e.target.dataset.elementName;
			if (!elementName) return;

			e.preventDefault();
			this.shadowRoot!.dispatchEvent(
				new CustomEvent("navigate", {
					bubbles: true,
					composed: true,
					detail: {
						elementName,
						href: e.target.href,
					},
				})
			);
		});

		this.shadowRoot!.addEventListener("navigate", (e) =>
			this.shadowRoot!.dispatchEvent(e)
		);

		const token = localStorage.getItem("bearer");
		if (token) {
			try {
				this.loggedInUserId = parseJwt(token).id;
			} catch (e) {
				console.error(e);
			}
		}

		initElementIfUninit("frame-list");

		setTimeout(() => {
			this.entityId = new URL(window.location.href).searchParams.get("id");
			this.getUser();
		});
	}

	render() {
		return html`
			<article>
				<h1>
					کاربر
					${this.loggedInUserId && this.loggedInUserId != this.entityId && false
						? html`
								<button class="secondary">
									<box-icon color="currentColor" name="user-plus"></box-icon>
									دنبال کردن
								</button>
						  `
						: null}
				</h1>

				<section>
					<dl>
						<dt>نام</dt>
						<dd>${this.model.name}</dd>
						<dt>نام کاربری</dt>
						<dd>${this.model.username}</dd>
						<dt>ایمیل</dt>
						<dd>${this.model.email}</dd>
					</dl>
				</section>

				<frame-list .userId="${this.entityId}"></frame-list>
			</article>
		`;
	}
}
