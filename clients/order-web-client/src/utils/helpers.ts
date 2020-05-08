import { config } from "../../package.json";

export function initElementIfUninit(elementName: string) {
	if (!customElements.get(elementName)) {
		const clientName = elementName.split("-")[0];
		//@ts-ignore
		const clientAddress: string = config.clientsAddresses[clientName];
		const elementFileAddress = `${clientAddress}/elements/${elementName}.js`;
		const loadElementScript = document.createElement("script");
		loadElementScript.src = elementFileAddress;
		document.head.appendChild(loadElementScript);
	}
}

export function parseJwt(token: string) {
	var base64Url = token.split(".")[1];
	var base64 = base64Url.replace(/-/g, "+").replace(/_/g, "/");
	var jsonPayload = decodeURIComponent(
		atob(base64)
			.split("")
			.map(function(c) {
				return "%" + ("00" + c.charCodeAt(0).toString(16)).slice(-2);
			})
			.join("")
	);

	return JSON.parse(jsonPayload);
}
