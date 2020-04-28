import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";

@customElement("user-list")
export class UserList extends LitElement {
	static styles = [
		commonStyles,
		css`
			fieldset > div {
				display: grid;
				grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
				gap: 24px;
			}

			section > a {
				display: grid;
				grid-template-columns: auto 1fr;
				grid-template-rows: 1fr 1fr 1fr;
				grid-auto-flow: column;
				border: 1px black solid;
				padding: 24px;
				column-gap: 24px;
				row-gap: 8px;
				text-decoration: none !important;
			}

			img {
				grid-row: 1 / -1;
				width: 100px;
				height: 100px;
				object-fit: contain;
			}
		`,
	];

	@property({ type: Array })
	users = new Array<{
		id: string;
		name: string;
		username: string;
		email: string;
	}>();

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
	}

	shouldUpdate(changedProperties: Map<string | number | symbol, unknown>) {
		return super.shouldUpdate(changedProperties);
	}

	render() {
		return html`
			<article>
				<h2>کاربران</h2>

				<div>
					${this.users.map(
						(user) =>
							html`
								<section>
									<a
										href="/user/view?id=${user.id}"
										data-element-name="user-view"
									>
										<dl>
											<dt>نام</dt>
											<dd>${user.name}</dd>
											<dt>نام کاربری</dt>
											<dd>${user.username}</dd>
											<dt>ایمیل</dt>
											<dd>${user.email}</dd>
										</dl>
									</a>
								</section>
							`
					)}
				</div>
			</article>
		`;
	}
}
