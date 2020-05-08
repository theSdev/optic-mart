import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";
import { parseJwt } from "../utils/helpers";

enum Flag {
	Processed = "processed",
	Rejected = "rejected",
}

enum Type {
	New = "new",
	Processed = "processed",
	Rejected = "rejected",
}

enum Type2 {
	Received = "received",
	Placed = "placed",
}

@customElement("order-index")
export class OrderIndex extends LitElement {
	static styles = [
		commonStyles,
		css`
			article > div {
				overflow-x: auto;
			}

			table {
				border-collapse: collapse;
				table-layout: fixed;
				text-align: center;
				width: calc(100% - 32px);
				box-sizing: border-box;
				margin: 16px;
				min-width: 400px;
			}

			th,
			td {
				border-bottom: 1px black solid;
				padding: 12px;
			}
		`,
	];

	@property({ type: String }) userId = "";

	@property({ type: String }) type = Type.New;
	@property({ type: String }) type2 = Type2.Received;

	@property({ type: Array })
	orders = new Array<{
		id: string;
		customerId: string;
		ownerId: string;
		frameColor: string;
		frameId: string;
		quantity: number;
		total: number;
		processed: boolean;
		rejected: boolean;
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

	async flag(entityId: string, flag: Flag) {
		const response = await fetch(
			`${config.serviceAddress}/orders/${entityId}`,
			{
				headers: {
					Authorization: `Bearer ${localStorage.getItem("bearer")}`,
					"Content-Type": "application/json",
				},
				method: "PUT",
				body: JSON.stringify({
					processed: flag == Flag.Processed,
					rejected: flag == Flag.Rejected,
				}),
			}
		);
		this.orders = await response.json();
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

		const token = localStorage.getItem("bearer");
		if (token) {
			try {
				this.userId = parseJwt(token).id;
			} catch (e) {
				console.error(e);
			}
		}

		this.getOrders();
	}

	get receivedTable() {
		return html`
			<table>
				<thead>
					<tr>
						<th>فریم</th>
						<th>سفارش دهنده</th>
						<th>رنگ</th>
						<th>تعداد</th>
						<th>مجموع</th>
						<th>عملیات</th>
					</tr>
				</thead>
				<tbody>
					${this.orders
						.filter((o) => o.ownerId == this.userId)
						.filter((o) =>
							this.type == Type.Rejected
								? o.rejected
								: this.type == Type.Processed
								? o.processed
								: !o.processed && !o.rejected
						)
						.map(
							(order) => html`
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
									<td>
										<button
											type="button"
											@click=${() => this.flag(order.id, Flag.Processed)}
											?hidden=${this.type == Type.Processed}
										>
											<box-icon
												color="limegreen"
												name="check"
												aria-label="تعیین به عنوان پردازش شده"
												title="تعیین به عنوان پردازش شده"
											></box-icon>
										</button>

										<button
											type="button"
											@click=${() => this.flag(order.id, Flag.Rejected)}
											?hidden=${this.type == Type.Rejected}
										>
											<box-icon
												color="tomato"
												name="x"
												aria-label="رد"
												title="رد"
											></box-icon>
										</button>
									</td>
								</tr>
							`
						)}
				</tbody>
			</table>
		`;
	}

	get placedTable() {
		return html`
			<table>
				<thead>
					<tr>
						<th>فریم</th>
						<th>سفارش گیرنده</th>
						<th>رنگ</th>
						<th>تعداد</th>
						<th>مجموع</th>
					</tr>
				</thead>
				<tbody>
					${this.orders
						.filter((o) => o.customerId == this.userId)
						.filter((o) =>
							this.type == Type.Rejected
								? o.rejected
								: this.type == Type.Processed
								? o.processed
								: !o.processed && !o.rejected
						)
						.map(
							(order) => html`
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
											>سفارش گیرنده</a
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

	render() {
		return html`
			<article>
				<h2>سفارشات</h2>

				<form>
					<fieldset>
						<legend>فیلتر</legend>

						<div>
							<label>
								نوع
								<select
									@input="${(e: Event) =>
										(this.type2 = <Type2>(
											(e.target as HTMLSelectElement).value
										))}"
								>
									<option value="${Type2.Received}">دریافت شده</option>
									<option value="${Type2.Placed}">ثبت شده</option>
								</select>
							</label>

							<label>
								نوع
								<select
									@input="${(e: Event) =>
										(this.type = <Type>(e.target as HTMLSelectElement).value)}"
								>
									<option value="${Type.New}">جدید</option>
									<option value="${Type.Processed}">پردازش شده</option>
									<option value="${Type.Rejected}">رد شده</option>
								</select>
							</label>
						</div>
					</fieldset>
				</form>

				<div>
					${this.type2 == Type2.Received
						? html`${this.receivedTable}`
						: html`${this.placedTable}`}
				</div>
			</article>
		`;
	}
}
