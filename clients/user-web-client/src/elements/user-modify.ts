import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";
import { parseJwt } from "../utils/helpers";

@customElement("user-modify")
export class UserModify extends LitElement {
	@property({ type: Object }) model = {
		id: "",
		name: "",
		username: "",
		email: "",
	};

	static styles = [commonStyles, css``];

	loggedInUserId: string | null = null;

	modify(e: Event) {
		e.preventDefault();
		fetch(`${config.serviceAddress}/users/${this.model.id}`, {
			body: JSON.stringify(this.model),
			headers: {
				Authorization: `Bearer ${localStorage.getItem("bearer")}`,
				"Content-Type": "application/json",
			},
			method: "PUT",
		});
	}

	async getUser() {
		const response = await fetch(
			`${config.queryAddress}/users/${this.model.id}`,
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

		this.shadowRoot!.addEventListener("navigate", (e) =>
			this.shadowRoot!.dispatchEvent(e)
		);

		const token = localStorage.getItem("bearer");
		if (token) {
			try {
				this.loggedInUserId = parseJwt(token).id;
			} catch (e) {
				console.error(e);
			}
		}

		setTimeout(() => {
			this.model.id = new URL(window.location.href).searchParams.get("id")!;
			this.getUser();
		});
	}

	render() {
		return html`
			<article>
				<h2>ویرایش کاربر</h2>

				<form @submit="${this.modify}">
					<fieldset>
						<legend>اطلاعات اولیه</legend>
						<div>
							<label>
								نام
								<input
									@input="${(e: Event) =>
										(this.model.name = (e.target as HTMLInputElement).value)}"
									autocomplete="name"
									pattern="^.{2,50}$"
									required
									value=${this.model.name}
								/>
							</label>

							<label>
								نام کاربری
								<input
									@input="${(e: Event) =>
										(this.model.username = (e.target as HTMLInputElement).value)}"
									autocomplete="username"
									pattern="^[a-zA-Z0-9_]{1,50}$"
									required
									value=${this.model.username}
								/>
							</label>

							<label>
								ایمیل
								<input
									@input="${(e: Event) =>
										(this.model.email = (e.target as HTMLInputElement).value)}"
									autocomplete="email"
									required
									type="email"
									value=${this.model.email}
								/>
							</label>
						</div>
					</fieldset>

					<button type="submit">
						<box-icon color="currentColor" name="save"></box-icon>
						ذخیره
					</button>
				</form>
			</article>
		`;
	}
}
