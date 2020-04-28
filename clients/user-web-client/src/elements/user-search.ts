import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";
import { initElementIfUninit } from "../utils/helpers";

@customElement("user-search")
export class UserSearch extends LitElement {
	static styles = [commonStyles, css``];

	@property({ type: String })
	term = "";

	@property({ type: Array })
	users = new Array<{
		id: string;
		name: string;
		username: string;
		email: string;
	}>();

	async searchUsers() {
		if (!this.term) {
			this.users = [];
			return;
		}

		const response = await fetch(
			`${config.queryAddress}/search?term=${this.term}`,
			{
				method: "GET",
			}
		);
		this.users = await response.json();
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

		initElementIfUninit("user-list");
	}

	shouldUpdate(changedProperties: Map<string | number | symbol, unknown>) {
		if (changedProperties.has("term")) {
			this.searchUsers();
		}
		return super.shouldUpdate(changedProperties);
	}

	render() {
		return html`<user-list .users=${this.users}></user-list>`;
	}
}
