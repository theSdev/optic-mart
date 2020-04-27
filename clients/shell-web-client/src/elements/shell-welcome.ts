import commonStyles from "../utils/common-styles";
import { css, customElement, html, LitElement, property } from "lit-element";
import { initElementIfUninit } from "../utils/helpers";

@customElement("shell-welcome")
export class ShellWelcome extends LitElement {
	static styles = [
		commonStyles,
		css`
			p {
				text-align: center;
				font-size: 2em;
			}

			form {
				width: 300px;
				margin: auto;
				display: flex;
				flex-direction: column;
			}

			form > :first-child {
				display: flex;
				justify-content: space-between;
				align-items: center;
				margin-bottom: 1em;
			}

			form > :nth-child(2) {
				display: flex;
				margin-bottom: 1em;
			}

			[type="search"] {
				flex: 1 1;
			}
		`,
	];

	@property({ type: Object })
	searchModel = {
		type: 0,
		term: "",
	};

	async search(e: Event) {
		e.preventDefault();
		/*if (this.searchModel.type) {
		} else {
			(this.shadowRoot!.querySelector(
				"frame-search"
			)! as any).term = this.searchModel.term;
		}*/
	}

	connectedCallback() {
		super.connectedCallback();

		initElementIfUninit("user-search");
		initElementIfUninit("frame-search");
	}

	render() {
		return html`<p>به اپتیک مارت خوش آمدید!</p>
			<form @submit="${this.search}">
				<div>
					<span>جستجو برای:</span>

					<div>
						<input
							type="radio"
							name="search-type"
							id="main-search-type-frame"
							value="0"
							checked
							@input="${(e: Event) =>
								(this.searchModel = {
									...this.searchModel,
									type: parseInt((e.target as HTMLInputElement).value),
								})}"
						/>
						<label for="main-search-type-frame">عینک</label>
					</div>

					<div>
						<input
							type="radio"
							name="search-type"
							id="main-search-type-user"
							value="1"
							@input="${(e: Event) =>
								(this.searchModel = {
									...this.searchModel,
									type: parseInt((e.target as HTMLInputElement).value),
								})}"
						/>
						<label for="main-search-type-user">کاربر</label>
					</div>
				</div>

				<div>
					<input
						type="search"
						@input="${(e: Event) => {
							this.searchModel = {
								...this.searchModel,
								term: (e.target as HTMLInputElement).value,
							};
						}}"
					/>
				</div>

				<user-search
					?hidden=${!this.searchModel.type}
					.term=${this.searchModel.term}
				></user-search>
				<frame-search
					?hidden=${this.searchModel.type}
					.term=${this.searchModel.term}
				></frame-search>
			</form>`;
	}
}
