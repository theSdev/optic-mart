import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";
import { resizeImage } from "../utils/utils";

@customElement("order-place")
export class OrderPlace extends LitElement {
	@property({ type: Object }) model = {
		frameColor: "",
		frameId: "",
		quantity: 0,
	};

	@property({ type: Array }) colors: Array<string> = [];
	@property({ type: String }) frameName: string = "";
	@property({ type: Number }) framePrice: number = 0;

	static styles = [commonStyles, css``];

	create(e: Event) {
		e.preventDefault();
		fetch(`${config.serviceAddress}/orders`, {
			body: JSON.stringify(this.model),
			headers: {
				Authorization: `Bearer ${localStorage.getItem("bearer")}`,
				"Content-Type": "application/json",
			},
			method: "POST",
		});
	}

	async getFrame() {
		const response = await fetch(
			`${config.frameQueryAddress}/frames/${this.model.frameId}`,
			{
				method: "GET",
			}
		);
		const result = await response.json();
		this.frameName = `${result.brandName} - ${result.modelName}`;
		this.framePrice = result.price;
		this.colors = result.colors;
		this.requestUpdate();
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
		setTimeout(() => {
			this.model.frameId =
				new URL(window.location.href).searchParams.get("frameId") || "";

			this.getFrame();
		});
	}

	render() {
		return html`
			<article>
				<h1>سفارش ${this.frameName}</h1>

				<form @submit="${this.create}">
					<fieldset>
						<legend>اطلاعات</legend>
						<div>
							<label>
								تعداد
								<input
									type="number"
									@input="${(e: Event) =>
										(this.model.quantity = parseInt(
											(e.target as HTMLInputElement).value
										))}"
									min="1"
									max="65535"
									name="quantity"
									required
								/>
							</label>

							<label>
								رنگ
								<select
									@input="${(e: Event) =>
										(this.model.frameColor = (e.target as HTMLSelectElement).value)}"
								>
									<option value="${null}">بدون مقدار</option>
									${this.colors.map(
										color =>
											html`
												<option value="${color}">${color}</option>
											`
									)}
								</select>
							</label>

							<!--<label>
								قیمت کل
								<input
									readonly
									.value=${this.framePrice * this.model.quantity}
								/>
							</label>-->
						</div>
					</fieldset>

					<button type="submit">
						<box-icon color="currentColor" name="plus-circle"></box-icon>
						سفارش
					</button>
				</form>
			</article>
		`;
	}
}
