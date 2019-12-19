import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";

@customElement("frame-view")
export class FrameView extends LitElement {
	@property({ type: Object }) model = {
		brandName: "",
		colors: new Array<string>(),
		coverImage: null,
		description: "",
		hasCase: false,
		materials: new Array<string>(),
		modelName: "",
		otherImages: new Array<string>(),
		price: 0,
		privacyMode: 1,
	};

	entity_id = new URL(window.location.href).searchParams.get("id");

	static styles = [commonStyles, css``];

	async getFrame(e: Event) {
		e.preventDefault();
		const response = await fetch(
			`${config.serviceAddress}/frames/${this.entity_id}`,
			{
				method: "GET",
			}
		);
		this.model = await response.json();
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
