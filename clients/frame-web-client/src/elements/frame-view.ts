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

	entityId: string | null = null;

	static styles = [
		commonStyles,
		css`
			h1 {
				display: flex;
			}

			h1 a {
				margin-right: auto;
				font-size: 1rem;
			}

			dl {
				display: grid;
				grid-template-columns: auto 1fr;
				gap: 12px;
			}

			dt::after {
				content: ":";
			}

			@media (min-width: 768px) {
				img {
					float: left;
				}
			}
		`,
	];

	async getFrame() {
		const response = await fetch(
			`${config.queryAddress}/frames/${this.entityId}`,
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
		setTimeout(() => {
			this.entityId = new URL(window.location.href).searchParams.get("id");

			this.getFrame();
		});
	}

	render() {
		return html`
			<article>
				<h2>
					عینک
					<a
						href="/order/place?frameId=${this.entityId}"
						data-element-name="order-place"
					>
						<box-icon color="currentColor" name="cart"></box-icon>
						سفارش
					</a>
				</h2>

				<section>
					${this.model.coverImage
						? html` <img src="${this.model.coverImage}" /> `
						: null}
					<dl>
						<dt>برند</dt>
						<dd>${this.model.brandName}</dd>
						<dt>مدل</dt>
						<dd>${this.model.modelName}</dd>
						<dt>رنگ ها</dt>
						<dd>${this.model.colors.join(" - ") || "-"}</dd>
						<dt>توضیحات</dt>
						<dd>${this.model.description || "-"}</dd>
						<dt>متریال ها</dt>
						<dd>${this.model.materials.join(" - ") || "-"}</dd>
						<dt>قیمت</dt>
						<dd>${this.model.price}</dd>
					</dl>
				</section>
			</article>
		`;
	}
}
