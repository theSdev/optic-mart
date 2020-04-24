import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";

@customElement("user-register")
export class UserRegister extends LitElement {
	@property({ type: Object }) model = {
		name: "",
		username: "",
		password: "",
		email: "",
	};

	static styles = [commonStyles, css``];

	register(e: Event) {
		e.preventDefault();
		fetch(`${config.serviceAddress}/users`, {
			body: JSON.stringify(this.model),
			headers: {
				"Content-Type": "application/json",
			},
			method: "POST",
		});
	}

	render() {
		return html`
			<article>
				<h1>ثبت نام</h1>

				<form @submit="${this.register}">
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
								/>
							</label>

							<label>
								گذرواژه
								<input
									@input="${(e: Event) =>
										(this.model.password = (e.target as HTMLInputElement).value)}"
									autocomplete="new-password"
									required
									type="password"
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
								/>
							</label>
						</div>
					</fieldset>

					<button type="submit">
						<box-icon color="currentColor" name="user-plus"></box-icon>
						ثبت نام
					</button>
				</form>
			</article>
		`;
	}
}
