import { css } from "lit-element";

export default css`
	:host:not([hidden]) {
		display: block;
	}

	* {
		border-radius: var(--border-radius);
		font-family: inherit;
	}

	.break-line {
		grid-column-start: 1;
	}

	.checkbox-wrapper {
		display: flex;
	}

	.checkbox-wrapper label {
		align-self: center;
		display: flex;
		align-items: center;
	}

	.primary,
	[type="submit"] {
		background-color: var(--first-color);
		color: var(--fourth-color);
	}

	.whole-row {
		grid-column: 1 / -1;
	}

	a:link,
	a:visited {
		color: currentColor;
		text-decoration: underline solid;
	}

	a * {
		pointer-events: none;
	}

	a,
	button:not([hidden]) {
		align-items: center;
		display: inline-flex;
		justify-content: center;
	}

	a box-icon,
	button box-icon {
		margin-left: 0.3em;
	}

	article {
		display: grid;
		grid-gap: 1.5em;
		padding: 1em;
	}

	button {
		padding: 0.5em 1em;
		border: none;
	}

	button:hover {
		filter: contrast(1.1);
	}

	button:active {
		transform: translateX(-1px);
	}

	fieldset {
		border: 1px currentColor solid;
		padding: 1em;
		margin-bottom: 1.5em;
	}

	fieldset > div {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		grid-gap: 20px;
	}

	fieldset > div > label {
		display: grid;
		grid-gap: 0.5em;
	}

	h2 {
		margin: 0;
	}

	input[type="file"],
	input[type="number"],
	input[type="text"],
	select {
		height: 5ex;
	}

	input[type="file"],
	input[type="number"],
	input[type="text"],
	select,
	textarea {
		border: 1px currentColor solid;
		box-sizing: border-box;
		height: 5ex;
		padding: 1ex;
		width: 100%;
	}

	textarea {
		resize: block;
		min-height: 8ex;
	}
`;
