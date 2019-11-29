import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";

@customElement("frame-index")
export class FrameIndex extends LitElement {
	@property({ type: Object }) model = {
		username: "",
		password: "",
	};

	static styles = [commonStyles, css``];

	login(e: Event) {
		e.preventDefault();
		fetch(`${config.serviceAddress}/users/${this.model.username}/tokens`, {
			body: this.model.password,
			headers: {
				"Content-Type": "text/plain",
			},
			method: "POST",
		});
	}

	connectedCallback() {
		super.connectedCallback();

		//redispatch anchor click event as navigate event to pass shadow boundary;
		this.shadowRoot!.addEventListener("click", e => {
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

	render() {
		return html`
			<article>
				<h1>عینک</h1>

				<section>
					<a href="/frame/create" data-element-name="frame-create">
						<box-icon color="currentColor" name="user-plus"></box-icon>
						<span>افزودن</span>
					</a>
				</section>
			</article>
		`;
	}
}
