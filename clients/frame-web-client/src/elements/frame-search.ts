import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";
import { initElementIfUninit } from "../utils/helpers";

@customElement("frame-search")
export class FrameSearch extends LitElement {
	static styles = [commonStyles, css``];

	@property({ type: String })
	term = "";

	@property({ type: Array })
	frames = new Array<{
		id: string;
		brandName: string;
		colors: Array<string>;
		coverImage: string;
		description: string;
		hasCase: boolean;
		materials: Array<string>;
		modelName: string;
		otherImages: Array<string>;
		price: number;
		privacyMode: number;
	}>();

	async searchFrames() {
		const response = await fetch(
			`${config.queryAddress}/search?term=${this.term}`,
			{
				method: "GET",
			}
		);
		this.frames = await response.json();
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

		initElementIfUninit("frame-list");
	}

	shouldUpdate(changedProperties: Map<string | number | symbol, unknown>) {
		if (changedProperties.has("term")) {
			this.searchFrames();
		}
		return super.shouldUpdate(changedProperties);
	}

	render() {
		return html`<frame-list .frames=${this.frames}></frame-list>`;
	}
}
