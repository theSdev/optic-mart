import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";

@customElement("order-index")
export class OrderIndex extends LitElement {
	static styles = [
		commonStyles,
		css`
			table {
				border-collapse: collapse;
				table-layout: fixed;
				text-align: center;
				width: calc(100% - 32px);
				box-sizing: border-box;
				margin: 16px;
			}

			th,
			td {
				border-bottom: 1px black solid;
				padding: 12px;
			}
		`,
	];

	@property({ type: String }) userId = "";

	@property({ type: Array })
	orders = new Array<{
		id: string;
		customerId: string;
		frameColor: string;
		frameId: string;
		quantity: number;
		total: number;
	}>();

	async getOrders() {
		const response = await fetch(`${config.queryAddress}/orders`, {
			headers: {
				Authorization: `Bearer ${localStorage.getItem("bearer")}`,
				"Content-Type": "application/json",
			},
			method: "GET",
		});
		this.orders = await response.json();
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

		this.getOrders();
	}

	render() {
		return html`
			<h2>سفارشات دریافت شده</h2>

			<table>
				<thead>
					<tr>
						<th>فریم</th>
						<th>سفارش دهنده</th>
						<th>رنگ</th>
						<th>تعداد</th>
						<th>مجموع</th>
					</tr>
				</thead>
				<tbody>
					${this.orders.map(
						order => html`
							<tr>
								<td>
									<a
										href="/frame/view?id=${order.frameId}"
										data-element-name="frame-view"
										>فریم</a
									>
								</td>
								<td>
									<a
										href="/user/view?id=${order.customerId}"
										data-element-name="user-view"
										>سفارش دهنده</a
									>
								</td>
								<td>${order.frameColor}</td>
								<td>${order.quantity}</td>
								<td>${order.total}</td>
							</tr>
						`
					)}
				</tbody>
			</table>
		`;
	}
}
