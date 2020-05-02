import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";
import { parseJwt, initElementIfUninit } from "../utils/helpers";

@customElement("frame-index")
export class FrameIndex extends LitElement {
	static styles = [commonStyles, css``];

	@property({ type: String })
	loggedInUserId: string | null = null;

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
		
		initElementIfUninit("frame-list");

		const token = localStorage.getItem("bearer");
		if (token) {
			try {
				const parsedToken = parseJwt(token);
				this.loggedInUserId = parsedToken.id;
			} catch (e) {
				console.error(e);
			}
		}
	}

	render() {
		return html`
			<article>
				<h2>عینک</h2>

				<section>
					<a href="/frame/create" data-element-name="frame-create">
						<box-icon color="currentColor" name="plus-circle"></box-icon>
						<span>افزودن</span>
					</a>
				</section>

				<frame-list userId=${this.loggedInUserId}></frame-list>
			</article>
		`;
	}
}
