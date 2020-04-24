import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";

@customElement("user-login")
export class UserLogin extends LitElement {
	@property({ type: Object }) model = {
		username: "",
		password: "",
	};

	static styles = [commonStyles, css``];

	async login(e: Event) {
		e.preventDefault();
		const response = await fetch(
			`${config.serviceAddress}/users/${this.model.username}/tokens`,
			{
				body: this.model.password,
				headers: {
					"Content-Type": "text/plain",
				},
				method: "POST",
			}
		);
		const token = await response.text();
		localStorage.setItem("bearer", token);
		window.location.href = "/";
	}

	render() {
		return html`
			<article>
				<h1>ورود</h1>

				<form @submit="${this.login}">
					<fieldset>
						<legend>اطلاعات حساب</legend>
						<div>
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
									autocomplete="current-password"
									required
									type="password"
								/>
							</label>
						</div>
					</fieldset>

					<button type="submit">
						<box-icon
							color="currentColor"
							type="solid"
							name="arrow-to-left"
						></box-icon>
						ورود
					</button>
				</form>
			</article>
		`;
	}
}
