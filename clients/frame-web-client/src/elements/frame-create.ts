import { LitElement, html, property, customElement, css } from "lit-element";
import commonStyles from "../utils/common-styles";
import { config } from "../../package.json";
import { resizeImage } from "../utils/utils";

@customElement("frame-create")
export class FrameCreate extends LitElement {
	@property({ type: Object }) model = {
		brandName: "",
		colors: new Array<string>(),
		coverImage: "",
		description: "",
		hasCase: false,
		materials: new Array<string>(),
		modelName: "",
		otherImages: new Array<string>(),
		price: 0,
		privacyMode: 1,
	};

	static styles = [commonStyles, css``];

	create(e: Event) {
		e.preventDefault();
		fetch(`${config.serviceAddress}/frames`, {
			body: JSON.stringify(this.model),
			headers: {
				Authorization: `Bearer ${localStorage.getItem("bearer")}`,
				"Content-Type": "application/json",
			},
			method: "POST",
		});
	}

	async tryReadFile(e: Event) {
		if (!(e.target instanceof HTMLInputElement)) return;

		if (!e.target.files || !e.target.files.length) {
			this.model.coverImage = "";
			return;
		}

		this.model.coverImage = await resizeImage({
			maxSize: 240,
			file: e.target.files[0],
		});
	}

	render() {
		return html`
			<article>
				<h1>افزودن عینک</h1>

				<form @submit="${this.create}">
					<fieldset>
						<legend>اطلاعات اولیه</legend>
						<div>
							<label>
								نام برند
								<input
									type="text"
									@input="${(e: Event) =>
										(this.model.brandName = (e.target as HTMLInputElement).value)}"
									name="brand-name"
									pattern="^.{2,50}$"
									required
								/>
							</label>

							<label>
								نام مدل
								<input
									type="text"
									@input="${(e: Event) =>
										(this.model.modelName = (e.target as HTMLInputElement).value)}"
									name="model-name"
									pattern="^.{2,50}$"
									required
								/>
							</label>

							<label>
								رنگ ها
								<input
									type="text"
									@input="${(e: Event) =>
										(this.model.colors = (e.target as HTMLInputElement).value.split(
											" "
										))}"
									name="colors"
								/>
							</label>

							<label>
								متریال ها
								<input
									type="text"
									@input="${(e: Event) =>
										(this.model.materials = (e.target as HTMLInputElement).value.split(
											" "
										))}"
									name="materials"
								/>
							</label>

							<label>
								تصویر اصلی
								<input type="file" @input="${this.tryReadFile}" name="image" />
							</label>

							<label>
								قیمت
								<input
									type="number"
									@input="${(e: Event) =>
										(this.model.price = parseFloat(
											(e.target as HTMLInputElement).value
										))}"
									min="0"
									name="price"
								/>
							</label>

							<div class="checkbox-wrapper">
								<label>
									<input
										type="checkbox"
										@input="${(e: Event) =>
											(this.model.hasCase = (e.target as HTMLInputElement).checked)}"
										name="materials"
									/>
									&nbsp; دارای جلد
								</label>
							</div>

							<!--<label>
								نحوه نمایش
								<select
									@input="${(e: Event) =>
								(this.model.privacyMode = parseInt(
									(e.target as HTMLSelectElement).value
								))}"
								>
									<option value="1" selected>نمایش برای همه</option>
									<option value="2">نمایش فقط برای دنبال کنندگان</option>
									<option value="3">نمایش قیمت فقط برای دنبال کنندگان</option>
								</select>
							</label>-->

							<label class="whole-row">
								توضیحات
								<textarea
									@input="${(e: Event) =>
										(this.model.description = (e.target as HTMLTextAreaElement).value)}"
									name="description"
									pattern="^.{2,50}$"
								></textarea>
							</label>
						</div>
					</fieldset>

					<button type="submit">
						<box-icon color="currentColor" name="plus-circle"></box-icon>
						افزودن
					</button>
				</form>
			</article>
		`;
	}
}
