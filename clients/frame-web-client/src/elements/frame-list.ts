import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";

@customElement("frame-list")
export class FrameList extends LitElement {
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

	@property({ type: String }) userId = "";

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

	async getFrames() {
		if (!this.userId) return;
    
		const response = await fetch(
			`${config.queryAddress}/users/${this.userId}/frames`,
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

		if (this.userId) {
			setTimeout(this.getFrames, 500);
		}
	}

	shouldUpdate(changedProperties: Map<string | number | symbol, unknown>) {
		if (changedProperties.has("userId")) {
			this.getFrames();
		}
		return super.shouldUpdate(changedProperties);
	}

	render() {
		return html`
			<fieldset>
				<legend>عینک ها</legend>

				<div>
					${this.frames.map(
						(frame) =>
							html`
								<section>
									<a
										href="/frame/view?id=${frame.id}"
										data-element-name="frame-view"
									>
										${frame.coverImage
											? html` <img src="${frame.coverImage}" /> `
											: html` <img /> `}
										<span>${frame.brandName}</span>
										<span>${frame.modelName}</span>
										<span>${frame.price}</span>
									</a>
								</section>
							`
					)}
				</div>
			</fieldset>
		`;
	}
}
